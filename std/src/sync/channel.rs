use std::sync::mpsc::{self, Sender, Receiver};

pub struct Channel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Channel { sender, receiver }
    }

    pub fn send(&self, value: T) -> Result<(), mpsc::SendError<T>> {
        self.sender.send(value)
    }

    pub fn recv(&self) -> Result<T, mpsc::RecvError> {
        self.receiver.recv()
    }
}