//! QcSubset definition
use crate::errors::QcSubsetError;
use gnss_rs::prelude::{Constellation, SV};
use hifitime::{Duration, Epoch};

/// [QcSubset] represents items that our filters may target.
#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub enum QcSubset {
    /// Applies to all dataset unconditionnally.
    #[default]
    All,
    /// Applies to specific Datetime ([Epoch]) only
    Datetime(Epoch),
    /// [Duration]
    Duration(Duration),
    /// Signal to noise ratio
    SNR(f64),
    /// Elevation angle in degrees
    ElevationDegrees(f64),
    /// Azimuth angle in degrees
    AzimuthDegrees(f64),
    /// List of satellites described by [SV]
    Satellites(Vec<SV>),
    /// List of [Constellation]s described by [Constellation]
    Constellations(Vec<Constellation>),
    /// Clock Offset (in seconds)
    ClockOffetSeconds(f64),
    /// Clock drift (in s.s⁻¹)
    ClockDriftSecondsSeconds(f64),
    /// Single readable item we cannot interprate at this level
    String(String),
    /// List of string we cannot interprate at this level, orginnaly described by CSV.
    CsvStringArray(Vec<String>),
}

impl QcSubset {
    /// Builds a [QcSubset::Datetime]
    pub fn from_datetime(t: Epoch) -> Self {
        Self::Datetime(t)
    }

    /// Builds a [QcSubset::Duration]
    pub fn from_duration(dt: Duration) -> Self {
        Self::Duration(dt)
    }

    /// Builds a [QcSubset::Satellites] from this unique [SV]
    pub fn from_satellite(sv: SV) -> Self {
        Self::Satellites(vec![sv])
    }

    /// Builds a [QcSubset::Satellites] from list of [SV]
    pub fn from_satellites(sv: &[SV]) -> Self {
        Self::Satellites(sv.to_vec())
    }

    /// Builds a [QcSubset::Constellations] from this unique [Constellation]
    pub fn from_constellation(constellation: Constellation) -> Self {
        Self::Constellations(vec![constellation])
    }

    /// Builds a [QcSubset::Constellations] from list of [Constellation]s
    pub fn from_constellations(constellations: &[Constellation]) -> Self {
        Self::Constellations(constellations.to_vec())
    }

    /// Builds a [QcSubset::ElevationDegrees]
    pub fn from_elevation_deg(elev_deg: f64) -> Self {
        Self::ElevationDegrees(elev_deg)
    }

    /// Builds a [QcSubset::ElevationDegrees] from elevation angle in radians
    pub fn from_elevation_rad(elev_rad: f64) -> Self {
        Self::ElevationDegrees(elev_rad.to_degrees())
    }

    /// Builds a [QcSubset::AzimuthDegrees]
    pub fn from_azimuth_deg(azim_deg: f64) -> Self {
        Self::AzimuthDegrees(azim_deg)
    }

    /// Builds a [QcSubset::AzimuthDegrees] from azimuth angle in radians
    pub fn from_azimuth_rad(azim_rad: f64) -> Self {
        Self::AzimuthDegrees(azim_rad.to_degrees())
    }

    /// Builds a [QcSubset::SNR]
    pub fn from_snr(snr: f64) -> Self {
        Self::SNR(snr)
    }

    /// Builds a [QcSubset::ClockOffsetSeconds] from offset in seconds
    pub fn from_clock_offset_sec(offset_s: f64) -> Self {
        Self::ClockOffetSeconds(offset_s)
    }

    /// Builds a [QcSubset::ClockDriftSecondsSeconds] from drift in s.s⁻¹
    pub fn from_clock_drift_sec_sec(drift_s_s: f64) -> Self {
        Self::ClockDriftSecondsSeconds(drift_s_s)
    }

    /// Builds a [QcSubset::ComplexString] from uninterpretable string description
    pub fn from_complex_str(s: &str) -> Self {
        Self::ComplexString(s.to_string())
    }

    /// Builds a [QcSubset::ComplexStringArray] from array of uninterpretable string descriptions
    pub fn from_complex_str_array(s: &[&str]) -> Self {
        Self::ComplexStringArray(s.iter().map(|s| s.to_string()).collect())
    }
}

