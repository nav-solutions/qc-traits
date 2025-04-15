use hifitime::{Epoch, TimeScale, Unit};

#[derive(Debug)]
pub enum TimeOffsetError {
    /// Polynomial terms can only apply for a week (at most)
    /// and require much more recent updates in high precision applications.
    OutdatedWeeklyPolynomials,
}

/// The [TimeShift] trait is implemented by any structure that can
/// be converted to another [TimeScale]
pub trait Timeshift {
    /// [TimeScale] transposition without mutable access
    fn time_shift(&self, time_offsets: &[TimeOffset]) -> Self
    where
        Self: Sized;

    /// [TimeScale] transposition with mutable access
    fn time_shift_mut(&mut self, time_offsets: &[TimeOffset]);
}

/// [TimeOffset] allows precise [Epoch] translation from original to another [TimeScale].
/// For example, |[TimeScale::GPST]-[TimeScale::UTC]| when referencing or monitoring
/// [TimeScale::GPST] against [TimeScale::UTC].
#[derive(Copy, Clone, PartialEq)]
pub struct TimeOffset {
    /// Reference [Epoch] expressed in the [TimeScale] for which the provided polynomials apply.
    pub ref_epoch: Epoch,
    /// Polynomials (s, s.s⁻¹, s.s⁻²)
    polynomials: (f64, f64, f64),
    /// Stored [TimeScale] for (a-b) validity checks
    secondary_ts: TimeScale,
}

impl TimeOffset {
    /// Define a new [TimeOffset].
    /// ## Input
    /// - ref_epoch: Reference [Epoch] expressed in the [TimeScale] to which the provided polynomials apply.
    /// For example, [TimeScale::GST] polynomials as in |TimeScale::GST - TimeScale::GPST| in GST/GPST referencing.
    /// - secondary_ts: stored [Timescale] to verify operation validity. This would be [TimeScale::GPST] in the previous example.
    /// - polynomials: for interpolation calculations
    pub fn from_reference_epoch(
        ref_epoch: Epoch,
        secondary_ts: TimeScale,
        polynomials: (f64, f64, f64),
    ) -> Self {
        Self {
            ref_epoch,
            polynomials,
            secondary_ts,
        }
    }

    /// Define a new [TimeOffset] from week counter and elapsed nanoseconds within week.
    /// ## Input
    /// - ref_epoch: Reference [Epoch] expressed in the [TimeScale] to which the provided polynomials apply.
    /// For example, [TimeScale::GST] polynomials as in |TimeScale::GST - TimeScale::GPST| in GST/GPST referencing.
    /// - secondary_ts: stored [Timescale] to verify operation validity. This would be [TimeScale::GPST] in the previous example.
    /// - polynomials: for interpolation calculations
    pub fn from_reference_time_of_week_nanos(
        ref_week: u32,
        ref_tow: u64,
        ref_timescale: TimeScale,
        secondary_ts: TimeScale,
        polynomials: (f64, f64, f64),
    ) -> Self {
        let ref_epoch = Epoch::from_time_of_week(ref_week, ref_tow, ref_timescale);

        Self {
            ref_epoch,
            polynomials,
            secondary_ts,
        }
    }

    /// Returns both [TimeScale]s this [TimeOffset] supports. Meaning that
    /// it allows conversion to either one.
    pub fn supported_timescales(&self) -> (TimeScale, TimeScale) {
        (self.ref_epoch.time_scale, self.secondary_ts)
    }

    /// Define a new [TimeOffset] with new desired reference [Epoch].
    /// The [TimeScale] it is expressed in must remain the same, or you will to update the polynomial terms
    /// accordingly, to continue obtaining correct results.
    pub fn with_reference_epoch(mut self, ref_epoch: Epoch) -> Self {
        self.ref_epoch = ref_epoch;
        self
    }

    /// Define a new [TimeOffset] with new desired reference [Epoch] still expressed for the previously defined [TimeScale].
    /// defined as elapsed weeks and nanoseconds within current week. As typically used when working with GNSS receivers.
    /// This most likely should be tied to a polynoliam terms update: [Self::with_polynomials].
    pub fn with_reference_time_of_week_nanos(mut self, ref_epoch_tow_nanos: (u32, u64)) -> Self {
        let (ref_week, ref_tow) = ref_epoch_tow_nanos;
        self.ref_epoch = Epoch::from_time_of_week(ref_week, ref_tow, self.ref_epoch.time_scale);
        self
    }

