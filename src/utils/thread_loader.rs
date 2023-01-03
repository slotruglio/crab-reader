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

    #[cfg(test)]
    /// Blocking version of try_recv, meant for test purposes only
    pub fn recv(&self) -> ThreadResult<T> {
        self.receiver.recv().unwrap()
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

    #[cfg(test)]
    pub fn value_and_idx(self) -> (T, usize) {
        (self.result.into_inner().unwrap(), self.idx)
    }
}

unsafe impl<T> Send for ThreadResult<T> {}

#[cfg(test)]
mod tests {
    #[test]
    fn thread_loader_test() {
        let items = [1, 2, 3];
        let tl = super::ThreadLoader::default();

        for (idx, item) in items.iter().enumerate() {
            let res = super::ThreadResult::new(item, idx);
            tl.sender.send(res).unwrap();
        }

        for _ in 0..items.len() {
            let res = tl.recv();
            let (value, idx) = res.value_and_idx();
            assert_eq!(*value, items[idx]);
        }
    }

    #[test]
    #[should_panic(expected = "Nothing was received")]
    fn thread_loader_no_value() {
        let tl = super::ThreadLoader::<i32>::default();
        let res = tl.try_recv();
        res.expect("Nothing was received").value();
        panic!("Should not get here");
    }
}
