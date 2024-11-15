#![warn(clippy::all, clippy::pedantic)]

use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
};

use colored::Colorize;
use futures::{
    channel::mpsc,
    future::{self, BoxFuture, LocalBoxFuture},
    stream::{self, LocalBoxStream},
    FutureExt, SinkExt, StreamExt,
};
use reactive::driver::Driver;

type PostBuildReturn =
    BoxFuture<'static, Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>>;

#[derive(derive_more::Debug)]
#[must_use]
pub struct Parent {
    #[debug(skip)]
    builder_command: OsString,
    #[debug(skip)]
    builder_args: Vec<OsString>,
    output_dir: camino::Utf8PathBuf,
    builder: BuilderState,
    #[debug("{}", match post_build { None => "None", Some(_) => "Some(function)" })]
    post_build: Option<Box<dyn Fn() -> PostBuildReturn>>,
}

const LOCALHOST: &str = "localhost";

/// Error type returned from the reactive app
#[derive(Debug, thiserror::Error)]
pub enum DevError {
    #[error(transparent)]
    Notify(#[from] reactive::driver::notify::Error),
    #[error(transparent)]
    Io(std::io::Error),
    #[error("no free port")]
    NoFreePort,
}

fn local_url(port: portpicker::Port) -> reqwest::Url {
    reqwest::Url::parse(&format!("http://{LOCALHOST}:{port}")).expect("valid")
}

#[derive(Debug, Default)]
enum BuilderState {
    // this exists to ensure that only one child is live at a time
    AwaitingKillResult,
    #[default]
    AwaitingChild,
    AwaitingObsoleteChild,
    Started(tokio::process::Child),
}

impl BuilderState {
    fn killing_unchecked(&mut self) -> OutputEvent {
        let Self::Started(child) = std::mem::replace(self, Self::AwaitingKillResult) else {
            panic!("builder state must be started")
        };

        OutputEvent::KillChildProcess(child)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("builder: {_0}")]
    Io(#[from] std::io::Error),
    #[error("builder terminated with exit code {_0}")]
    ExitCode(i32),
    #[error("builder terminated without exit code")]
    NoExitCode,
    #[error("post_build: {0}")]
    PostBuild(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl Parent {
    pub fn new(
        output_dir: impl Into<camino::Utf8PathBuf>,
        builder_command: impl AsRef<OsStr> + 'static,
        builder_args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> Self {
        Self {
            output_dir: output_dir.into(),
            builder_command: builder_command.as_ref().to_owned(),
            builder_args: builder_args
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>(),
            builder: BuilderState::default(),
            post_build: None,
        }
    }

    pub fn post_build(mut self, f: impl Fn() -> PostBuildReturn + 'static) -> Self {
        self.post_build = Some(Box::new(f));
        self
    }

    /// Build once
    ///
    /// # Errors
    ///
    /// See the error type.
    pub async fn build(&self) -> Result<(), BuildError> {
        let mut command = self.builder_command();
        let status = command.status().await?;
        if !status.success() {
            return Err(status
                .code()
                .map_or(BuildError::NoExitCode, BuildError::ExitCode));
        }
        if let Some(post_build) = &self.post_build {
            post_build().await?;
        }
        Ok(())
    }

    /// Sets up a development environment that watches the file system,
    /// recompiling the crate that when run describes the website on localhost when there are changes.
    pub async fn dev(
        self,
        paths_to_watch: impl IntoIterator<Item = impl Into<PathBuf>>,
        launch_browser: bool,
    ) -> DevError {
        let Some(port) = portpicker::pick_unused_port() else {
            return DevError::NoFreePort;
        };

        let server_task =
            live_server::listen(LOCALHOST, port, self.output_dir.as_std_path().to_owned())
                .map(|result| result.expect_err("unreachable"))
                .boxed();

        let cargo_run_builder = self.builder_command();

        let url = local_url(port);

        let (builder_driver, builder_started) =
            reactive::driver::command::StaticCommandDriver::new(cargo_run_builder);
        let (child_process_killer_driver, child_killed) =
            reactive::driver::child_process_killer::ChildProcessKillerDriver::new();
        let (open_browser_driver, browser_opened) =
            reactive::driver::open_that::StaticOpenThatDriver::new(url.to_string());
        let (eprintln_driver, ()) = reactive::driver::println::EprintlnDriver::new();
        let (notify_driver, notify) = match reactive::driver::notify::FsChangeDriver::new(
            paths_to_watch.into_iter().map(Into::into).collect(),
        ) {
            Ok(val) => val,
            Err(e) => return e.into(),
        };

        let inputs = Inputs {
            server_task,
            child_killed,
            notify,
            builder_started,
            launch_browser,
            browser_opened,
            url,
        };

        let outputs = self.outputs(inputs);

        let Outputs {
            stderr,
            open_browser,
            error,
            kill_child,
            run_builder,
            stream_splitter_task,
        } = outputs;

        let builder_driver_task = builder_driver.init(run_builder);
        let child_process_killer_driver_task = child_process_killer_driver.init(kill_child);
        let open_browser_driver_task = open_browser_driver.init(open_browser);
        let stderr_driver_task = eprintln_driver.init(stderr);
        let notify_driver_task = notify_driver.init(());

        futures::select! {
            error = error.fuse() => error,
            () = builder_driver_task.fuse() => unreachable!(),
            () = child_process_killer_driver_task.fuse() => unreachable!(),
            () = stderr_driver_task.fuse() => unreachable!(),
            () = open_browser_driver_task.fuse() => unreachable!(),
            () = stream_splitter_task.fuse() => unreachable!(),
            () = notify_driver_task.fuse() => unreachable!(),
        }
    }

    fn builder_command(&self) -> tokio::process::Command {
        let mut command = tokio::process::Command::new(&self.builder_command);
        command.args(&self.builder_args);
        command
    }

    fn input_event(&mut self, input: InputEvent) -> Option<OutputEvent> {
        match input {
            InputEvent::BuilderKilled(result) => self.builder_killed(result),
            InputEvent::FsChange(result) => self.fs_change(result),
            InputEvent::BuilderStarted(child) => self.builder_started(child),
            InputEvent::BrowserOpened(result) => Self::browser_opened(result),
            InputEvent::ServerError(error) => Some(OutputEvent::Error(DevError::Io(error))),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn builder_killed(&mut self, result: std::io::Result<()>) -> Option<OutputEvent> {
        match result {
            Ok(()) => match self.builder {
                BuilderState::AwaitingKillResult => {
                    self.builder = BuilderState::AwaitingChild;
                    Some(OutputEvent::RunBuilder)
                }
                _ => unreachable!(
                    "when we issue a kill we always set the state to expect the kill result"
                ),
            },
            Err(error) => Some(OutputEvent::Error(DevError::Io(error))),
        }
    }

    fn fs_change(
        &mut self,
        result: Result<reactive::driver::notify::Event, reactive::driver::notify::Error>,
    ) -> Option<OutputEvent> {
        match result {
            Ok(event) => match event.kind {
                reactive::driver::notify::EventKind::Create(_)
                | reactive::driver::notify::EventKind::Modify(_)
                | reactive::driver::notify::EventKind::Remove(_) => match &mut self.builder {
                    BuilderState::AwaitingChild => {
                        self.builder = BuilderState::AwaitingObsoleteChild;
                        None
                    }
                    BuilderState::Started(_) => Some(self.builder.killing_unchecked()),
                    BuilderState::AwaitingKillResult | BuilderState::AwaitingObsoleteChild => None,
                },
                _ => None,
            },
            Err(error) => Some(OutputEvent::Error(DevError::Notify(error))),
        }
    }

    fn builder_started(
        &mut self,
        child: std::io::Result<tokio::process::Child>,
    ) -> Option<OutputEvent> {
        match child {
            Ok(child) => match self.builder {
                BuilderState::AwaitingKillResult => {
                    unreachable!("we don't start a builder before the previous one has been killed")
                }
                BuilderState::AwaitingChild => {
                    self.builder = BuilderState::Started(child);
                    None
                }
                BuilderState::AwaitingObsoleteChild => {
                    self.builder = BuilderState::AwaitingKillResult;
                    Some(OutputEvent::KillChildProcess(child))
                }
                BuilderState::Started(_) => {
                    let current_child = self.builder.killing_unchecked();
                    self.builder = BuilderState::Started(child);
                    Some(current_child)
                }
            },
            Err(error) => Some(OutputEvent::Error(DevError::Io(error))),
        }
    }

    fn browser_opened(result: Result<(), std::io::Error>) -> Option<OutputEvent> {
        match result {
            Ok(()) => None,
            Err(error) => Some(OutputEvent::Error(DevError::Io(error))),
        }
    }
}

#[derive(Debug)]
enum InputEvent {
    BuilderKilled(Result<(), std::io::Error>),
    FsChange(reactive::driver::notify::Result<reactive::driver::notify::Event>),
    BuilderStarted(std::io::Result<tokio::process::Child>),
    BrowserOpened(std::io::Result<()>),
    ServerError(std::io::Error),
}

#[derive(Debug)]
enum OutputEvent {
    Stderr(String),
    RunBuilder,
    KillChildProcess(tokio::process::Child),
    Error(DevError),
    OpenBrowser,
}

/// Inputs for bootstrapping the reactive app
struct Inputs {
    server_task: LocalBoxFuture<'static, std::io::Error>,
    child_killed: LocalBoxStream<'static, std::io::Result<()>>,
    notify:
        LocalBoxStream<'static, reactive::driver::notify::Result<reactive::driver::notify::Event>>,
    builder_started: LocalBoxStream<'static, std::io::Result<tokio::process::Child>>,
    launch_browser: bool,
    browser_opened: LocalBoxStream<'static, std::io::Result<()>>,
    url: reqwest::Url,
}

/// Outputs from the reactive app
struct Outputs {
    stderr: LocalBoxStream<'static, String>,
    kill_child: LocalBoxStream<'static, tokio::process::Child>,
    run_builder: LocalBoxStream<'static, ()>,
    open_browser: LocalBoxStream<'static, ()>,
    error: LocalBoxFuture<'static, DevError>,
    stream_splitter_task: LocalBoxFuture<'static, ()>,
}

impl Parent {
    fn outputs(self, inputs: Inputs) -> Outputs {
        let Inputs {
            server_task,
            child_killed,
            notify: builder_crate_fs_change,
            builder_started,
            launch_browser,
            browser_opened: browser_launch,
            url: local_host_port_url,
        } = inputs;

        let message = format!("\nServer started at {local_host_port_url}\n")
            .blue()
            .to_string();

        let mut initial = vec![OutputEvent::RunBuilder, OutputEvent::Stderr(message)];
        if launch_browser {
            initial.push(OutputEvent::OpenBrowser);
        }
        let initial = stream::iter(initial);

        let reaction = stream::select_all([
            stream::once(server_task)
                .map(InputEvent::ServerError)
                .boxed_local(),
            child_killed.map(InputEvent::BuilderKilled).boxed_local(),
            builder_crate_fs_change
                .map(InputEvent::FsChange)
                .boxed_local(),
            builder_started
                .map(InputEvent::BuilderStarted)
                .boxed_local(),
            browser_launch.map(InputEvent::BrowserOpened).boxed_local(),
        ])
        .scan(self, move |parent, input| {
            future::ready(Some(parent.input_event(input)))
        })
        .filter_map(future::ready);

        let mut output = initial.chain(reaction);

        let (mut kill_child_sender, kill_child) = mpsc::channel(1);
        let (mut run_builder_sender, run_builder) = mpsc::channel(1);
        let (mut error_sender, error) = mpsc::channel(1);
        let (mut stderr_sender, stderr) = mpsc::channel(1);
        let (mut open_browser_sender, open_browser) = mpsc::channel(1);

        let stream_splitter_task = async move {
            loop {
                let event = output.next().await.unwrap();
                match event {
                    OutputEvent::Stderr(output) => {
                        stderr_sender.send(output).await.unwrap();
                    }
                    OutputEvent::RunBuilder => {
                        run_builder_sender.send(()).await.unwrap();
                    }
                    OutputEvent::KillChildProcess(child) => {
                        kill_child_sender.send(child).await.unwrap();
                    }
                    OutputEvent::Error(error) => {
                        error_sender.send(error).await.unwrap();
                    }
                    OutputEvent::OpenBrowser => {
                        open_browser_sender.send(()).await.unwrap();
                    }
                }
            }
        }
        .boxed_local();

        let error = error
            .into_future()
            .map(|(error, _tail_of_stream)| error.unwrap())
            .boxed_local();

        Outputs {
            stderr: stderr.boxed_local(),
            kill_child: kill_child.boxed_local(),
            run_builder: run_builder.boxed_local(),
            open_browser: open_browser.boxed_local(),
            error,
            stream_splitter_task,
        }
    }
}

#[cfg(test)]
mod test {
    use std::ffi::OsString;

    use crate::{BuilderState, Parent};

    #[test]
    fn parent_debug() {
        let parent_no_post_build = Parent {
            output_dir: camino::Utf8PathBuf::from("path/to/there"),
            builder_command: OsString::new(),
            builder_args: vec![],
            builder: BuilderState::default(),
            post_build: None,
        };

        let actual = format!("{parent_no_post_build:?}");
        let expected =
            "Parent { output_dir: \"path/to/there\", builder: AwaitingChild, post_build: None, .. }";
        assert_eq!(actual, expected);
    }
}
