use thiserror::Error;

mod correction;
pub use correction::TimeCorrection;

mod database;
pub use database::TimeCorrectionsDB;

use hifitime::TimeScale;

/// [TimeCorrectionError] returned by precise correction methods.
#[derive(Debug, Error)]
pub enum TimeCorrectionError {
    #[error("no correction available for {0}/{1}")]
    NoCorrectionAvailable(TimeScale, TimeScale),
}

/// The [Timeshift] trait allows transposition to different [TimeScale]s and precise stirring.
pub trait Timeshift {
    /// Temporal transposition to desired [TimeScale], without mutable access.
    /// The difference between this method and [Self::precise_correction] is that
    /// it cannot take the _actual_ [TimeScale] states into account.
    fn timeshift(&self, timescale: TimeScale) -> Self
    where
        Self: Sized;

    /// Temporal transposition to desired [TimeScale], without mutable access.
    /// The difference between this method and [Self::precise_correction] is that
    /// it cannot take the _actual_ [TimeScale] states into account.
    fn timeshift_mut(&mut self, timescale: TimeScale);

    /// Precise stirring to desired [TimeScale], without mutable access.
    /// The difference between this method and [Self::timeshift] is that this
    /// method takes the _actual_ [TimeScale] states into account.
    fn precise_correction(
        &self,
        db: &TimeCorrectionsDB,
        target: TimeScale,
    ) -> Result<Self, TimeCorrectionError>
    where
        Self: Sized;

    /// Precise stirring to desired [TimeScale], with mutable access.
    /// The difference between this method and [Self::timeshift_mut] is that this
    /// method takes the _actual_ [TimeScale] states into account.
    fn precise_correction_mut(
        &mut self,
        db: &TimeCorrectionsDB,
        target: TimeScale,
    ) -> Result<(), TimeCorrectionError>;
}
