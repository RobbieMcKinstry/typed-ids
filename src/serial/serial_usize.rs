use crate::id::Id;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::num::NonZeroUsize;
use std::sync::atomic::AtomicPtr;

#[derive(Debug)]
pub struct SerialUsize<T: ?Sized>(NonZeroUsize, PhantomData<AtomicPtr<Box<T>>>);

impl<T: ?Sized> SerialUsize<T> {
    /// Collect the inner value, returnng the u64.
    #[allow(dead_code)]
    pub fn get(self) -> usize {
        self.0.get()
    }
}

impl<T: ?Sized> From<NonZeroUsize> for SerialUsize<T> {
    fn from(val: NonZeroUsize) -> Self {
        SerialUsize(val, PhantomData)
    }
}

impl<T: ?Sized> TryFrom<usize> for SerialUsize<T> {
    type Error = &'static str;
    fn try_from(val: usize) -> Result<Self, Self::Error> {
        let err = "Serial must be greater than 0";
        NonZeroUsize::new(val).ok_or(err).map(SerialUsize::from)
    }
}

impl<T: ?Sized> Clone for SerialUsize<T> {
    fn clone(&self) -> Self {
        SerialUsize::from(self.0)
    }
}

impl<T: ?Sized> Copy for SerialUsize<T> {}

impl<T: ?Sized> Hash for SerialUsize<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: ?Sized> PartialEq for SerialUsize<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: ?Sized> Ord for SerialUsize<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: ?Sized> PartialOrd for SerialUsize<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: ?Sized> Eq for SerialUsize<T> {}

impl<T: ?Sized + 'static> Id<T> for SerialUsize<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;

    #[test]
    fn threadsafe_identifiers() {
        assert_impl_all!(SerialUsize<()>: Send, Sync);
    }
}
