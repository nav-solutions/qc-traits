use hifitime::{Duration, Epoch, Polynomial, TimeScale};

#[cfg(feature = "python")]
use pyo3::prelude::pyclass;

#[cfg(feature = "python")]
use pyo3::{pymethods, PyResult};

/// [TimeCorrection] allows precise [Epoch] translation to another [TimeScale].
/// For example, |[TimeScale::GPST]-[TimeScale::UTC]| when referencing [TimeScale::GPST] to [TimeScale::UTC].
#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(feature = "python", pyclass)]
#[cfg_attr(feature = "python", pyo3(module = "qc_traits"))]
pub struct TimeCorrection {
    /// LHS [TimeScale] to which [Polynomial] applies
    pub lhs_timescale: TimeScale,

    /// RHS [TimeScale] to which [Polynomial] applies
    pub rhs_timescale: TimeScale,

    /// Reference [Epoch] usually expressed in LHS [TimeScale], but we support any [TimeScale] here.
    pub ref_epoch: Epoch,

    /// Validity period as [Duration]
    pub validity_period: Duration,

    /// [Polynomial]
    pub polynomial: Polynomial,
}

impl core::fmt::Display for TimeCorrection {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "({}-{})={} at {}",
            self.lhs_timescale, self.rhs_timescale, self.polynomial, self.ref_epoch
        )
    }
}

impl TimeCorrection {
    /// Define new [TimeCorrection] from Reference [Epoch] expressed as week counter
    /// and elapsed seconds within week.
    pub fn from_reference_time_of_week_seconds(
        ref_week: u32,
        ref_tow: u64,
        validity_period: Duration,
        lhs_timescale: TimeScale,
        rhs_timescale: TimeScale,
        polynomial: Polynomial,
    ) -> Self {
        Self::from_reference_time_of_week_nanos(
            ref_week,
            ref_tow * 1_000_000_000,
            validity_period,
            lhs_timescale,
            rhs_timescale,
            polynomial,
        )
    }

    /// Define new [TimeCorrection] from reference [Epoch] that must be expressed in the correct [TimeScale]
    pub fn from_reference_epoch(
        ref_epoch: Epoch,
        validity_period: Duration,
        rhs_timescale: TimeScale,
        polynomial: Polynomial,
    ) -> Self {
        Self {
            ref_epoch,
            validity_period,
            lhs_timescale: ref_epoch.time_scale,
            rhs_timescale,
            polynomial,
        }
    }

    /// Define a new [TimeCorrections] from Reference [Epoch] expressed as week counter and
    /// elapsed nanoseconds within week.
    pub fn from_reference_time_of_week_nanos(
        ref_week: u32,
        ref_tow_nanos: u64,
        validity_period: Duration,
        lhs_timescale: TimeScale,
        rhs_timescale: TimeScale,
        polynomial: Polynomial,
    ) -> Self {
        let ref_epoch = Epoch::from_time_of_week(ref_week, ref_tow_nanos, lhs_timescale);

        Self {
            ref_epoch,
            validity_period,
            lhs_timescale,
            rhs_timescale,
            polynomial,
        }
    }

    /// Returns true if this [TimeCorrection] should apply at ongoing [Epoch],
    /// acoording to publication validity period.
    pub fn applies(&self, now: Epoch) -> bool {
        let dt = (now - self.ref_epoch).abs();
        dt < self.validity_period
    }

    /// Returns first [Epoch] for which this [TimeCorrection] should apply.
    pub fn validity_period_start(&self) -> Epoch {
        self.ref_epoch - self.validity_period
    }

    /// Returns last [Epoch] for which this [TimeCorrection] should apply.
    pub fn validity_period_end(&self) -> Epoch {
        self.ref_epoch + self.validity_period
    }
}

#[pymethods]
impl TimeCorrection {
    #[new]
    fn py_new(lhs: TimeScale, rhs: TimeScale, ref_epoch: Epoch, polynomial: Polynomial) -> Self {
        Self {
            lhs_timescale: lhs,
            rhs_timescale: rhs,
            ref_epoch,
            validity_period: Default::default(),
            polynomial,
        }
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }

    fn __format__(&self, _specs: &str) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}
