#![doc(html_logo_url = "https://raw.githubusercontent.com/rtk-rs/.github/master/logos/logo2.jpg")]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod errors;
mod field;
mod merge;
mod products;
mod repair;
mod rework;
mod scope;
mod split;
mod subset;

pub(crate) mod ops;

#[cfg(feature = "processing")]
#[cfg_attr(docsrs, doc(cfg(feature = "processing")))]
mod filter;

pub use crate::{
    errors::{QcError, QcSubsetError},
    field::{Error as QcFieldError, QcField},
    merge::{Error as QcMergeError, QcMerge},
    products::QcProductType,
    repair::QcRepair,
    rework::QcRework,
    scope::QcScope,
    split::QcSplit,
    subset::QcSubset,
};

#[cfg(feature = "processing")]
pub use processing::{
    Decimate, DecimationError, DecimationFilter, DecimationFilterType, Filter, FilterItem,
    MaskError, MaskFilter, MaskOperand, Masking, Preprocessing, Split,
};

#[cfg(feature = "html")]
pub use maud::{html, Markup};

/// HTML reporting
#[cfg(feature = "html")]
#[cfg_attr(docsrs, doc(cfg(feature = "html")))]
pub trait QcHtmlReporting {
    fn render(&self) -> Markup;
}