// pub(crate) fn parse_sv_list(items: Vec<&str>) -> Result<Vec<SV>, SVParsingError> {
//     let mut ret: Vec<SV> = Vec::with_capacity(items.len());
//     for item in items {
//         let sv = SV::from_str(item.trim())?;
//         ret.push(sv);
//     }
//     Ok(ret)
// }
//
// pub(crate) fn parse_gnss_list(
//     items: Vec<&str>,
// ) -> Result<Vec<Constellation>, ConstellationParsingError> {
//     let mut ret: Vec<Constellation> = Vec::with_capacity(items.len());
//     for item in items {
//         let c = Constellation::from_str(item.trim())?;
//         ret.push(c);
//     }
//     Ok(ret)
// }
//
// fn parse_float_payload(content: &str) -> Result<f64, ParseFloatError> {
//     f64::from_str(content.trim())
// }

// // use itertools::Itertools;
//
impl std::str::FromStr for QcSubset {
    type Err = QcSubsetError;
    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let c = content.trim();
        let items: Vec<&str> = c.split(',').collect();

        if items.len() < 1 {
            return Err(QcSubsetError::EmptyItem);
        }

        let trimmed = items[0].trim();

        // type guessing
        if let Ok(t) = Epoch::from_str(trimmed) {
            Ok(Self::Datetime(t))
        } else if let Ok(dt) = Duration::from_str(trimmed) {
            Ok(Self::Duration(dt))
        } else if let Ok(sv) = SV::from_str(trimmed) {
            Ok(Self::from_satellite(sv))
        } else if let Ok(constell) = Constellation::from_str(trimmed) {
            Ok(Self::from_constellation(constell))
        } else {
            Err(QcSubsetError::EmptyItem)
        }
    }
}

impl std::fmt::Display for QcSubset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Datetime(t) => write!(f, "{}", t),
            Self::Duration(dt) => write!(f, "dt = {}", dt),
            Self::AzimuthDegrees(azim) => write!(f, "azim ={:.3}°", azim),
            Self::ElevationDegrees(elev) => write!(f, "elev ={:.3}°", elev),
            Self::ClockOffetSeconds(dt) => write!(f, "clock = {:.5e}s", dt),
            Self::ClockDriftSecondsSeconds(dt) => write!(f, "drift = {:.5e}s/s", dt),
            Self::SNR(snr) => write!(f, "snr = {:.2}", snr),
            Self::Satellites(svnn) => {
                if svnn.len() == 1 {
                    write!(f, "sv = {}", svnn[0])
                } else {
                    write!(f, "sv = {:?}", svnn)
                }
            }
            Self::Constellations(constells) => {
                if constells.len() == 1 {
                    write!(f, "gnss = {}", constells[0])
                } else {
                    write!(f, "gnss = {:?}", constells)
                }
            }
            Self::ComplexString(s) => write!(f, "{}", s),
            Self::ComplexStringArray(array) => {
                if array.len() == 1 {
                    write!(f, "{}", array[0])
                } else {
                    write!(f, "{:?}", array)
                }
            }
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use gnss_rs::prelude::{Constellation, SV};
//     use std::str::FromStr;
//     #[test]
//     fn algo_target_item() {
//         let e = Epoch::default();
//         let target: FilterItem = e.into();
//         assert_eq!(target, FilterItem::EpochItem(e));
//
//         assert_eq!(
//             FilterItem::from_str("g08,g09,R03").unwrap(),
//             FilterItem::SvItem(vec![
//                 SV::from_str("G08").unwrap(),
//                 SV::from_str("G09").unwrap(),
//                 SV::from_str("R03").unwrap()
//             ])
//         );
//
//         assert_eq!(
//             FilterItem::from_str("GPS , BDS").unwrap(),
//             FilterItem::ConstellationItem(vec![Constellation::GPS, Constellation::BeiDou])
//         );
//
//         let dt = Duration::from_str("1 d").unwrap();
//         let target: FilterItem = dt.into();
//         assert_eq!(target, FilterItem::DurationItem(dt));
//     }
//     #[test]
//     fn test_from_elevation() {
//         let desc = "90";
//         assert!(
//             FilterItem::from_elevation(desc).is_ok(),
//             "Failed to parse Elevation Target Item"
//         );
//     }
//     #[test]
//     fn test_from_azimuth() {
//         let desc = " 12.34  ";
//         assert!(
//             FilterItem::from_azimuth(desc).is_ok(),
//             "Failed to parse Azimuth Target Item"
//         );
//     }
//     #[test]
//     fn test_from_snr() {
//         let desc = " 12.34  ";
//         assert!(
//             FilterItem::from_snr(desc).is_ok(),
//             "Failed to parse SNR Target Item"
//         );
//     }
// }
