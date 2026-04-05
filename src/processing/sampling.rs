//! Sampling characteristics and definitions
use hifitime::{Duration, Epoch, TimeSeries};

/// [TemporalAxis] is associated to any dataset
/// sampled in time. The most common example being GNSS signals observation.
/// We use [TemporalAxis] to define a serie of temporal arc: continuous time windows
/// with stable sampling rate, to be used in analysis. The build block
/// is a [TimeSeries] for which the sampling rate is stable.
#[derive(Clone, Debug, PartialEq)]
pub struct TemporalAxis {
    /// List of temporal arcs
    arcs: Vec<TimeSeries>,

    /// Current pointer
    ptr: usize,
}

impl TemporalAxis {
    /// Creates a single point [TemporalAxis], this can be used to obtain
    /// a single temporal solution.
    pub fn single_point(epoch: Epoch) -> Self {
        Self {
            ptr: 0,
            arcs: vec![TimeSeries::inclusive(epoch, epoch, Duration::ZERO)],
        }
    }

    /// Returns a new [TemporalAxis] extended by one single point.
    /// It does not have to be provided in chronogical order.
    pub fn extended(&self, epoch: Epoch) -> Self {
        let mut s = self.clone();
        s.extend_mut(epoch)
        s
    }

    /// Extend this [TemporalAxis] by providing a new [Epoch],
    /// not necessarily in chronological order.
    pub fn extend_mut(&mut self, epoch: Epoch) {
        if self.is_empty() {
            self = single_point(epoch);
        } else {
            
        }
    }

    /// True if no temporal arc is defined.
    fn is_empty(&self) -> bool {
        self.arcs.is_empty()
    }

    /// Returns total number of temporal arcs.
    fn size(&self) -> usize {
        self.arcs.len()
    }

    /// Returns first temporal arc of this [TemporalAxis].
    /// A temporal arc is a time window where the sampling rate is stable.
    pub fn first_arc(&self) -> Option<TimeSeries> {
        if self.is_empty() {
            None
        } else {
            Some(self.arcs[0].clone())
        }
    }

    /// Returns temporal arc wrapping this [Epoch] and contained
    /// in this [TemporalAxis].
    pub fn temporal_arc(&self, epoch: Epoch) -> Option<TimeSeries> {
        for i in 0..self.size() {
            let (first, last) = (self.arcs[i].nth(0)?, self.arcs[i].last()?);
            if first >= epoch {
                if last <= epoch {
                    return Some(self.arcs[i].clone());
                }
            }
        }
        None
    }

    /// Returns first epoch of first temporal arc.
    pub fn first_epoch(&self) -> Option<Epoch> {
        self.first_arc()?.nth(0)
    }

    /// Returns last temporal arc of this [TemporalAxis].
    /// A temporal arc is a time window where the sampling rate is stable.
    pub fn last_arc(&self) -> Option<TimeSeries> {
        if self.is_empty() {
            None
        } else {
            Some(self.arcs[self.size() - 1].clone())
        }
    }

    /// Returns last epoch of first temporal arc.
    pub fn last_epoch(&self) -> Option<Epoch> {
        self.last_arc()?.last()
    }

    /// Returns total [Duration] of this [TemporalAxis]
    pub fn duration(&self) -> Option<Duration> {
        if let Some(first) = self.first_epoch() {
            if let Some(last) = self.last_epoch() {
                Some(last - first)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Iterator for TemporalAxis {
    type Item = Epoch;

    /// Grab next [Epoch] in this quantized [TemporalAxis].
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr >= self.size() {
            return None;
        }

        match self.arcs[self.ptr].next() {
            Some(epoch) => Some(epoch),
            None => {
                self.ptr += 1;
                self.next()
            }
        }
    }
}
