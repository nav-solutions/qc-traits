use crate::{
    errors::QcSelectionError,
    prelude::{Constellation, Duration, Epoch, SV},
    QcAngle,
};

#[derive(Clone, Debug, PartialEq)]
pub enum QcSelectionStepItem {
    /// Applies to specific Datetime ([Epoch]) only
    Datetime(Epoch),
    /// [Duration]
    Duration(Duration),
    /// Elevation angle
    Elevation(QcAngle),
    /// Azimuth angle
    Azimuth(QcAngle),
    /// List of satellites described by [SV]
    Satellites(Vec<SV>),
    /// List of [Constellation]s described by [Constellation]
    Constellations(Vec<Constellation>),
    /// Readable CSV description we cannot interprate at this level.
    UninterpretedCsvString(String),
}

impl QcSelectionStepItem {
    /// Builds a [QcSelectionStepItem::Datetime]
    pub fn from_datetime(t: Epoch) -> Self {
        Self::Datetime(t)
    }

    /// Builds a [QcSelectionStepItem::Duration]
    pub fn from_duration(dt: Duration) -> Self {
        Self::Duration(dt)
    }

    /// Builds a [QcSelectionStepItem::Satellites] from this unique [SV]
    pub fn from_satellite(sv: SV) -> Self {
        Self::Satellites(vec![sv])
    }

    /// Builds a [QcSelectionStepItem::Satellites] from list of [SV]
    pub fn from_satellites(sv: &[SV]) -> Self {
        Self::Satellites(sv.to_vec())
    }

    /// Builds a [QcSelectionStepItem::Constellations] from this unique [Constellation]
    pub fn from_constellation(constellation: Constellation) -> Self {
        Self::Constellations(vec![constellation])
    }

    /// Builds a [QcSelectionStepItem::Constellations] from list of [Constellation]s
    pub fn from_constellations(constellations: &[Constellation]) -> Self {
        Self::Constellations(constellations.to_vec())
    }

    /// Builds a [QcSelectionStepItem::Elevation]
    pub fn from_elevation_deg(deg: f64) -> Self {
        Self::Elevation(QcAngle::from_degrees(deg))
    }

    /// Builds a [QcSelectionStepItem::Elevation] from elevation angle in radians
    pub fn from_elevation_rad(rad: f64) -> Self {
        Self::Elevation(QcAngle::from_radians(rad))
    }

    /// Builds a [QcSelectionStepItem::Azimuth] from azimuth angle in degrees
    pub fn from_azimuth_deg(deg: f64) -> Self {
        Self::Azimuth(QcAngle::from_degrees(deg))
    }

    /// Builds a [QcSelectionStepItem::Azimuth] from azimuth angle in radians
    pub fn from_azimuth_rad(rad: f64) -> Self {
        Self::Azimuth(QcAngle::from_radians(rad))
    }

    /// Builds a [QcSelectionStepItem::UninterpretedCsvString] we cannot interprate at this level.
    pub fn from_csv_string(s: &str) -> Self {
        Self::UninterpretedCsvString(s.to_string())
    }
}

impl std::str::FromStr for QcSelectionStepItem {
    type Err = QcSelectionError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let c = content.trim();
        let items: Vec<&str> = c.split(',').collect();

        if items.len() < 1 {
            return Err(QcSelectionError::EmptyStep);
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
            Err(QcSelectionError::EmptyStep)
        }
    }
}

impl std::fmt::Display for QcSelectionStepItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Datetime(t) => write!(f, "{}", t),
            Self::Duration(dt) => write!(f, "dt = {}", dt),
            Self::Azimuth(angle) => write!(f, "az {}", angle),
            Self::Elevation(angle) => write!(f, "el {}", angle),
            Self::Satellites(sv) => write!(f, "{:?}", sv),
            Self::Constellations(c) => write!(f, "{:?}", c),
            Self::UninterpretedCsvString(s) => write!(f, "{}", s),
        }
    }
}
