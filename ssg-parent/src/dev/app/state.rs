impl super::App {
    pub(super) fn input_event(&mut self, input: super::InputEvent) -> Option<super::OutputEvent> {
        match input {
            super::InputEvent::BuilderKilled(result) => self.builder_killed(result),
            super::InputEvent::FsChange(result) => self.fs_change(result),
            super::InputEvent::BuilderStarted(child) => self.builder_started(child),
            super::InputEvent::BrowserOpened(result) => Self::browser_opened(result),
            super::InputEvent::ServerError(error) => {
                Some(super::OutputEvent::Error(super::super::DevError::Io(error)))
            }
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn builder_killed(&mut self, result: std::io::Result<()>) -> Option<super::OutputEvent> {
        match result {
            Ok(()) => match self.builder {
                BuilderState::AwaitingKillResult => {
                    self.builder = BuilderState::AwaitingChild;
                    Some(super::OutputEvent::RunBuilder)
                }
                _ => unreachable!(
                    "when we issue a kill we always set the state to expect the kill result"
                ),
            },
            Err(error) => Some(super::OutputEvent::Error(super::super::DevError::Io(error))),
        }
    }

    fn fs_change(
        &mut self,
        result: Result<reactive::driver::notify::Event, reactive::driver::notify::Error>,
    ) -> Option<super::OutputEvent> {
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
            Err(error) => Some(super::OutputEvent::Error(super::super::DevError::Notify(
                error,
            ))),
        }
    }

    fn builder_started(
        &mut self,
        child: std::io::Result<tokio::process::Child>,
    ) -> Option<super::OutputEvent> {
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
                    Some(super::OutputEvent::KillChildProcess(child))
                }
                BuilderState::Started(_) => {
                    let current_child = self.builder.killing_unchecked();
                    self.builder = BuilderState::Started(child);
                    Some(current_child)
                }
            },
            Err(error) => Some(super::OutputEvent::Error(super::super::DevError::Io(error))),
        }
    }

    fn browser_opened(result: Result<(), std::io::Error>) -> Option<super::OutputEvent> {
        match result {
            Ok(()) => None,
            Err(error) => Some(super::OutputEvent::Error(super::super::DevError::Io(error))),
        }
    }
}

#[derive(Debug, Default)]
pub(super) enum BuilderState {
    // this exists to ensure that only one child is live at a time
    AwaitingKillResult,
    #[default]
    AwaitingChild,
    AwaitingObsoleteChild,
    Started(tokio::process::Child),
}

impl BuilderState {
    fn killing_unchecked(&mut self) -> super::OutputEvent {
        let Self::Started(child) = std::mem::replace(self, Self::AwaitingKillResult) else {
            panic!("builder state must be started")
        };

        super::OutputEvent::KillChildProcess(child)
    }
}
