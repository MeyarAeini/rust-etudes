use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

// Flavors
// - Synchronous channels : channel where Send can block(). Limited capacity.
//   Mutex + Condvar + VecDeque
//   Atomic VecDeque + thread::park + thread::Thread::notify
// - Asynchronous channels : channel where Send cann't block(). Unbounded.
//   Mutex + Condvar + VecDeque
//   Mutex + Condvar + LinkedList
//   Atomic linked list , linked list of T
//   Atomic block linked list, linked list of VecDeque<T>
// - Rendezvous channels : Syncronous channel with capacity = 0. use for thread synchronizaton.
// - Oneshot channels: any capacity. In practice, only one call to Send().
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

//here we could have use #[derive(Clone)] for Sender, however this way
//the compiler could implement clone even for the T or the inner, which is not our intention
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);
        Self {
            //here we could have write self.clone(), but it could clone the inner or the T as well
            //that is why we must implement the Clone manually and call the Arc clone directly
            //Arc::clone disables auto deref that happens in self.inner.clone()
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last_one = inner.senders == 0;
        drop(inner);
        if was_last_one {
            self.shared.available.notify_one();
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let q = &mut self.shared.inner.lock().unwrap().queue;
        q.push_back(t);
        let _ = q; //drop(q);
        self.shared.available.notify_one();
    }
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }
        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    if !inner.queue.is_empty() {
                        //swap the remaning items to the buffer to reduce the number of lock per receive
                        std::mem::swap(&mut self.buffer, &mut inner.queue);
                    }
                    return Some(t);
                }
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared {
        inner: Mutex::new({
            Inner {
                queue: VecDeque::default(),
                senders: 1,
            }
        }),
        available: Condvar::new(),
    });

    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
            buffer: VecDeque::default(),
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);

        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn tx_closed() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn rx_closed() {
        let (mut tx, rx) = channel();
        drop(rx);
        tx.send(42);
    }
}
