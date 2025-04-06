use crate::errors::QcPipelineError;

pub mod select;
pub use select::{QcSelection, QcSelectionStep, QcSelectionStepItem, QcSelectionStepOperand};

use crate::QcScope;

/// [QcPipeline] describes a processing pipeline
/// that may apply to the entire or focused on a specific scope.
/// See supported [QcSelectionStepItem]s that may be described in each step.
///
/// Basic [QcPipeline] description:
///
/// - Unscoped (entire set) operations: "decim:%2"
/// - Masking (discard anything but): "Select:Gal"
/// - Masking (discard this value): "Select:!Gal"
/// - Scoped operations: "Select:Gal,GPS:decim:%2"
/// - Scoped operations: "Select:!Gal:decim:%2"
///
/// ## Single Step [QcPipeline]
///
/// Item selection (masking) is implied, anytime the operand is omitted.
/// That means we discard everything but provided value(s).
///
/// - One item: "Select:Gal"
/// - Ues CSV to describe several items: "Select:Gal,GPS"
///
/// Conditional selection applies to SV, datetime, durations and angles:
///
/// - PRN# masking: "Select:>E01"
/// - Datetime masking: "Select:<=2020-01-01T00:00:00 UTC"
/// - Elevation masking: "Select:el>10"
/// - Elevation masking: "Select:el>10deg"
/// - Elevation masking: "Select:el>10 rad"
/// - Azimuth masking: "Select:az>10"
/// - Azimuth masking: "Select:az>10deg"
/// - Azimuth masking: "Select:az>10 rad"
///
/// ## [QcScope]d selection
///
/// Scoped selection, by prepending a scope definition.
/// See [QcScope] definitions for available scopes.
///
/// - Retain only Gal and GPS constellations from Observed products `Scope:obs:Select:Gal,GPS`
/// - Retain only Gal and GPS from MyAgency publish: `Scope:agency:MyAgency:Select:Gal,GPS`
pub struct QcPipeline {
    /// [QcScope]
    pub scope: QcScope,
    /// [QcSelection]
    pub select: QcSelection,
}

enum Token {
    Delimiter,
    Agency,
    Operator,
    File,
    Scope(QcScope),
    Step(QcSelectionStep),
}

struct Parser;

impl Parser {
    fn tokenize(s: &str) -> Vec<Token> {
        let trimmed = s.trim();
        let mut buffer = String::new();
        let mut token = Option::<Token>::None;
        let mut tokens = Vec::with_capacity(4);

        for c in trimmed.chars() {
            buffer.push(c);

            match c {
                ':' => {
                    if let Some(token) = &token {
                    } else {
                    }
                }
                _ => {}
            }
        }

        tokens
    }
}

impl std::str::FromStr for QcPipeline {
    type Err = QcPipelineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = Parser::tokenize(s);

        Err(QcPipelineError::EmptyPipeline)
    }
}

// #[cfg(test)]
// mod test {
//     use std::str::FromStr;
//     use super::QcPipeline;
//     use crate::QcScope;

//     #[test]
//     fn single_ops_pipeline_parsing() {
//         let content = "decim:%10";

//         let pipeline = QcPipeline::from_str(content).unwrap();
//         assert_eq!(pipeline.scope, QcScope::All);
//     }
// }
