use crate::{
    merge::{Error as MergeError, Merge},
    processing::TimeCorrection,
};

use hifitime::{Epoch, TimeScale, Unit};

#[cfg(doc)]
use super::Timeshift;

/// [TimeCorrectionsDB] is a [TimeCorrection]s database used by [TimeScale]
/// monitoring applications and applications that need exact [TimeScale] states at all times.
/// Our [Timeshift] trait uses it in the precise conversion method.
/// [TimeCorrectionsDB] has no means to "check" the internal content with respect
/// to your application, other than the possible verification of the corrections validity (in time).
/// You are responsible of the database content with respect to the current time and keeping
/// it up to date. To avoid memory growth in long term applications, we propose:
/// - [TimeCorrectionsDB::outdate_past] to declare past [TimePolynomial]s as outdated
/// - and [TimeCorrectionsDB::outdate_weekly] to discard [TimePolynomial]s published before that week
#[derive(Default, Clone)]
pub struct TimeCorrectionsDB {
    /// When strict validity is asserted, we will respect
    /// the corrections validity strictly. Otherwise, the last
    /// correction available may be used (propagated) in the future.
    strict_validity: bool,

    /// [TimeCorrection] database
    corrections: Vec<TimeCorrection>,
}

impl TimeCorrectionsDB {
    /// The database will respect the corrections validity period strictly,
    /// and will not propose corrections past the last available in time.
    pub fn strict_validity(&self) -> Self {
        let mut s = self.clone();
        s.strict_validity = true;
        s
    }

    /// Add a new [TimeCorrection] to the database.
    /// This does not discard possible [TimeCorrection]s that may apply
    /// to these timescales.
    pub fn add(&mut self, correction: TimeCorrection) {
        self.corrections.push(correction);
    }

    /// Discard corrections past this [Epoch].
    /// Corrections must still exist or be provided quickly, for the database
    /// to remain valid.
    pub fn outdate_past(&mut self, instant: Epoch) {
        self.corrections.retain(|poly| poly.ref_epoch > instant);
    }

    /// Discard corrections published the week before this [Epoch].
    /// Corrections must still exist or be provided quickly, for the database
    /// to remain valid.
    pub fn outdate_weekly(&mut self, instant: Epoch) {
        let limit = instant - 7.0 * Unit::Week;
        self.corrections.retain(|poly| poly.ref_epoch > limit);
    }

    /// [Epoch] interpolation & correction attempt, into desired [TimeScale].
    pub fn precise_epoch_correction(&self, t: Epoch, target: TimeScale) -> Option<Epoch> {
        if t.time_scale == target {
            // nothing to be done!
            return Some(t);
        }

        if let Some(poly) = self
            .corrections
            .iter()
            .filter_map(|poly| {
                if poly.lhs_timescale == t.time_scale && poly.rhs_timescale == target {
                    Some(poly)
                } else {
                    None
                }
            })
            .min_by_key(|poly| (t - poly.ref_epoch).abs())
        {
            let mut applies = poly.applies(t);
            if !self.strict_validity {
                applies |= true;
            }

            if applies {
                Some(
                    t.precise_timescale_conversion(true, poly.ref_epoch, poly.polynomial, target)
                        .unwrap(),
                )
            } else {
                None
            }
        } else if let Some(poly) = self
            .corrections
            .iter()
            .filter_map(|poly| {
                if poly.lhs_timescale == target && poly.rhs_timescale == t.time_scale {
                    Some(poly)
                } else {
                    None
                }
            })
            .min_by_key(|poly| (t - poly.ref_epoch).abs())
        {
            let mut applies = poly.applies(t);
            if !self.strict_validity {
                applies |= true;
            }

            if applies {
                Some(
                    t.precise_timescale_conversion(false, poly.ref_epoch, poly.polynomial, target)
                        .unwrap(),
                )
            } else {
                None
            }
        } else {
            // mixed combinations not supported yet
            None
        }
    }

    // else if let Some(poly) = self
    //     .corrections
    //     .iter()
    //     .filter(|poly| {
    //         if poly.lhs_timescale == t.time_scale {
    //             Some(poly)
    //         } else {
    //             None
    //         }
    //     })
    //     .min_by_key(|poly| {
    //         let transposed = t.to_time_scale(poly.lhs_timescale);
    //         transposed - poly.ref_epoch
    //     })
    // {
    //     // got a forward (1) proposal
    //     if let Some(poly) = self
    //         .corrections
    //         .iter()
    //         .filter(|poly| {
    //             if poly.rhs_timescale == target {
    //                 Some(poly)
    //             } else {
    //                 None
    //             }
    //         })
    //         .min_by_key(|poly| {
    //             let transposed = t.to_time_scale(poly.lhs_timescale);
    //             transposed - poly.ref_epoch
    //         })
    //     {
    //         // got a forward (2) proposal
    //     } else {
    //         // got a backward (2) proposal
    //         None
    //     }
    // } else {
    //     None
    // }
    //     Some(
    //         t.precise_timescale_conversion(true, poly.ref_epoch, poly.polynomial, target)
    //             .unwrap(),
    //     )

