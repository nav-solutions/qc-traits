use crate::{errors::QcSelectionError, QcAngle};

pub mod item;
pub mod operand;

pub use item::QcSelectionStepItem;
pub use operand::QcSelectionStepOperand;

#[cfg(doc)]
use crate::{pipeline::QcPipeline, QcAngle};

/// [QcSelectionStep] to target specific items with a [QcPipeline]
///
/// - Select one constellation: `Select:Gal`
/// - Select several constellations: `Select:Gal,GPS`
/// - Specific [SV]: `Select:E10,E11,E12`
/// - PRN# selection for a [Constellation]: `Select:>E10`
///
/// - Select elevation condition: `Select:el>10 deg`.  
/// Any valid [QcAngle] description may apply
///  
/// - Select azimutal condition: `Select:az<=100 deg`.  
/// Any valid [QcAngle] description may apply
#[derive(Debug, Clone, PartialEq)]
pub struct QcSelectionStep {
    pub item: QcSelectionStepItem,
    pub operand: QcSelectionStepOperand,
}

#[derive(Debug, PartialEq)]
enum Token {
    Azimuth,
    Elevation,
    Angle(QcAngle),
    Item(QcSelectionStepItem),
    Operand(QcSelectionStepOperand),
}

struct Parser;

impl Parser {
    fn tokenize(s: &str) -> Vec<Token> {
        let trimmed = s.trim();
        let mut buffer = String::with_capacity(8);
        let mut tokens = Vec::with_capacity(3);

        for c in trimmed.chars() {
            buffer.push(c);

            match c {
                'z' => {
                    if buffer.contains("az") {
                        buffer.clear();
                        tokens.push(Token::Azimuth);
                    }
                }
                'l' => {
                    if buffer.contains("el") {
                        buffer.clear();
                        tokens.push(Token::Elevation);
                    }
                }
                _ => {
                    if buffer.len() > 1 {
                        if let Ok(parsed) = buffer.parse::<QcSelectionStepOperand>() {
                            buffer.remove(0);
                            buffer.remove(0);
                            tokens.push(Token::Operand(parsed));
                        } else if let Ok(parsed) = buffer[..1].parse::<QcSelectionStepOperand>() {
                            buffer.remove(0);
                            tokens.push(Token::Operand(parsed));
                        }
                    }
                }
            }
        }

        if !buffer.is_empty() {
            if let Ok(angle) = buffer.trim().parse::<QcAngle>() {
                tokens.push(Token::Angle(angle))
            } else if let Ok(parsed) = buffer.parse::<QcSelectionStepItem>() {
                tokens.push(Token::Item(parsed));
            }
        }

        tokens
    }
}

impl std::fmt::Display for QcSelectionStep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.operand == QcSelectionStepOperand::Equals {
            write!(f, "{}", self.item)
        } else {
            write!(f, "{}{}", self.operand, self.item)
        }
    }
}

impl std::str::FromStr for QcSelectionStep {
    type Err = QcSelectionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = Parser::tokenize(s.trim());

        println!("TokenStream: {:?}", tokens);

        let operand = tokens
            .iter()
            .filter_map(|token| match token {
                Token::Operand(operand) => Some(operand),
                _ => None,
            })
            .reduce(|k, _| k);

