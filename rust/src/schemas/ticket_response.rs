use time::UtcDateTime;

use crate::models::priority::Priority;

pub struct TicketResponse {
    id: u64,
    title: String,
    description: Option<String>,
    status: TicketStatus, // That's a leak of our domain model, but let's keep it simple for now
    priority: Priority, // That's a leak of our domain model, but let's keep it simple for now
    assignee: Option<String>,
    created_at: UtcDateTime,
}