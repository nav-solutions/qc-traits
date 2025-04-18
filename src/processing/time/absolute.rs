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
        self.polynomials
            .retain(|t| t.ref_epoch.time_scale != polynomial.ref_epoch.time_scale);

        self.polynomials.push(polynomial);
    }

    /// [Epoch] interpolation & correction attempt, into desired [TimeScale].
    pub fn epoch_time_correction(&self, t: Epoch, target: TimeScale) -> Option<Epoch> {
        if let Some(polynomials) = self
            .polynomials
            .iter()
            .find(|poly| poly.lhs_timescale == t.time_scale && poly.ref_epoch.time_scale == target)
        {
            Some(polynomials.epoch_time_correction(true, t, target))
        } else if let Some(polynomials) = self
            .polynomials
            .iter()
            .find(|poly| poly.ref_epoch.time_scale == t.time_scale && poly.lhs_timescale == target)
        {
            Some(polynomials.epoch_time_correction(false, t, target))
        } else {
            //GPST-UTC=a
            //UTC=GPST-a =  forward
            //GPST=a+UTC = backward

            // try to form a combination
            for poly_1 in self.polynomials.iter() {
                if poly_1.lhs_timescale == t.time_scale && poly_1.ref_epoch.time_scale != target {
                    for poly_2 in self.polynomials.iter() {
                        if poly_2.lhs_timescale != t.time_scale
                            && poly_2.ref_epoch.time_scale == target
                        {
                            // GPST-UTC
                            // GPST-GST=a1  GST-UTC=a2
                            // GPST-GST    + GST-UTC  a1 + GST     + a2-GST
                            let mut correction = poly_1.correction_duration(t);
                            correction += poly_2.correction_duration(t);

                            let converted = t.to_time_scale(target);
                            return Some(converted + correction);
                        }
                    }
                }
            }

            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{GnssAbsoluteTime, TimePolynomial};
    use hifitime::{Epoch, TimeScale};
    use std::str::FromStr;

    #[test]
    fn test_direct_absolute_time() {
        let polynomials = [TimePolynomial {
            lhs_timescale: TimeScale::GST,
            ref_epoch: Epoch::from_str("2020-01-01T00:00:00 GPST").unwrap(),
            polynomials: (1.0E-9, 1.0E-10, 0.0),
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
                polynomials: (1.0E-9, 1.0E-10, 0.0),
            },
            TimePolynomial {
                lhs_timescale: TimeScale::UTC,
                ref_epoch: Epoch::from_str("2020-01-01T00:00:00 GPST").unwrap(),
                polynomials: (2.0E-9, 2.0E-10, 0.0),
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

        let t_gst_gpst = solver
            .epoch_time_correction(t_gst, TimeScale::GPST)
            .unwrap();

        assert_eq!(t_gst_gpst.time_scale, TimeScale::GPST);

        // let t_gst_gpst = solver.epoch_time_correction(t_gst, TimeScale::GPST)
        //     .unwrap();

        // assert_eq!(t_gst_gpst.time_scale, TimeScale::GPST);

        // let t_gpst_gst = solver.epoch_time_correction(t_gpst, TimeScale::GST)
        //     .unwrap();

        // assert_eq!(t_gpst_gst.time_scale, TimeScale::GST);
    }
}
