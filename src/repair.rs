//! [QcRepair] for data repairment

/// The [QcRepair] Trait allows local and in place data repairment
/// (micro patching).
pub trait QcRepair {
    /// Repair Zero values that are considered physical non-sense
    /// or forbidden values (only!)
    fn zero_repair_mut(&mut self);

    fn zero_repair(&self) -> Self
    where
        Self: Clone + Sized,
    {
        let mut s = self.clone();
        s.zero_repair_mut();
        s
    }
}
