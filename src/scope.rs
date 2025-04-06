use crate::{errors::QcScopeError, QcProductType};

/// [QcScope] describes part of the dataset we may focus on.
///
/// ## Agency (publisher) description
///
/// - "agency:$agency"
/// - "ag:$agency"
///
/// ## Operator (person) description
///
/// - "operator:$operator"
/// - "op:$operator"
///
/// ## File name description
///
/// - "file:$name"
///
/// ## Product type description
///
/// Any valid [QcProductType] description applies.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum QcScope {
    /// All (non restrictive scope)
    #[default]
    All,
    /// Scope by [QcProductType] (file type)
    ProductType(QcProductType),
    /// Scope by file name
    FileName(String),
    /// Scope by Agency (publisher)
    Agency(String),
    /// Scope by Operator (person, employee).
    /// Refered to as "Observer" in RINEX terminology.
    Operator(String),
}

impl std::str::FromStr for QcScope {
    type Err = QcScopeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        let len = trimmed.len();

        if let Ok(product) = QcProductType::from_str(trimmed) {
            Ok(QcScope::ProductType(product))
        } else {
            if len > 3 && trimmed.starts_with("ag:") {
                let agency = trimmed[3..].to_string();
                Ok(QcScope::Agency(agency))
            } else if len > 7 && trimmed.starts_with("agency:") {
                let agency = trimmed[8..].to_string();
                Ok(QcScope::Agency(agency))
            } else if len > 9 && trimmed.starts_with("operator:") {
                let operator = trimmed[10..].to_string();
                Ok(QcScope::Operator(operator))
            } else if len > 3 && trimmed.starts_with("op:") {
                let operator = trimmed[3..].to_string();
                Ok(QcScope::Operator(operator))
            } else if len > 5 && trimmed.starts_with("file:") {
                let file = trimmed[6..].to_string();
                Ok(QcScope::FileName(file))
            } else {
                Err(QcScopeError::InvalidScope)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{QcProductType, QcScope};

    #[test]
    fn qc_scope_parsing() {
        for (value, expected) in [
            ("obs", QcScope::ProductType(QcProductType::Observation)),
            (
                "observation",
                QcScope::ProductType(QcProductType::Observation),
            ),
            (
                "nav",
                QcScope::ProductType(QcProductType::BroadcastNavigation),
            ),
            (
                "brdc",
                QcScope::ProductType(QcProductType::BroadcastNavigation),
            ),
            ("ag:LAB", QcScope::Agency("LAB".to_string())),
            ("agency:LAB", QcScope::Agency("LAB".to_string())),
            ("op:MySelf", QcScope::Operator("MySelf".to_string())),
            ("operator:MySelf", QcScope::Operator("MySelf".to_string())),
            ("file:Bleh", QcScope::FileName("Bleh".to_string())),
        ] {
            let scope = QcScope::from_str(value)
                .unwrap_or_else(|e| panic!("Failed to parse QcScope from \"{}\" - {}", value, e));

            assert_eq!(scope, expected);
        }
    }
}
