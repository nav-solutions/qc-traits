use hifitime::{Duration, Epoch, TimeScale, Unit};

/// [TimePolynomial] allows precise [Epoch] translation to another [TimeScale].
/// For example, |[TimeScale::GPST]-[TimeScale::UTC]| when referencing [TimeScale::GPST] to [TimeScale::UTC].
#[derive(Copy, Clone, PartialEq)]
pub struct TimePolynomial {
    /// lhs [TimeScale]
    pub lhs_timescale: TimeScale,
    /// Reference [Epoch] expressed in the [TimeScale] in which the polynomials apply,
    /// with 1 ns accuracy.
    pub ref_epoch: Epoch,
    /// Polynomials (s, s.s⁻¹, s.s⁻²)
    pub polynomials: (f64, f64, f64),
}

impl TimePolynomial {
    /// Define new [TimePolynomial] from Reference [Epoch] expressed as week counter
    /// and elapsed seconds within week.
    pub fn from_reference_time_of_week_seconds(
        lhs_timescale: TimeScale,
        ref_week: u32,
        ref_tow: u64,
        ref_timescale: TimeScale,
        polynomials: (f64, f64, f64),
    ) -> Self {
        Self::from_reference_time_of_week_nanos(
            lhs_timescale,
            ref_week,
            ref_tow * 1_000_000_000,
            ref_timescale,
            polynomials,
        )
    }

    /// Define a new [TimePolynomials] from Reference [Epoch] expressed as week counter and
    /// elapsed nanoseconds within week.
    pub fn from_reference_time_of_week_nanos(
        lhs_timescale: TimeScale,
        ref_week: u32,
        ref_tow: u64,
        ref_timescale: TimeScale,
        polynomials: (f64, f64, f64),
    ) -> Self {
        let ref_epoch = Epoch::from_time_of_week(ref_week, ref_tow, ref_timescale);

        Self {
            ref_epoch,
            lhs_timescale,
            polynomials,
        }
    }

    pub fn correction_duration(&self, t: Epoch) -> Duration {
        let t = t.to_time_scale(self.lhs_timescale);
        let t_ref = self.ref_epoch.to_time_scale(self.lhs_timescale);

        let dt_s = (t - t_ref).to_seconds();
        let (a0, a1, a2) = self.polynomials;

        Duration::from_seconds(a0 + a1 * dt_s + a2 * dt_s.powi(2))
    }

    /// Converts and corrects provided [Epoch] into provided [TimeScale], with 1 ns accuracy.
    pub fn epoch_time_correction(&self, forward: bool, t: Epoch, timescale: TimeScale) -> Epoch {
        let t_ref = self.ref_epoch.to_time_scale(t.time_scale);

        // supports any offset to reference instant.
        // While usually this should remain within the same week (or even much closer)
        let dt_s = (t - t_ref).to_seconds();

        let (a0, a1, a2) = self.polynomials;
        let dt_s = a0 + a1 * dt_s + a2 * dt_s.powi(2);

        let converted = t.to_time_scale(timescale);

        if forward {
            converted - dt_s * Unit::Second
        } else {
            converted + dt_s * Unit::Second
        }
    }
}

#[cfg(test)]
mod test {
    use crate::TimePolynomial;
    use hifitime::{Epoch, TimeScale, Unit};

    #[test]
    fn test_time_polynomials() {
        let t_ref_gpst = Epoch::from_gregorian(2020, 1, 1, 0, 0, 0, 0, TimeScale::GPST);

        let (a0, a1, a2) = (1.0E-9, 1.0E-10, 0.0);

        let time_offset = TimePolynomial {
            ref_epoch: t_ref_gpst,
            polynomials: (a0, a1, a2),
            lhs_timescale: TimeScale::UTC,
        };

        // +1 s
        let t = t_ref_gpst + 1.0 * Unit::Second;

        let t_utc = time_offset.epoch_time_correction(true, t, TimeScale::UTC);

        assert_eq!(t_utc.time_scale, TimeScale::UTC);

        assert_eq!(
            t_utc,
            t.to_time_scale(TimeScale::UTC) - (a0 * Unit::Second + 1.0 * Unit::Second * a1)
        );

        let reversed = time_offset.epoch_time_correction(false, t_utc, TimeScale::GPST);
        assert_eq!(reversed, t);

        // test backwards
        let backwards = time_offset.epoch_time_correction(false, t, TimeScale::GPST);
        assert_eq!(backwards.time_scale, TimeScale::GPST);

        // +1 ns
        let t = t_ref_gpst + 1.0 * Unit::Nanosecond;

        let t_utc = time_offset.epoch_time_correction(true, t, TimeScale::UTC);

        assert_eq!(t_utc.time_scale, TimeScale::UTC);

        let correction_s = a0 + 1.0E-9 * a1;
        assert_eq!(
            t_utc,
            t.to_time_scale(TimeScale::UTC) - correction_s * Unit::Second
        );

        let reversed = time_offset.epoch_time_correction(false, t_utc, TimeScale::GPST);
        assert_eq!(reversed, t);

        // +1 us
        let t = t_ref_gpst + 1000.0 * Unit::Nanosecond;

        let t_utc = time_offset.epoch_time_correction(true, t, TimeScale::UTC);

        assert_eq!(t_utc.time_scale, TimeScale::UTC);

        let correction_s = a0 + 1000.0E-9 * a1;
        assert_eq!(
            t_utc,
            t.to_time_scale(TimeScale::UTC) - correction_s * Unit::Second
        );

        let reversed = time_offset.epoch_time_correction(false, t_utc, TimeScale::GPST);
        assert_eq!(reversed, t);
    }
}
