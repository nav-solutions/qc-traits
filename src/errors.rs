//! Error definitions
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QcError {
    #[error("qc-error: unknown product type")]
    UnknownProductType,
}

/// Errors related to [QcSubset] parsing
#[derive(Debug, Error)]
pub enum QcSubsetError {
    #[error("qc-subset: empty description")]
    EmptyItem,
    #[error("qc-subset: invalid")]
    InvalidSubset,
}

/// [QcFilter] parsing error;
#[derive(Debug, Error)]
pub enum QcFilterError {
    #[error("qc-filter: invalid modulo filter")]
    InvalidModuloFilter,
    #[error("qc-filter: invalid duration")]
    InvalidDuration,
    #[error("qc-filter: invalid operand")]
    InvalidOperand,
}
