//! # RBConcurrency
//!
//! RBConcurrency provides a reusable thread pool.
// TODO: Change the line above when the purpose of the crate is defined better. ^
mod shared_iterator;
#[cfg(test)]
mod tests;

use shared_iterator::SharedIterator;

use std::thread;
use std::thread::JoinHandle;

use std::iter::Iterator;

use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::marker::Send;
use std::sync::mpsc::IntoIter;
use std::clone::Clone;

struct Block<I, It, O>
    where It: Iterator<Item=I> {
    items: SharedIterator<It>,
    sender: Sender<O>
}


/// A WorkerPool services an iterable list of elements by a fixed thread-pool.
///
/// The `thread_count` property describes the number of threads in the thread pool.
pub struct WorkerPool<I, It, O>
    where It: Iterator<Item=I> {
    senders: Vec<Sender<Block<I, It, O>>>,
    _threads: Vec<JoinHandle<()>>
}

impl<I, It, O> WorkerPool<I, It, O>
    where It: Iterator<Item=I> {

    pub fn new<F>(thread_count: u8, map_fn: F) -> WorkerPool<I, It, O>
        where I: 'static,
              It: Send + 'static,
              O: Send + 'static,
              F: Fn(I) -> O + Clone + Send + 'static {
        WorkerPool::internal_new(thread_count, map_fn, Option::None)
    }

    pub fn new_named<F>(name: String, thread_count: u8, map_fn: F) -> WorkerPool<I, It, O>
        where I: 'static,
              It: Send + 'static,
              O: Send + 'static,
              F: Fn(I) -> O + Clone + Send + 'static {
        WorkerPool::internal_new(thread_count, map_fn, Option::Some(name))
    }

    // TODO: At some point in the future, we should have an implementation where the number of threads the user wants is not needed/
    // TODO: The method should be smart enough to detect the processor on which the program is running, and figure out a best guess of the count...
    // TODO: ...that allows for the most efficient concurrency.

    fn internal_new<F>(thread_count: u8, map_fn: F, name: Option<String>) -> WorkerPool<I, It, O>
        where I: 'static,
              It: Send + 'static,
              O: Send + 'static,
              F: Fn(I) -> O + Clone + Send + 'static {

        let mut senders = Vec::new();
        let mut _threads = Vec::new();

        let mut number_of_threads = thread_count;
        if thread_count < 1 {
            // TODO: Maybe we should panic here instead? The caller may rely on a 'panic on the wrong input' behaviour?
            number_of_threads = 1;
        }

        for i in 0..number_of_threads {
            let (s, r): (Sender<Block<I, It, O>>, Receiver<Block<I, It, O>>) = channel();

            let mut thread_builder = thread::Builder::new();

            match name.clone() {
                Some(pool_name) => {
                    thread_builder = thread_builder.name(format!("{}-{}", pool_name, i));
                },
                None => {
                    // No name
                }
            }

            let fn_clone = map_fn.clone();

            let thread = thread_builder.spawn(move || {
                loop {
                    match r.recv() {
                        Ok(block) => {
                            let sender = block.sender;
                            block.items.for_each ( |item| {
                                match sender.send(fn_clone(item)) {
                                    Ok(_) => {},
                                    Err(_) => {
                                        // Receiver has disconnected.
                                        // Entirely possible, since the caller didn't keep a reference to the results produced by `submit()` and the call to `submit()` went out of scope.
                                        // In this case, we can't do anything. No point looping forward.

                                        // TODO: Use an IntoIterator so we can break again.
//                                    break;

                                        // TODO: No way I can think of that the caller would want this to happen intentionally. Maybe there's a way to notify the user about this in debug builds?
                                    }
                                }
                            })
                        },
                        Err(_error) => {
                            // Sender hung up.
                            break
                        }
                    }
                }
            }).unwrap();

            senders.push(s);
            _threads.push(thread);
        }


        WorkerPool {
            _threads,
            senders
        }
    }
}

impl<I, It, O> WorkerPool<I, It, O>
    where It: Iterator<Item=I> {

    pub fn submit<InIt>(&self, items: InIt) -> IntoIter<O>
        where I: 'static,
              It: Send + 'static,
              InIt: IntoIterator<IntoIter=It, Item=I>,
              O: Send + 'static {
        let (s, r) = channel();
        let items = SharedIterator::wrap(items.into_iter());

        for sender in self.senders.clone() {
            let block = Block{
                items: items.clone(),
                sender: s.clone()
            };

            sender.send(block).unwrap();
        }

        return r.into_iter();
    }
}