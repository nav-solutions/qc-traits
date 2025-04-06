use crate::filter::{
    QcDecimationFilter,
    QcMaskFilter,
};

pub enum QcOps {
    Masking(QcMask),
    Decimation(QcDecimation),
}