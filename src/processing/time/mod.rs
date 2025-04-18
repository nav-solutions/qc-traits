mod polynomial;
pub use polynomial::TimePolynomial;

mod absolute;
pub use absolute::GnssAbsoluteTime;

use hifitime::TimeScale;

/// The [Timeshift] trait is implemented by any structure that can
/// be converted to another [TimeScale]
pub trait Timeshift {
    /// Temporal transposition to desired [TimeScale], without mutable access
    fn timeshift(&self, solver: &GnssAbsoluteTime, target: TimeScale) -> Self
    where
        Self: Sized;

    /// [TimeScale] transposition with mutable access
    fn timeshift_mut(&mut self, solver: &GnssAbsoluteTime, target: TimeScale);
}
