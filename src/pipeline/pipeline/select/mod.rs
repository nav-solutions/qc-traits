pub mod step;

pub use step::{QcSelectionStep, QcSelectionStepItem, QcSelectionStepOperand};

/// [QcSelection] filter to apply a custom algorithm or filter
/// to a specific subset
#[derive(Debug, Clone, Default)]
pub struct QcSelection {
    steps: Vec<QcSelectionStep>,
}

impl QcSelection {
    /// Builds a new [QcSelection] made of a single [QcSelectionStep]
    pub fn new(step: QcSelectionStep) -> Self {
        Self { steps: vec![step] }
    }

    /// Adds a new [QcSelectionStep] forming a new [QcSelection]
    pub fn add_step(&mut self, step: QcSelectionStep) {
        self.steps.push(step);
    }

    /// Returns total number of [QcSelectionStep]s.
    pub fn size(&self) -> usize {
        self.steps.len()
    }
}

impl Iterator for QcSelection {
    type Item = QcSelectionStep;

    fn next(&mut self) -> Option<Self::Item> {
        self.steps.pop()
    }
}