        if tokens.contains(&Token::Azimuth) {
            // angle value required
            let angle = tokens
                .iter()
                .filter_map(|token| match token {
                    Token::Angle(angle) => Some(angle),
                    _ => None,
                })
                .reduce(|k, _| k)
                .ok_or(QcSelectionError::InvalidItem)?;

            if let Some(operand) = operand {
                Ok(QcSelectionStep {
                    operand: *operand,
                    item: QcSelectionStepItem::Azimuth(*angle),
                })
            } else {
                Ok(QcSelectionStep {
                    operand: QcSelectionStepOperand::default(),
                    item: QcSelectionStepItem::Azimuth(*angle),
                })
            }
        } else if tokens.contains(&Token::Elevation) {
            // angle value required
            let angle = tokens
                .iter()
                .filter_map(|token| match token {
                    Token::Angle(angle) => Some(angle),
                    _ => None,
                })
                .reduce(|k, _| k)
                .ok_or(QcSelectionError::InvalidItem)?;

            if let Some(operand) = operand {
                Ok(QcSelectionStep {
                    operand: *operand,
                    item: QcSelectionStepItem::Elevation(*angle),
                })
            } else {
                Ok(QcSelectionStep {
                    operand: QcSelectionStepOperand::default(),
                    item: QcSelectionStepItem::Elevation(*angle),
                })
            }
        } else {
            // valid item is required
            let item = tokens
                .iter()
                .filter_map(|token| match token {
                    Token::Item(item) => Some(item),
                    _ => None,
                })
                .reduce(|k, _| k)
                .ok_or(QcSelectionError::InvalidItem)?;

            if let Some(operand) = operand {
                Ok(QcSelectionStep {
                    item: item.clone(),
                    operand: *operand,
                })
            } else {
                Ok(QcSelectionStep {
                    item: item.clone(),
                    operand: QcSelectionStepOperand::default(),
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{QcSelectionStep, QcSelectionStepItem, QcSelectionStepOperand};
    use crate::prelude::{Constellation, Duration, SV};
    use std::str::FromStr;

    #[test]
    fn pipeline_selection_step_parsing() {
        const GPS: Constellation = Constellation::GPS;
        let g01 = SV::new(GPS, 01);

        const GAL: Constellation = Constellation::Galileo;
        let e07 = SV::new(GAL, 07);
        let e10 = SV::new(GAL, 10);

        let dt_30s = Duration::from_seconds(30.0);

        for (pipeline_str, expected) in [
            (
                "30 s",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_duration(dt_30s),
                    operand: QcSelectionStepOperand::Equals,
                },
            ),
            (
                ">E10",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_satellite(e10),
                    operand: QcSelectionStepOperand::GreaterThan,
                },
            ),
            (
                ">=E07",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_satellite(e07),
                    operand: QcSelectionStepOperand::GreaterEquals,
                },
            ),
            (
                ">=E07 ",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_satellite(e07),
                    operand: QcSelectionStepOperand::GreaterEquals,
                },
            ),
            (
                " >=E07 ",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_satellite(e07),
                    operand: QcSelectionStepOperand::GreaterEquals,
                },
            ),
            (
                "GPS",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_constellation(GPS),
                    operand: QcSelectionStepOperand::Equals,
                },
            ),
            (
                " GPS",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_constellation(GPS),
                    operand: QcSelectionStepOperand::Equals,
                },
            ),
            (
                " GPS ",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_constellation(GPS),
                    operand: QcSelectionStepOperand::Equals,
                },
            ),
            (
                "E07",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_satellite(e07),
                    operand: QcSelectionStepOperand::Equals,
                },
            ),
            (
                "E07,G01",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_satellites(&[e07, g01]),
                    operand: QcSelectionStepOperand::Equals,
                },
            ),
            (
                "el>10",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_elevation_deg(10.0),
                    operand: QcSelectionStepOperand::GreaterThan,
                },
            ),
            (
                "az>30.2rad",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_azimuth_rad(30.2),
                    operand: QcSelectionStepOperand::GreaterThan,
                },
            ),
            (
                " az>30.2rad",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_azimuth_rad(30.2),
                    operand: QcSelectionStepOperand::GreaterThan,
                },
            ),
            (
                " az>30.2rad ",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_azimuth_rad(30.2),
                    operand: QcSelectionStepOperand::GreaterThan,
                },
            ),
            (
                " az>30.2 rad ",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_azimuth_rad(30.2),
                    operand: QcSelectionStepOperand::GreaterThan,
                },
            ),
            (
                " el<=12.2 rad ",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_elevation_rad(12.2),
                    operand: QcSelectionStepOperand::LowerEquals,
                },
            ),
            (
                " el<=12.2 deg ",
                QcSelectionStep {
                    item: QcSelectionStepItem::from_elevation_deg(12.2),
                    operand: QcSelectionStepOperand::LowerEquals,
                },
            ),
        ] {
            let pipeline = QcSelectionStep::from_str(pipeline_str).unwrap_or_else(|e| {
                panic!(
                    "Failed to parse processing pipeline step: \"{}\": {}",
                    pipeline_str, e
                )
            });

            assert_eq!(
                pipeline, expected,
                "invalid value parsed from \"{}\"",
                pipeline_str
            );
        }
    }
}
