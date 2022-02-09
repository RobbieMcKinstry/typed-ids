use crate::id::Id;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::num::NonZeroU64;
use std::sync::atomic::AtomicPtr;

#[derive(Debug)]
pub struct SerialU64<T: ?Sized>(NonZeroU64, PhantomData<AtomicPtr<Box<T>>>);

impl<T: ?Sized> SerialU64<T> {
    /// Collect the inner value, returnng the u64.
    #[allow(dead_code)]
    pub fn get(self) -> u64 {
        self.0.get()
    }
}

impl<T: ?Sized> From<NonZeroU64> for SerialU64<T> {
    fn from(val: NonZeroU64) -> Self {
        SerialU64(val, PhantomData)
    }
}

impl<T: ?Sized> TryFrom<u64> for SerialU64<T> {
    type Error = &'static str;
    fn try_from(val: u64) -> Result<Self, Self::Error> {
        let err = "Serial must be greater than 0";
        NonZeroU64::new(val).ok_or(err).map(SerialU64::from)
    }
}

impl<T: ?Sized> Clone for SerialU64<T> {
    fn clone(&self) -> Self {
        SerialU64::from(self.0)
    }
}

impl<T: ?Sized> Copy for SerialU64<T> {}

impl<T: ?Sized> Hash for SerialU64<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: ?Sized> PartialEq for SerialU64<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: ?Sized> Ord for SerialU64<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: ?Sized> PartialOrd for SerialU64<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: ?Sized> Eq for SerialU64<T> {}

impl<T: ?Sized + 'static> Id<T> for SerialU64<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;

    #[test]
    fn threadsafe_identifiers() {
        assert_impl_all!(SerialU64<()>: Send, Sync);
    }
}
