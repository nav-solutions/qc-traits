use super::TimePolynomial;

use hifitime::{Epoch, TimeScale};

#[cfg(doc)]
use super::Timeshift;

/// [GnssAbsoluteTime] is used by the [Timeshift] trait and applications
/// that need to manage several timescales and allow correction conversion
/// back & forth at all times.
#[derive(Default)]
pub struct GnssAbsoluteTime {
    polynomials: Vec<TimePolynomial>,
}

impl GnssAbsoluteTime {
    /// Create a new [GnssAbsoluteTime] manager, from current [TimePolynomial] database knowledge.
    pub fn new(polynomials: &[TimePolynomial]) -> Self {
        Self {
            polynomials: polynomials.to_vec(),
        }
    }

    /// Add a new [TimePolynomial] to this management pool.
    /// Usually right after its publication.
    pub fn add_polynomial(&mut self, polynomial: TimePolynomial) {
        self.polynomials.retain(|poly| {
            let same_ref = poly.ref_epoch.time_scale == polynomial.ref_epoch.time_scale;

            let same_lhs = poly.ref_epoch.time_scale == polynomial.ref_epoch.time_scale;
            !(same_ref && same_lhs)
        });

        self.polynomials.push(polynomial);
    }

    /// [Epoch] interpolation & correction attempt, into desired [TimeScale].
    pub fn epoch_time_correction(&self, t: Epoch, target: TimeScale) -> Option<Epoch> {
        // default case:
        if t.time_scale == target {
            return Some(t);
        }

        if let Some(poly) = self
            .polynomials
            .iter()
            .find(|poly| poly.lhs_timescale == t.time_scale && poly.ref_epoch.time_scale == target)
        {
            Some(
                t.precise_timescale_conversion(true, poly.ref_epoch, poly.polynomial, target)
                    .unwrap(),
            )
        } else if let Some(poly) = self
            .polynomials
            .iter()
            .find(|poly| poly.ref_epoch.time_scale == t.time_scale && poly.lhs_timescale == target)
        {
            Some(
                t.precise_timescale_conversion(false, poly.ref_epoch, poly.polynomial, target)
                    .unwrap(),
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{GnssAbsoluteTime, TimePolynomial};
    use hifitime::{Duration, Epoch, Polynomial, TimeScale};
    use std::str::FromStr;

    #[test]
    fn test_direct_absolute_time() {
        let polynomials = [TimePolynomial {
            lhs_timescale: TimeScale::GST,
            ref_epoch: Epoch::from_str("2020-01-01T00:00:00 GPST").unwrap(),
            polynomial: Polynomial {
                constant: Duration::from_seconds(1.0E-9),
                rate: Duration::from_seconds(1.0E-10),
                accel: Duration::from_seconds(0.0),
            },
        }];

        let solver = GnssAbsoluteTime::new(&polynomials);

        let t_gst = Epoch::from_str("2020-01-01T00:00:10 GST").unwrap();
        let t_gpst = Epoch::from_str("2020-01-01T00:00:10 GPST").unwrap();

        let t_gst_gpst = solver
            .epoch_time_correction(t_gst, TimeScale::GPST)
            .unwrap();

        assert_eq!(t_gst_gpst.time_scale, TimeScale::GPST);

        let t_gpst_gst = solver
            .epoch_time_correction(t_gpst, TimeScale::GST)
            .unwrap();

        assert_eq!(t_gpst_gst.time_scale, TimeScale::GST);
    }

    #[test]
    fn test_indirect_absolute_time() {
        let polynomials = [
            TimePolynomial {
                lhs_timescale: TimeScale::GST,
                ref_epoch: Epoch::from_str("2020-01-01T00:00:00 UTC").unwrap(),
                polynomial: Polynomial {
                    constant: Duration::from_seconds(1.0E-9),
                    rate: Duration::from_seconds(1.0E-10),
                    accel: Duration::from_seconds(0.0),
                },
            },
            TimePolynomial {
                lhs_timescale: TimeScale::UTC,
                ref_epoch: Epoch::from_str("2020-01-01T00:00:00 GPST").unwrap(),
                polynomial: Polynomial {
                    constant: Duration::from_seconds(2.0E-9),
                    rate: Duration::from_seconds(2.0E-10),
                    accel: Duration::from_seconds(0.0),
                },
            },
        ];

        let solver = GnssAbsoluteTime::new(&polynomials);

        let t_gst = Epoch::from_str("2020-01-01T00:00:10 GST").unwrap();
        let t_utc = Epoch::from_str("2020-01-01T00:00:10 UTC").unwrap();
        let t_gpst = Epoch::from_str("2020-01-01T00:00:10 GPST").unwrap();

        let t_gst_utc = solver.epoch_time_correction(t_gst, TimeScale::UTC).unwrap();

        assert_eq!(t_gst_utc.time_scale, TimeScale::UTC);

        let t_utc_gpst = solver
            .epoch_time_correction(t_utc, TimeScale::GPST)
            .unwrap();

        assert_eq!(t_utc_gpst.time_scale, TimeScale::GPST);

        let t_utc_gpst = solver
            .epoch_time_correction(t_utc, TimeScale::GPST)
            .unwrap();

        assert_eq!(t_utc_gpst.time_scale, TimeScale::GPST);

        let t_gpst_utc = solver
            .epoch_time_correction(t_gpst, TimeScale::UTC)
            .unwrap();

        assert_eq!(t_gpst_utc.time_scale, TimeScale::UTC);

        // not feasible yet
        // let t_gst_gpst = solver
        //     .epoch_time_correction(t_gst, TimeScale::GPST)
        //     .unwrap();

        // assert_eq!(t_gst_gpst.time_scale, TimeScale::GPST);

        // let t_gst_gpst = solver.epoch_time_correction(t_gst, TimeScale::GPST)
        //     .unwrap();

        // assert_eq!(t_gst_gpst.time_scale, TimeScale::GPST);

        // let t_gpst_gst = solver.epoch_time_correction(t_gpst, TimeScale::GST)
        //     .unwrap();

        // assert_eq!(t_gpst_gst.time_scale, TimeScale::GST);
    }
}
