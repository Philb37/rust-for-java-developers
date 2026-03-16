use serde::Deserialize;

use crate::models::{priority::Priority, ticket_status::TicketStatus};

#[derive(Deserialize)]
pub struct RequestParams {
    pub status: Option<TicketStatus>,
    pub priority: Option<Priority>
}