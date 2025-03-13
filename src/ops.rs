use crate::{QcScope, QcSubset};

/// Generic [QcOps]
pub struct QcGenericOps<O> {
    /// [QcScope] of this operation
    pub scope: QcScope,
    /// [QcSubset] targeted by this operation
    pub subset: QcSubset,
    /// Generic operation
    pub op: O,
}
