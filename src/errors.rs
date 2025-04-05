//! Error definitions
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QcError {
    #[error("Invalid scope")]
    InvalidScope,
    #[error("Unknown product type")]
    UnknownProductType,
}

/// Errors related to [QcSubset] parsing
#[derive(Debug, Error)]
pub enum QcSubsetError {
    #[error("empty description")]
    EmptyItem,
    #[error("invalid subset")]
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
