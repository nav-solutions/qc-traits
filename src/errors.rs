//! Error definitions
use thiserror::Error;

#[cfg(docs)]
use crate::{
    QcScope,
    pipeline::QcPipeline,
    QcAngle,
};

/// Errors while designing a [QcPipeline]
#[derive(Debug, Error)]
pub enum QcPipelineError {
    #[error("invalid pipeline scope: {0}")]
    InvalidScope(QcScopeError),
    #[error("invalid pipeline selection: {0}")]
    InvalidSelect(QcSelectError),
}

/// Errors while parsing a [QcAngle]
#[derive(Debug, Error)]
pub enum QcAngleParsingError {
    #[error("invalid unit")]
    InvalidUnit,
    #[error("invalid angle value")]
    InvalidValue,
}

/// Errors while parsing a [QcScope]
#[derive(Debug, Error)]
pub enum QcScopeError {
    #[error("Invalid scope")]
    InvalidScope,
    #[error("Unknown product type")]
    UnknownProductType,
}


/// Errors while describing a [QcSelect]ion method
#[derive(Debug, Error)]
pub enum QcSelectError {
}