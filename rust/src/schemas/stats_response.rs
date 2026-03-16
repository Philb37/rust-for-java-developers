use std::collections::HashMap;

use serde::Serialize;

use crate::models::{priority::Priority, ticket_status::TicketStatus};

#[derive(Serialize)]
pub struct StatsResponse {
    pub by_status: HashMap<TicketStatus, i32>,
    pub by_priority: HashMap<Priority, i32>,
}