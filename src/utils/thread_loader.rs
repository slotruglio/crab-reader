use std::{
    ops::Deref,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
};

use threadpool::ThreadPool;

pub(crate) struct ThreadLoader<T> {
    pool: ThreadPool,
    sender: Sender<ThreadResult<T>>,
    receiver: Receiver<ThreadResult<T>>,
}

pub struct ThreadResult<T> {
    result: Mutex<T>,
    idx: usize,
}

impl<T> Deref for ThreadLoader<T> {
    type Target = ThreadPool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

impl<T> Default for ThreadLoader<T> {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            pool: ThreadPool::new(8),
            sender,
            receiver,
        }
    }
}

impl<T> ThreadLoader<T> {
    pub fn tx(&self) -> Sender<ThreadResult<T>> {
        self.sender.clone()
    }

    pub fn try_recv(&self) -> Option<ThreadResult<T>> {
        if let Ok(result) = self.receiver.try_recv() {
            Some(result)
        } else {
            None
        }
    }
}

impl<T: Clone> ThreadResult<T> {
    pub fn new(result: T, idx: usize) -> Self {
        Self {
            result: Mutex::from(result),
            idx,
        }
    }

    pub fn value(self) -> T {
        self.result.into_inner().unwrap()
    }

    pub fn idx(&self) -> usize {
        self.idx
    }
}

unsafe impl<T> Send for ThreadResult<T> {}
