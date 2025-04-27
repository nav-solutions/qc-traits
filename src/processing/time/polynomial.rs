use hifitime::{Epoch, Polynomial, TimeScale};

/// [TimePolynomial] allows precise [Epoch] translation to another [TimeScale].
/// For example, |[TimeScale::GPST]-[TimeScale::UTC]| when referencing [TimeScale::GPST] to [TimeScale::UTC].
#[derive(Copy, Clone, PartialEq)]
pub struct TimePolynomial {
    /// LHS [TimeScale] to which the [Polynomial] applies
    pub lhs_timescale: TimeScale,
    /// RHS [TimeScale] to which the [Polynomial] applies
    pub rhs_timescale: TimeScale,
    /// Reference [Epoch] usually expressed in LHS [TimeScale], but we support any [TimeScale] here.
    pub ref_epoch: Epoch,
    /// [Polynomial]
    pub polynomial: Polynomial,
}

impl TimePolynomial {
    /// Define new [TimePolynomial] from Reference [Epoch] expressed as week counter
    /// and elapsed seconds within week.
    pub fn from_reference_time_of_week_seconds(
        ref_week: u32,
        ref_tow: u64,
        lhs_timescale: TimeScale,
        rhs_timescale: TimeScale,
        polynomial: Polynomial,
    ) -> Self {
        Self::from_reference_time_of_week_nanos(
            ref_week,
            ref_tow * 1_000_000_000,
            lhs_timescale,
            rhs_timescale,
            polynomial,
        )
    }

    /// Define a new [TimePolynomials] from Reference [Epoch] expressed as week counter and
    /// elapsed nanoseconds within week.
    pub fn from_reference_time_of_week_nanos(
        ref_week: u32,
        ref_tow_nanos: u64,
        lhs_timescale: TimeScale,
        rhs_timescale: TimeScale,
        polynomial: Polynomial,
    ) -> Self {
        let ref_epoch = Epoch::from_time_of_week(ref_week, ref_tow_nanos, lhs_timescale);

        Self {
            ref_epoch,
            lhs_timescale,
            rhs_timescale,
            polynomial,
        }
    }
}
