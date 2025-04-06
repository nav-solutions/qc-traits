#![doc(html_logo_url = "https://raw.githubusercontent.com/rtk-rs/.github/master/logos/logo2.jpg")]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod angle;
mod errors;
mod field;
mod merge;
mod pipeline;
mod processing;
mod products;
mod repair;
mod rework;
mod scaling;
mod scope;
mod split;

pub use crate::{
    angle::QcAngle,
    field::{Error as QcFieldError, QcField},
    merge::{QcMerge, QcMergeError},
    pipeline::QcPipeline,
    processing::filter::QcDecimationFilter,
    products::QcProductType,
    repair::QcRepair,
    rework::QcRework,
    scope::QcScope,
    split::QcSplit,
};

pub mod prelude {
    pub use gnss_rs::prelude::{Constellation, SV};
    pub use hifitime::{Duration, Epoch, Unit};
}

pub trait QcPreprocessing: QcRepair {
    fn downsampling_mut(&mut self, filter: &QcPipeline<QcDecimationFilter>);

    fn downsampling(&self, filter: &QcPipeline<QcDecimationFilter>) -> Self
    where
        Self: Sized + Clone,
    {
        let mut s = self.clone();
        s.downsampling_mut(filter);
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
