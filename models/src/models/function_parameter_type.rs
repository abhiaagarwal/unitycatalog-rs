use crate::models;
use serde::{Deserialize, Serialize};

/// FunctionParameterType : The type of function parameter.
/// The type of function parameter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FunctionParameterType {
    #[serde(rename = "PARAM")]
    Param,
    #[serde(rename = "COLUMN")]
    Column,
}

impl std::fmt::Display for FunctionParameterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Param => write!(f, "PARAM"),
            Self::Column => write!(f, "COLUMN"),
        }
    }
}

impl Default for FunctionParameterType {
    fn default() -> FunctionParameterType {
        Self::Param
    }
}