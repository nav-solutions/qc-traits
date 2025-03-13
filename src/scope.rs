use crate::QcProductType;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum QcScope {
    /// All (non restrictive scope)
    #[default]
    All,
    /// Scope by [ProductType] (file type)
    ProductType(QcProductType),
    /// Scope by file name
    FileName(String),
    /// Scope by Agency
    Agency(String),
    /// Scope by Operator / Observer
    Operator(String),
}