    //     for lhs_poly in self.corrections.iter() {
    //         for rhs_poly in self.corrections.iter() {
    //             if lhs_poly.lhs_timescale == t.time_scale && rhs_poly.rhs_timescale == target {
    //                 // indirect forward transforms

    //                 // |BDT-GST|=a0_bdt & |GST-GPST|=a1 dt_gst
    //                 // GST=BDT-a0_bdt
    //                 // BDT-a0 dt_bdt - GPST = a1 dt_gpst
    //                 // BDT-GPST (foward indirect) = a1 dt_gpst + a0 dt_bdt

    //                 let dt_lhs_s = (t.to_time_scale(lhs_poly.lhs_timescale)
    //                     - lhs_poly.ref_epoch)
    //                     .to_seconds();

    //                 let dt_rhs_s = (t.to_time_scale(rhs_poly.lhs_timescale)
    //                     - rhs_poly.ref_epoch)
    //                     .to_seconds();

    //                 let mut correction = lhs_poly.polynomial.constant.to_seconds()
    //                     + lhs_poly.polynomial.rate.to_seconds() * dt_lhs_s
    //                     + lhs_poly.polynomial.accel.to_seconds() * dt_lhs_s.powi(2);

    //                 // println!("correction = {}", correction);

    //                 correction += rhs_poly.polynomial.constant.to_seconds()
    //                     + rhs_poly.polynomial.rate.to_seconds() * dt_rhs_s
    //                     + rhs_poly.polynomial.accel.to_seconds() * dt_rhs_s.powi(2);

    //                 // println!("total correction = {}", correction);

    //                 return Some(t.to_time_scale(target) - Duration::from_seconds(correction));
    //             } else if lhs_poly.rhs_timescale == t.time_scale
    //                 && rhs_poly.rhs_timescale == target
    //             {
    //                 // indirect backward + forward transforms
    //             } else if lhs_poly.lhs_timescale == t.time_scale
    //                 && rhs_poly.lhs_timescale == target
    //             {
    //                 // indirect forward + backward transforms
    //             } else if lhs_poly.rhs_timescale == t.time_scale
    //                 && rhs_poly.lhs_timescale == target
    //             {
    //                 // indirect backward transforms

    //                 // |BDT-GST|=a0_bdt & |GST-GPST|=a1 dt_gst
    //                 // BDT  = a0_bdt + GST
    //                 // GPST = GST -a1 dt_gpst
    //                 // GPST-BDT (backward indirect) = -a1 -a0

    //                 let dt_lhs_s = (t.to_time_scale(lhs_poly.lhs_timescale)
    //                     - lhs_poly.ref_epoch)
    //                     .to_seconds();

    //                 let dt_rhs_s = (t.to_time_scale(rhs_poly.lhs_timescale)
    //                     - rhs_poly.ref_epoch)
    //                     .to_seconds();

    //                 let correction_a = lhs_poly.polynomial.constant.to_seconds()
    //                     + lhs_poly.polynomial.rate.to_seconds() * dt_lhs_s
    //                     + lhs_poly.polynomial.accel.to_seconds() * dt_lhs_s.powi(2);

    //                 // println!("correction = {}", correction_a);

    //                 let correction_b = rhs_poly.polynomial.constant.to_seconds()
    //                     + rhs_poly.polynomial.rate.to_seconds() * dt_rhs_s
    //                     + rhs_poly.polynomial.accel.to_seconds() * dt_rhs_s.powi(2);

    //                 // println!("correction = {}", correction_b);

    //                 return Some(
    //                     t.to_time_scale(target)
    //                         + Duration::from_seconds(correction_a)
    //                         + Duration::from_seconds(correction_b),
    //                 );
    //             }
    //         }
    //     }

    //     None
}

impl Merge for TimeCorrectionsDB {
    fn merge(&self, rhs: &Self) -> Result<Self, MergeError>
    where
        Self: Sized,
    {
        let mut s = self.clone();
        s.merge_mut(rhs)?;

        Ok(s)
    }

