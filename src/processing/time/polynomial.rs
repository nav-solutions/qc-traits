use hifitime::{Epoch, Polynomial, TimeScale};

/// [TimePolynomial] allows precise [Epoch] translation to another [TimeScale].
/// For example, |[TimeScale::GPST]-[TimeScale::UTC]| when referencing [TimeScale::GPST] to [TimeScale::UTC].
#[derive(Copy, Clone, PartialEq)]
pub struct TimePolynomial {
    /// lhs [TimeScale]
    pub lhs_timescale: TimeScale,
    /// Reference [Epoch] expressed in the [TimeScale] in which the polynomials apply,
    /// with 1 ns accuracy.
    pub ref_epoch: Epoch,
    /// [Polynomial]
    pub polynomial: Polynomial,
}

impl TimePolynomial {
    /// Define new [TimePolynomial] from Reference [Epoch] expressed as week counter
    /// and elapsed seconds within week.
    pub fn from_reference_time_of_week_seconds(
        lhs_timescale: TimeScale,
        ref_week: u32,
        ref_tow: u64,
        ref_timescale: TimeScale,
        polynomial: Polynomial,
    ) -> Self {
        Self::from_reference_time_of_week_nanos(
            lhs_timescale,
            ref_week,
            ref_tow * 1_000_000_000,
            ref_timescale,
            polynomial,
        )
    }

    /// Define a new [TimePolynomials] from Reference [Epoch] expressed as week counter and
    /// elapsed nanoseconds within week.
    pub fn from_reference_time_of_week_nanos(
        lhs_timescale: TimeScale,
        ref_week: u32,
        ref_tow: u64,
        ref_timescale: TimeScale,
        polynomial: Polynomial,
    ) -> Self {
        let ref_epoch = Epoch::from_time_of_week(ref_week, ref_tow, ref_timescale);

        Self {
            ref_epoch,
            lhs_timescale,
            polynomial,
        }
    }
}
