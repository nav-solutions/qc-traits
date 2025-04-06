#![doc(html_logo_url = "https://raw.githubusercontent.com/rtk-rs/.github/master/logos/logo2.jpg")]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod errors;
mod field;
mod filter;
mod merge;
mod products;
mod repair;
mod rework;
mod scope;
mod split;
mod angle;
mod pipeline;

pub use crate::{
    field::{Error as QcFieldError, QcField},
    filter::{QcDecimationFilter, QcFilter, QcFilterType, QcMaskOperand},
    merge::{QcMerge, QcMergeError},
    products::QcProductType,
    repair::QcRepair,
    rework::QcRework,
    scope::QcScope,
    split::QcSplit,
    angle::QcAngle,
    pipeline::QcPipeline,
};

/// The [QcPreprocessing] trait allows all preprocessing operations
/// that one may use at the input of a processing pipeline
pub trait QcPreprocessing {
    /// Apply a [QcFilter] with mutable access.
    fn filter_mut(&mut self, f: &QcFilter);

    /// Applies a [QcFilter] without mutable access,
    /// returns filtered result or simply a copy if this [QcFilter] does not apply.
    fn filter(&self, f: &QcFilter) -> Self
    where
        Self: Sized + Clone,
    {
        let mut s = self.clone();
        s.filter_mut(f);
        s
    }
}

#[cfg(feature = "html")]
pub use maud::{html, Markup};

/// HTML reporting
#[cfg(feature = "html")]
#[cfg_attr(docsrs, doc(cfg(feature = "html")))]
pub trait QcHtmlReporting {
    fn render(&self) -> Markup;
}