    /// Define a new [TimeOffset] with new polynomials, while preserving other components.
    pub fn with_polynomials(mut self, polynomials: (f64, f64, f64)) -> Self {
        self.polynomials = polynomials;
        self
    }

    /// Returns the total number of nanoseconds to apply to convert this [Epoch] to other [TimeScale].
    /// ## Input
    /// - t: interpolation instant expressed as [Epoch] with 1ns accuracy.
    /// The correction is calculated using the previously latched polynomial parameters.
    pub fn time_correction_nanos(&self, t: Epoch) -> Result<f64, TimeOffsetError> {
        // express correctly in the reference Timescale
        let t = t.to_time_scale(self.ref_epoch.time_scale);

        let (t_week, t_nanos) = t.to_time_of_week();
        let (ref_week, ref_nanos) = self.ref_epoch.to_time_of_week();

        if t_week != ref_week {
            return Err(TimeOffsetError::OutdatedWeeklyPolynomials);
        }

        let (a0, a1, a2) = self.polynomials;
        let dt_s = (t_nanos as f64 - ref_nanos as f64) * 1.0E-9;
        let dt_s = a0 + a1 * dt_s + a2 * dt_s.powi(2);

        Ok(dt_s * 1.0E9)
    }

    /// Returns the total number of nanoseconds to apply to convert this [Epoch] to other [TimeScale].
    /// ## Input
    /// - t: interpolation instant expressed as [Epoch] with 1ns accuracy.
    /// It needs to be either of [Self::supported_timescales] for this operation to be valid.
    /// The correction is calculated for the other supported [TimeScale].
    pub fn time_correction_seconds(&self, t: Epoch) -> Result<f64, TimeOffsetError> {
        let correction_nanos = self.time_correction_nanos(t)?;
        Ok(correction_nanos * 1.0E-9)
    }

    /// Convert provided [Epoch] expressed in either of [Self::supported_timescales],
    /// to other supported [TimeScale]. This operation has a 1 ns accuracy.
    /// ## Input
    /// - t: interpolation instant expressed as [Epoch] with 1ns accuracy.
    /// It needs to be either of [Self::supported_timescales] for this operation to be valid.
    /// The correction is calculated for the other supported [TimeScale].
    pub fn epoch_time_correction(&self, t: Epoch) -> Result<Epoch, TimeOffsetError> {
        let correction_nanos = self.time_correction_nanos(t)?;
        let corrected = t + correction_nanos * Unit::Nanosecond;
        // perform the swap & return
        Ok(corrected.to_time_scale(self.secondary_ts))
    }
}

#[cfg(test)]
mod test {
    use crate::TimeOffset;
    use hifitime::{Epoch, TimeScale, Unit};

    #[test]
    fn test_1ns_time_offset() {
        // Tests the TimeOffset API with values slightly above hifitime precision.
        let polynomials = (1E-9, 0.0, 0.0);

        let known_timescales = [
            TimeScale::UTC,
            TimeScale::TAI,
            TimeScale::GPST,
            TimeScale::GST,
            TimeScale::BDT,
            TimeScale::QZSST,
        ];

        for ref_ts in known_timescales.iter() {
            for lhs_ts in known_timescales.iter() {
                // random t_ref in LHS timescale
                let t_ref = Epoch::from_gregorian(2020, 1, 1, 0, 0, 0, 0, *lhs_ts);

                // create valid TimeOffset
                let time_offset = TimeOffset::from_reference_epoch(t_ref, *ref_ts, polynomials);

                if ref_ts != lhs_ts {
                    ///////////////////////////////////////
                    // 1. some time LATER within that week
                    ///////////////////////////////////////
                    let instant = t_ref + 1.0 * Unit::Day;

                    // this is a simple case of a static offset
                    let dt_s = time_offset.time_correction_seconds(instant).unwrap();
                    assert_eq!(dt_s, polynomials.0);

                    let dt_nanos = time_offset.time_correction_nanos(instant).unwrap();
                    assert_eq!(dt_nanos, polynomials.0 * 1E9);

                    // Test that conversion did work
                    let converted = time_offset.epoch_time_correction(instant).unwrap();

                    assert_eq!(
                        converted.time_scale, *ref_ts,
                        "epoch_time_correction did not translate timescale!"
                    );

                    // this is a simple case of a static offset
                    let dt = (converted - instant).to_seconds();
                    assert_eq!(dt, polynomials.0);

                    /////////////////////////////////////////////////////////
                    // 2. some time BEFORE within that week (works both ways)
                    /////////////////////////////////////////////////////////
                    let instant = t_ref - 1.0 * Unit::Day;

                    // this is a simple case of a static offset
                    let dt_s = time_offset.time_correction_seconds(instant).unwrap();
                    assert_eq!(dt_s, polynomials.0); // same static offset

                    let dt_nanos = time_offset.time_correction_nanos(instant).unwrap();
                    assert_eq!(dt_nanos, polynomials.0 * 1E9); // same static offset

                    // Test that conversion did work
                    let converted = time_offset.epoch_time_correction(instant).unwrap();

                    assert_eq!(
                        converted.time_scale, *ref_ts,
                        "epoch_time_correction did not translate timescale!"
                    );

                    // this is a simple case of a static offset
                    let dt = (converted - instant).to_seconds();
                    assert_eq!(dt, polynomials.0);
                }
            }
        }
    }

