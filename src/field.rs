use hifitime::prelude::Epoch;

/// [QcField] describes interesting data fields
#[derive(Debug, Clone)]
pub enum QcField {
    /// 3D ECEF coordinates (m)
    EcefCoordinates3d((f64, f64, f64)),
    /// 3D Geodetic coordinates (lat°, long°, alt(m))
    GeoCoordinates3d((f64, f64, f64)),
    /// Operator name
    Operator(String),
    /// Agency / publisher name
    Agency(String),
    /// First declared datetime
    StartDateTime(Epoch),
    /// Last declared datetime
    EndDateTIme(Epoch),
    /// Scaling (static offset)
    Scaling(f64),
}