    fn merge_mut(&mut self, rhs: &Self) -> Result<(), MergeError> {
        // latch new corrections
        for polynomial in rhs.corrections.iter() {
            self.corrections.push(*polynomial);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{TimeCorrection, TimeCorrectionsDB};
    use hifitime::{Duration, Epoch, Polynomial, TimeScale};
    use std::str::FromStr;

    #[test]
    fn time_corrections_db_without_strict_validity() {
        let t_ref_gpst = Epoch::from_str("2020-01-01T00:00:00 GPST").unwrap();

        let (a0, _, _) = (1.0E-9, 0.0, 0.0);

        let polynomial = Polynomial {
            constant: Duration::from_seconds(a0),
            rate: Duration::ZERO,
            accel: Duration::ZERO,
        };

        let mut database = TimeCorrectionsDB::default();

        database.add(TimeCorrection {
            lhs_timescale: TimeScale::GST,
            rhs_timescale: TimeScale::GPST,
            ref_epoch: t_ref_gpst,
            polynomial,
            validity_period: Duration::from_hours(1.0),
        });

        // Random date in GST
        let t_gst = Epoch::from_str("2020-01-01T00:00:00 GST").unwrap();

        let t_gst_gpst = database
            .precise_epoch_correction(t_gst, TimeScale::GPST)
            .unwrap();

        assert_eq!(t_gst_gpst.time_scale, TimeScale::GPST);

        // Random date in GPST
        let t_gpst = Epoch::from_str("2020-01-01T00:00:10 GPST").unwrap();

        let t_gpst_gst = database
            .precise_epoch_correction(t_gpst, TimeScale::GST)
            .unwrap();

        assert_eq!(t_gpst_gst.time_scale, TimeScale::GST);

        // Random date in UTC
        let t_utc = Epoch::from_str("2020-01-01T00:00:10 UTC").unwrap();

        assert!(database
            .precise_epoch_correction(t_utc, TimeScale::GST)
            .is_none());

        assert!(database
            .precise_epoch_correction(t_utc, TimeScale::GPST)
            .is_none());
    }

    #[test]
    #[ignore]
    fn test_indirect_forward_transform_not_utc() {
        let t_ref_bdt = Epoch::from_str("2020-01-01T00:00:00 BDT").unwrap();
        let t_ref_gst = Epoch::from_str("2020-01-01T00:00:00 GST").unwrap();
        //let t_ref_gpst = Epoch::from_str("2020-01-01T00:00:00 GPST").unwrap();

        let (a0_bdt_gst, _, _) = (1.0E-9, 0.0, 0.0);
        let (a0_gst_gpst, _, _) = (2.0E-9, 0.0, 0.0);

        let mut solver = TimeCorrectionsDB::default();

        solver.add(TimeCorrection {
            lhs_timescale: TimeScale::BDT,
            rhs_timescale: TimeScale::GST,
            validity_period: Duration::from_hours(1.0),
            ref_epoch: t_ref_bdt,
            polynomial: Polynomial {
                constant: Duration::from_seconds(a0_bdt_gst),
                rate: Duration::ZERO,
                accel: Duration::ZERO,
            },
        });

        solver.add(TimeCorrection {
            lhs_timescale: TimeScale::GST,
            rhs_timescale: TimeScale::GPST,
            validity_period: Duration::from_hours(1.0),
            ref_epoch: t_ref_gst,
            polynomial: Polynomial {
                constant: Duration::from_seconds(a0_gst_gpst),
                rate: Duration::ZERO,
                accel: Duration::ZERO,
            },
        });

        // verify direct transforms still work
        let t_gst = Epoch::from_str("2020-01-01T00:00:00 GST").unwrap();
        let t_bdt = Epoch::from_str("2020-01-01T00:00:00 BDT").unwrap();
        let t_gpst = Epoch::from_str("2020-01-01T00:00:00 GPST").unwrap();

        let t_gst_gpst = solver
            .precise_epoch_correction(t_gst, TimeScale::GPST)
            .unwrap();

        assert_eq!(t_gst_gpst.time_scale, TimeScale::GPST);

        let t_gpst_gst = solver
            .precise_epoch_correction(t_gpst, TimeScale::GST)
            .unwrap();

        assert_eq!(t_gpst_gst.time_scale, TimeScale::GST);

        let t_gst_bdt = solver
            .precise_epoch_correction(t_gst, TimeScale::BDT)
            .unwrap();

        assert_eq!(t_gst_bdt.time_scale, TimeScale::BDT);

        let t_bdt_gst = solver
            .precise_epoch_correction(t_bdt, TimeScale::GST)
            .unwrap();

        assert_eq!(t_bdt_gst.time_scale, TimeScale::GST);

        // indirect forward transform
        let t_bdt_gpst = solver
            .precise_epoch_correction(t_bdt, TimeScale::GPST)
            .unwrap();

        assert_eq!(t_bdt_gpst.time_scale, TimeScale::GPST);

        let coarsed = t_bdt.to_time_scale(TimeScale::GPST);
        let dt = coarsed - t_bdt_gpst;

        assert_eq!(
            dt,
            Duration::from_seconds(a0_bdt_gst) + Duration::from_seconds(a0_gst_gpst)
        );

        // linearity
        let reciprocal = solver
            .precise_epoch_correction(t_bdt_gpst, TimeScale::BDT)
            .unwrap();

        assert_eq!(reciprocal.time_scale, TimeScale::BDT);
        assert_eq!(reciprocal, t_bdt);

        // indirect backward transform
        let t_gpst_bdt = solver
            .precise_epoch_correction(t_gpst, TimeScale::BDT)
            .unwrap();

        assert_eq!(t_gpst_bdt.time_scale, TimeScale::BDT);

        let coarsed = t_gpst.to_time_scale(TimeScale::BDT);
        let dt = coarsed - t_gpst_bdt;

        assert_eq!(
            dt,
            Duration::from_seconds(a0_bdt_gst) + Duration::from_seconds(a0_gst_gpst)
        );

        // linearity
        let reciprocal = solver
            .precise_epoch_correction(t_gpst_bdt, TimeScale::GPST)
            .unwrap();

        assert_eq!(reciprocal.time_scale, TimeScale::GPST);
        assert_eq!(reciprocal, t_gpst);
    }

    #[test]
    #[ignore]
    fn test_indirect_forward_transform_utc() {
        let t_ref_bdt = Epoch::from_str("2020-01-01T00:00:00 BDT").unwrap();
        //let t_ref_gst = Epoch::from_str("2020-01-01T00:00:00 GST").unwrap();
        let t_ref_gpst = Epoch::from_str("2020-01-01T00:00:00 GPST").unwrap();

        let (a0_bdt_gst, _, _) = (1.0E-9, 0.0, 0.0);
        let (a0_gpst_utc, _, _) = (2.0E-9, 0.0, 0.0);

        let mut database = TimeCorrectionsDB::default();

        database.add(TimeCorrection {
            lhs_timescale: TimeScale::BDT,
            rhs_timescale: TimeScale::GST,
            ref_epoch: t_ref_bdt,
            polynomial: Polynomial {
                constant: Duration::from_seconds(a0_bdt_gst),
                rate: Duration::ZERO,
                accel: Duration::ZERO,
            },
            validity_period: Duration::from_hours(1.0),
        });

        database.add(TimeCorrection {
            lhs_timescale: TimeScale::GPST,
            rhs_timescale: TimeScale::UTC,
            ref_epoch: t_ref_gpst,
            polynomial: Polynomial {
                constant: Duration::from_seconds(a0_gpst_utc),
                rate: Duration::ZERO,
                accel: Duration::ZERO,
            },
            validity_period: Duration::from_hours(1.0),
        });

        // verify direct transforms still work
        let t_gst = Epoch::from_str("2020-01-01T00:00:00 GST").unwrap();
        let t_bdt = Epoch::from_str("2020-01-01T00:00:00 BDT").unwrap();
        let t_gpst = Epoch::from_str("2020-01-01T00:00:00 GPST").unwrap();

        let t_gpst_utc = database
            .precise_epoch_correction(t_gpst, TimeScale::UTC)
            .unwrap();

        assert_eq!(t_gpst_utc.time_scale, TimeScale::UTC);

        // linearity
        let reciprocal = database
            .precise_epoch_correction(t_gpst_utc, TimeScale::GPST)
            .unwrap();

        assert_eq!(reciprocal.time_scale, TimeScale::GPST);
        assert_eq!(reciprocal, t_gpst);

        let t_gst_bdt = database
            .precise_epoch_correction(t_gst, TimeScale::BDT)
            .unwrap();

        assert_eq!(t_gst_bdt.time_scale, TimeScale::BDT);

        let t_bdt_gst = database
            .precise_epoch_correction(t_bdt, TimeScale::GST)
            .unwrap();

        assert_eq!(t_bdt_gst.time_scale, TimeScale::GST);

        // indirect forward transform
        let t_bdt_utc = database
            .precise_epoch_correction(t_bdt, TimeScale::UTC)
            .unwrap();

        assert_eq!(t_bdt_utc.time_scale, TimeScale::UTC);

        let coarsed = t_bdt.to_time_scale(TimeScale::UTC);
        let dt = coarsed - t_bdt_utc;

        assert_eq!(
            dt,
            Duration::from_seconds(a0_bdt_gst) + Duration::from_seconds(a0_gpst_utc)
        );

        // linearity
        let reciprocal = database
            .precise_epoch_correction(t_bdt_utc, TimeScale::BDT)
            .unwrap();

        assert_eq!(reciprocal.time_scale, TimeScale::BDT);
        assert_eq!(reciprocal, t_bdt);
    }
}
