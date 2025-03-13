//! File / Data set reworking
use crate::field::QcField;

/// The [QcRework] Trait allows in place and mutable dataset reworking
/// (modification of a single [QcField], at a time).
pub trait QcRework {
    /// Add specific [QcField]. If it exist, you should overwrite.
    fn add_mut(&mut self, field: &QcField);

    /// Remove specific [QcField]. If it does not exist, this will not cause an error.
    fn remove_mut(&mut self, field: &QcField);

    fn add(&self, field: &QcField) -> Self
    where
        Self: Clone + Sized,
    {
        let mut s = self.clone();
        s.add_mut(field);
        s
    }

    fn remove(&self, field: &QcField) -> Self
    where
        Self: Clone + Sized,
    {
        let mut s = self.clone();
        s.remove_mut(field);
        s
    }
}
