#![doc(html_logo_url = "https://raw.githubusercontent.com/rtk-rs/.github/master/logos/logo2.jpg")]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod angle;
mod errors;
mod field;
mod merge;
mod pipeline;
mod products;
mod repair;
mod rework;
mod scope;
mod split;

pub use crate::{
    angle::QcAngle,
    field::{Error as QcFieldError, QcField},
    merge::{QcMerge, QcMergeError},
    pipeline::QcPipeline,
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

// /// The [QcPreprocessing] trait allows all preprocessing operations
// /// that one may use at the input of a processing pipeline
// pub trait QcPreprocessing {
//     /// Apply a [QcFilter] with mutable access.
//     fn filter_mut(&mut self, f: &QcFilter);

//     /// Applies a [QcFilter] without mutable access,
//     /// returns filtered result or simply a copy if this [QcFilter] does not apply.
//     fn filter(&self, f: &QcFilter) -> Self
//     where
//         Self: Sized + Clone,
//     {
//         let mut s = self.clone();
//         s.filter_mut(f);
//         s
//     }
// }

#[cfg(feature = "html")]
pub use maud::{html, Markup};

/// HTML reporting
#[cfg(feature = "html")]
#[cfg_attr(docsrs, doc(cfg(feature = "html")))]
pub trait QcHtmlReporting {
    fn render(&self) -> Markup;
}
