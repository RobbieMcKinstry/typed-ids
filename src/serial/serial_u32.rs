use crate::id::Id;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::sync::atomic::AtomicPtr;

#[derive(Debug)]
pub struct SerialU32<T: ?Sized>(NonZeroU32, PhantomData<AtomicPtr<Box<T>>>);

impl<T: ?Sized> SerialU32<T> {
    /// Collect the inner value, returnng the u32.
    #[allow(dead_code)]
    pub fn get(self) -> u32 {
        self.0.get()
    }
}

impl<T: ?Sized> From<NonZeroU32> for SerialU32<T> {
    fn from(val: NonZeroU32) -> Self {
        SerialU32(val, PhantomData)
    }
}

impl<T: ?Sized> TryFrom<u32> for SerialU32<T> {
    type Error = &'static str;
    fn try_from(val: u32) -> Result<Self, Self::Error> {
        let err = "Serial must be greater than 0";
        NonZeroU32::new(val).ok_or(err).map(SerialU32::from)
    }
}

impl<T: ?Sized> Clone for SerialU32<T> {
    fn clone(&self) -> Self {
        SerialU32::from(self.0)
    }
}

impl<T: ?Sized> Copy for SerialU32<T> {}

impl<T: ?Sized> Hash for SerialU32<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: ?Sized> PartialEq for SerialU32<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: ?Sized> Ord for SerialU32<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: ?Sized> PartialOrd for SerialU32<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: ?Sized> Eq for SerialU32<T> {}

impl<T: ?Sized + 'static> Id<T> for SerialU32<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;

    #[test]
    fn threadsafe_identifiers() {
        assert_impl_all!(SerialU32<()>: Send, Sync);
    }
}
