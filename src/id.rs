use std::hash::Hash;

/// An `Id` (identifier) uniquely identifies something.
/// This crate's vision of an Id is that it must be small (i.e. copy),
/// thread-safe, ordered, and hashable. The ordered and hashable requirements
/// could probably be dropped, but it sends to be more useful than not.
/// In the future, this crate could add a variant for non-Ord and non-Hash Ids.
pub trait Id<T: ?Sized>: Ord + Copy + Send + Sync + Hash + 'static {}
