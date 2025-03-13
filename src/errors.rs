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
    #[error("item guessing error: {0}")]
    TypeGuessingError(String),
    #[error("two valid epochs are required to describe a duration")]
    InvalidDuration,
    #[error("invalid epoch description")]
    InvalidEpoch,
    #[error("invalid SNR description")]
    InvalidSNR,
    #[error("invalid elevation angle (0 <= e <= 90)")]
    InvalidElevationAngle,
    #[error("invalid azimuth angle description (0 <= a <= 360)")]
    InvalidAzimuthAngle,
    #[error("invalid float number")]
    FloatParsing(#[from] ParseFloatError),
    #[error("sv item parsing")]
    SVParsing(#[from] SVParsingError),
    #[error("constellation item parsing")]
    ConstellationParing(#[from] ConstellationParsingError),
    #[error("duration item parsing")]
    InvalidDurationItem(#[from] EpochParsingError),
}
