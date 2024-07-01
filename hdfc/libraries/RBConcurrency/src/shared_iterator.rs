use std::sync::Arc;
use std::sync::Mutex;
// Copied over from Pipeline (https://github.com/NfNitLoop/pipeliner). Thank you so much Cody Casterline (@NfNitLoop on GitHub)
/// An iterator which can be safely shared between threads.
pub struct SharedIterator<I: Iterator> {
    iterator: Arc<Mutex<I>>
}

impl<I: Iterator> SharedIterator<I> {
    pub fn wrap(iterator: I) -> Self {
        // Since we're going to be sharing among multiple threads, each thread will need to
        // get a None of its own to end. We need to make sure our iterator doesn't cycle:

        // TODO: Use a Fuse instead. Not assuming a fuse would break assumptions a caller might have.
//        let iterator = iterator.fuse();

        SharedIterator{iterator: Arc::new(Mutex::new(iterator))}
    }
}

impl<I: Iterator> Clone for SharedIterator<I> {
    fn clone(&self) -> Self {
        SharedIterator{iterator: self.iterator.clone()}
    }
}

impl<I: Iterator> Iterator for SharedIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iterator = self.iterator.lock().expect("No poisoning in SharedIterator");
        iterator.next()
    }
}