mod decim;
mod mask;

use crate::{QcScope, QcSubset};
use hifitime::Duration;

pub use crate::filter::{decim::DecimationFilter, mask::MaskOperand};

/// [QcFilterType] defines all supported filters
pub enum QcFilterType {
    /// [DecimationFilter] to reduce sample rate
    Decimation(DecimationFilter),
    /// [MaskOperand] to retain or filter out data
    Mask(MaskOperand),
}

impl QcFilterType {
    /// Builds a new [QcFilterType] (integral) % decimation
    pub fn from_decimation_modulo(modulo: u32) -> Self {
        Self::Decimation(DecimationFilter::from_modulo(modulo))
    }

    /// Builds a new [QcFilterType] downsampling filter
    pub fn from_downsampling_interval(dt: Duration) -> Self {
        Self::Decimation(DecimationFilter::from_duration(dt))
    }

    /// Builds a new [QcFilterType::Mask] filter with desired [MaskOperand]
    pub fn from_mask(mask: MaskOperand) -> Self {
        Self::Mask(mask)
    }
}

/// [QcFilter] defines a filtering operation
pub struct QcFilter {
    /// [QcScope] to which this operation should apply
    pub scope: QcScope,
    /// [QcSubset] targeted by this operation
    pub subset: QcSubset,
    /// [QcFilter] operation
    pub filter: QcFilterType,
}
