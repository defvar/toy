use futures::channel::mpsc::{self, Receiver, Sender};

pub fn stream<T>(buffer: usize) -> (Sender<T>, Receiver<T>) {
    mpsc::channel(buffer)
}
