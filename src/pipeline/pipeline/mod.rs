pub mod ops;
pub mod select;

use ops::QcOps;
use select::QcSelect;

use crate::{
    pipeline::parser::Parser,
    QcScope,
    errors::QcPipelineError,
};

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
    /// [QcSelect]
    pub select: QcSelect,
}

impl std::str::FromStr for QcPipeline {
    type Err = QcPipelineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::QcPipeline;
    use crate::QcScope;

    
    #[test] 
    fn single_ops_pipeline_parsing() {
        let content = "decim:%10";

        let pipeline = QcPipeline::from_str(content).unwrap();
        assert_eq!(pipeline.scope, QcScope::All);
    }
}