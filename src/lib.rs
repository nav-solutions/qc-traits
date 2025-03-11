#![doc(html_logo_url = "https://raw.githubusercontent.com/rtk-rs/.github/master/logos/logo2.jpg")]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod field;
pub use field::QcField;

mod merge;
pub use merge::{Error as MergeError, Merge};

mod repair;
pub use repair::QcRepair;

mod rework;
pub use rework::QcRework;

#[cfg(feature = "processing")]
#[cfg_attr(docsrs, doc(cfg(feature = "processing")))]
mod processing;

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
