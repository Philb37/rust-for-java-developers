use serde::{Deserialize};
use validator::{Validate, ValidationError};

use crate::models::{priority::Priority};

#[derive(Deserialize, Validate)]
pub struct CreateTicketRequest {
    #[validate(custom(function = "not_blank"))]
    pub title: String,
    pub priority: Priority, // That's a leak of our domain model, but let's keep it simple for now
    pub description: Option<String>,
    pub assignee: Option<String>,
}

fn not_blank(value: &str) -> Result<(), ValidationError> {
    match value.trim().is_empty() {
        true => Err(ValidationError::new("not_blank").with_message("Title must not be blank.".into())),
        false => Ok(()),
    }
}