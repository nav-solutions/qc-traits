pub mod select;

pub use select::{QcSelection, QcSelectionStep, QcSelectionStepItem, QcSelectionStepOperand};

use crate::QcScope;

/// [QcPipeline] describes a processing pipeline
/// that may apply to the entire scope or narrowed scoped operation.
///
/// Basic [QcPipeline] description:
///
/// - Unscoped (entire set): "decim:%2"
/// - Masking: "Select:Gal"
/// - Scoped operations: "Select:Gal,GPS:decim:%2"
///
/// ## [QcSelect]ion
///
/// Simple selection (item masking) to discard everything but.
/// See [QcSelect]ion items.
///
/// - One item: "Select:Gal"
/// - Several items: "Select:Gal,GPS"
///
/// Conditional selection applies to SV, datetime, durations and angles:
///
/// - PRN# masking: "Select:>E01"
/// - Datetime masking: "Select:<=2020-01-01T00:00:00 UTC"
/// -
///
/// ## [QcScope]d selection
///
/// Scoped selection, by prepending a scope definition.
/// See [QcScope] definitions for available scopes.
///
/// - "Scope:obs:Select:Gal,GPS"
pub struct QcPipeline {
    /// [QcScope]
    pub scope: QcScope,
    /// [QcSelection]
    pub select: QcSelection,
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