    #[test]
    fn test_1ns_time_offset_drift() {
        // Tests the TimeOffset API with values slightly above hifitime precision.
        let (a0, a1, a2) = (1E-9, 1E-10, 1E-15);

        let known_timescales = [
            TimeScale::UTC,
            TimeScale::TAI,
            TimeScale::GPST,
            TimeScale::GST,
            TimeScale::BDT,
            TimeScale::QZSST,
        ];

        for ref_ts in known_timescales.iter() {
            for lhs_ts in known_timescales.iter() {
                // random t_ref in LHS timescale
                let t_ref = Epoch::from_gregorian(2020, 1, 1, 0, 0, 0, 0, *lhs_ts);

                // create valid TimeOffset
                let time_offset = TimeOffset::from_reference_epoch(t_ref, *ref_ts, (a0, a1, a2));

                if ref_ts != lhs_ts {
                    // some time later within that week
                    let instant = t_ref + 1.0 * Unit::Day;

                    // Time offset + drift so time difference is integrated
                    let interval_s = (instant - t_ref).to_seconds();
                    let expected_s = a0 + a1 * interval_s.powi(2) + a2 * interval_s.powi(2);

                    let dt_s = time_offset.time_correction_seconds(instant).unwrap();
                    assert!((dt_s - expected_s) < 1E-9);

                    // Test that conversion did work
                    let converted = time_offset.epoch_time_correction(instant).unwrap();

                    assert_eq!(
                        converted.time_scale, *ref_ts,
                        "epoch_time_correction did not translate timescale!"
                    );

                    // // Time offset only, so time difference does not impact
                    // // and both timescales are offset by a static a0 value
                    // let dt = (converted - instant).to_seconds();
                    // assert_eq!(dt, polynomials.0);
                }
            }
        }
    }

    #[test]
    fn test_sub_nano_time_offset() {
        // for what it's worth.. tests that
        // Epoch is not translated if a0 < 1ns which is below Epoch
        // precision (for all timescales).
        let polynomials = (1E-10, 0.0, 0.0);

        let known_timescales = [
            TimeScale::UTC,
            TimeScale::TAI,
            TimeScale::GPST,
            TimeScale::GST,
            TimeScale::BDT,
            TimeScale::QZSST,
        ];

        for ref_ts in known_timescales.iter() {
            for lhs_ts in known_timescales.iter() {
                // random t_ref in LHS timescale
                let t_ref = Epoch::from_gregorian(2020, 1, 1, 0, 0, 0, 0, *lhs_ts);

                // create valid TimeOffset
                let time_offset = TimeOffset::from_reference_epoch(t_ref, *ref_ts, polynomials);

                if ref_ts != lhs_ts {
                    // some time later within that week
                    let instant = t_ref + 1.0 * Unit::Day;

                    // API should work
                    let _ = time_offset.time_correction_seconds(instant).unwrap();

                    // Test that conversion did work
                    let converted = time_offset.epoch_time_correction(instant).unwrap();

                    assert_eq!(
                        converted.time_scale, *ref_ts,
                        "epoch_time_correction did not translate timescale!"
                    );

                    // Epoch should remain the same because a0 is below current Hifitime precision
                    let initial_gregorian = instant.to_gregorian_utc();
                    let converted_gregorian = converted.to_gregorian_utc();
                    assert_eq!(initial_gregorian, converted_gregorian);
                }
            }
        }
    }
}
