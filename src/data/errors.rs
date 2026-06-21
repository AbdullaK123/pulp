use std::collections::HashMap;
use thiserror::Error;
use validator::{ValidationErrors};
use crate::infrastructure::errors::InfrastructureError;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Validation Error: {0}")]
    ValidationError(String)
}

impl From<ValidationErrors> for DataError {
    fn from(errors: ValidationErrors) -> Self {
        let field_errors: HashMap<String, Vec<String>> = errors
            .field_errors()
            .into_iter()
            .map(|(field, errs)| {
                let messages = errs.iter()
                    .map(|e| e.message.clone().unwrap_or_else(|| {
                        e.code.clone()
                    }).to_string())
                    .collect();
                (field.to_string(), messages)
            })
            .collect();
        DataError::ValidationError(serde_json::to_string(&field_errors).unwrap())
    }
}