use crate::errors::QcFilterError;

/// MaskOperand describes how to apply a given mask
#[derive(Debug, Clone, PartialEq)]
pub enum MaskOperand {
    /// Greater than, is symbolized by ">".
    GreaterThan,
    /// Greater Equals, symbolized by ">=".
    GreaterEquals,
    /// Lower than, symbolized by "<"."
    LowerThan,
    /// Lower Equals, symbolized by "<=".
    LowerEquals,
    /// Equals, symbolized by "=".
    /// Equals operand is implied anytime the operand is omitted in the description.
    Equals,
    /// Not Equals, symbolized by "!=".
    NotEquals,
}

impl std::str::FromStr for MaskOperand {
    type Err = QcFilterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.eq(">") {
            Ok(Self::GreaterThan)
        } else if trimmed.eq("<") {
            Ok(Self::LowerThan)
        } else if trimmed.eq("=") {
            Ok(Self::Equals)
        } else if trimmed.eq("!=") {
            Ok(Self::NotEquals)
        } else if trimmed.eq(">=") {
            Ok(Self::GreaterEquals)
        } else if trimmed.eq("<=") {
            Ok(Self::LowerEquals)
        } else {
            Err(QcFilterError::InvalidOperand)
        }
    }
}

impl std::ops::Not for MaskOperand {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Self::Equals => Self::NotEquals,
            Self::NotEquals => Self::Equals,
            Self::GreaterEquals => Self::LowerEquals,
            Self::GreaterThan => Self::LowerThan,
            Self::LowerThan => Self::GreaterThan,
            Self::LowerEquals => Self::GreaterEquals,
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use gnss_rs::prelude::{Constellation, SV};
//     use hifitime::Epoch;
//     use std::str::FromStr;

//     #[test]
//     fn mask_operand() {
//         for (descriptor, opposite_desc) in [
//             (">=", "<="),
//             (">", "<"),
//             ("=", "!="),
//             ("<", ">"),
//             ("<=", ">="),
//         ] {
//             let operand = MaskOperand::from_str(descriptor);
//             assert!(
//                 operand.is_ok(),
//                 "{} \"{}\"",
//                 "Failed to parse MaskOperand from",
//                 descriptor
//             );
//             let opposite = MaskOperand::from_str(opposite_desc);
//             assert!(
//                 opposite.is_ok(),
//                 "{} \"{}\"",
//                 "Failed to parse MaskOperand from",
//                 opposite_desc
//             );
//             assert_eq!(!operand.unwrap(), opposite.unwrap(), "MaskOperand::Not()");
//         }

//         let operand = MaskOperand::from_str("a");
//         assert!(
//             operand.is_err(),
//             "Parsed unexpectedly \"{}\" MaskOperand correctly",
//             "a"
//         );
//     }
//     #[test]
//     fn mask_epoch() {
//         let mask = MaskFilter::from_str(">2020-01-14T00:31:55 UTC").unwrap();
//         assert_eq!(
//             mask,
//             MaskFilter {
//                 operand: MaskOperand::GreaterThan,
//                 item: FilterItem::EpochItem(Epoch::from_str("2020-01-14T00:31:55 UTC").unwrap()),
//             }
//         );
//         let mask = MaskFilter::from_str(">JD 2452312.500372511 TAI");
//         assert!(mask.is_ok());
//     }
//     #[test]
//     fn mask_elev() {
//         for (desc, valid) in [
//             ("e>1.0", true),
//             ("e< 40.0", true),
//             ("e != 30", true),
//             (" e<40.0", true),
//             (" e < 40.0", true),
//             (" e > 120", false),
//             (" e >= 120", false),
//             (" e = 30", true),
//         ] {
//             let mask = MaskFilter::from_str(desc);
//             assert_eq!(
//                 mask.is_ok(),
//                 valid,
//                 "failed to parse elevation mask filter \"{}\"",
//                 desc
//             );
//         }
//     }
//     #[test]
//     fn mask_gnss() {
//         for (descriptor, opposite_desc) in [
//             (" = GPS", "!= GPS"),
//             ("= GAL,GPS", "!= GAL,GPS"),
//             (" =GLO,GAL", "!=  GLO,GAL"),
//         ] {
//             let mask = MaskFilter::from_str(descriptor);
//             assert!(
//                 mask.is_ok(),
//                 "Unable to parse MaskFilter from \"{}\"",
//                 descriptor
//             );
//             let opposite = MaskFilter::from_str(opposite_desc);
//             assert!(
//                 opposite.is_ok(),
//                 "Unable to parse MaskFilter from \"{}\"",
//                 opposite_desc
//             );
//             assert_eq!(!mask.unwrap(), opposite.unwrap(), "{}", "MaskFilter::Not()");
//         }

//         let mask = MaskFilter::from_str("=GPS,GAL,GLO").unwrap();
//         assert_eq!(
//             mask,
//             MaskFilter {
//                 operand: MaskOperand::Equals,
//                 item: FilterItem::ConstellationItem(vec![
//                     Constellation::GPS,
//                     Constellation::Galileo,
//                     Constellation::Glonass
//                 ]),
//             }
//         );

//         let mask = MaskFilter::from_str("!=BDS").unwrap();
//         assert_eq!(
//             mask,
//             MaskFilter {
//                 operand: MaskOperand::NotEquals,
//                 item: FilterItem::ConstellationItem(vec![Constellation::BeiDou]),
//             }
//         );
//     }
//     #[test]
//     fn mask_sv() {
//         for (descriptor, opposite_desc) in [(" = G01", "!= G01"), ("= R03,  G31", "!= R03,  G31")] {
//             let mask = MaskFilter::from_str(descriptor);
//             assert!(
//                 mask.is_ok(),
//                 "Unable to parse MaskFilter from \"{}\"",
//                 descriptor
//             );
//             let opposite = MaskFilter::from_str(opposite_desc);
//             assert!(
//                 opposite.is_ok(),
//                 "Unable to parse MaskFilter from \"{}\"",
//                 opposite_desc
//             );
//             assert_eq!(!mask.unwrap(), opposite.unwrap(), "{}", "MaskFilter::Not()");
//         }

//         let mask = MaskFilter::from_str("=G08,  G09, R03").unwrap();
//         assert_eq!(
//             mask,
//             MaskFilter {
//                 operand: MaskOperand::Equals,
//                 item: FilterItem::SvItem(vec![
//                     SV::from_str("G08").unwrap(),
//                     SV::from_str("G09").unwrap(),
//                     SV::from_str("R03").unwrap(),
//                 ]),
//             }
//         );
//         let m2 = MaskFilter::from_str("G08,G09,R03").unwrap();
//         assert_eq!(mask, m2);

//         let mask = MaskFilter::from_str("!=G31").unwrap();
//         assert_eq!(
//             mask,
//             MaskFilter {
//                 operand: MaskOperand::NotEquals,
//                 item: FilterItem::SvItem(vec![SV::from_str("G31").unwrap(),]),
//             }
//         );
//         let m2 = MaskFilter::from_str("!=G31").unwrap();
//         assert_eq!(mask, m2);
//     }
//     #[test]
//     fn mask_complex() {
//         let mask = MaskFilter::from_str("=L1C,S1C,D1P,C1W").unwrap();
//         assert_eq!(
//             mask,
//             MaskFilter {
//                 operand: MaskOperand::Equals,
//                 item: FilterItem::ComplexItem(vec![
//                     "L1C".to_string(),
//                     "S1C".to_string(),
//                     "D1P".to_string(),
//                     "C1W".to_string()
//                 ])
//             }
//         );
//     }
// }
