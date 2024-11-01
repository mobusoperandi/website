use std::convert::Infallible;

use futures::{
    channel::mpsc, future::LocalBoxFuture, stream::LocalBoxStream, FutureExt, SinkExt, StreamExt,
};
use tokio::process::{Child, Command};

use super::Driver;

/// The driver is initialised with a command, the command is run when it recieves an input, the output spawns the command as a process or sends back an error.
#[derive(Debug)]
pub struct StaticCommandDriver(Command, mpsc::Sender<Result<Child, std::io::Error>>);

impl StaticCommandDriver {
    pub fn new(command: Command) -> (Self, <Self as Driver>::Output) {
        <Self as Driver>::new(command).unwrap()
    }
}

impl Driver for StaticCommandDriver {
    type Args = Command;
    type ConstructionError = Infallible;
    type Input = LocalBoxStream<'static, ()>;
    type Output = LocalBoxStream<'static, std::io::Result<Child>>;

    fn new(command: Self::Args) -> Result<(Self, Self::Output), Self::ConstructionError> {
        let (sender, receiver) = mpsc::channel(1);
        let builder_driver = Self(command, sender);
        let output = receiver.boxed_local();
        Ok((builder_driver, output))
    }

    fn init(self, start_builder: Self::Input) -> LocalBoxFuture<'static, ()> {
        let Self(mut command, mut sender) = self;
        let mut s = start_builder.map(move |_| Ok(command.spawn()));
        async move { sender.send_all(&mut s).map(Result::unwrap).await }.boxed_local()
    }
}
