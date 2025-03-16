mod decim;
mod mask;

use crate::{QcScope, QcSubset};
use hifitime::Duration;

pub use crate::filter::{decim::QcDecimationFilter, mask::QcMaskOperand};

/// [QcFilterType] defines all supported filters
#[derive(Clone, Copy)]
pub enum QcFilterType {
    /// [DecimationFilter] to reduce sample rate
    Decimation(QcDecimationFilter),
    /// [MaskOperand] to retain or filter out data
    Mask(QcMaskOperand),
}

impl QcFilterType {
    /// Builds a new [QcFilterType] (integral) % decimation
    pub fn from_decimation_modulo(modulo: u32) -> Self {
        Self::Decimation(QcDecimationFilter::from_modulo(modulo))
    }

    /// Builds a new [QcFilterType] downsampling filter
    pub fn from_downsampling_interval(dt: Duration) -> Self {
        Self::Decimation(QcDecimationFilter::from_duration(dt))
    }

    /// Builds a new [QcFilterType::Mask] filter with desired [MaskOperand]
    pub fn from_mask(mask: QcMaskOperand) -> Self {
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
