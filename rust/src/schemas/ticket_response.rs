use serde::Serialize;
use time::{OffsetDateTime, serde::rfc3339};

use crate::models::{priority::Priority, ticket_status::TicketStatus};

use crate::models::ticket::Model as Ticket;

#[derive(Serialize)]
pub struct TicketResponse {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: TicketStatus, // That's a leak of our domain model, but let's keep it simple for now
    pub priority: Priority, // That's a leak of our domain model, but let's keep it simple for now
    pub assignee: Option<String>,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
}

impl From<Ticket> for TicketResponse {
    
    fn from(ticket: Ticket) -> Self {
        
        Self {
            id: ticket.id,
            title: ticket.title,
            description: ticket.description,
            status: ticket.status,
            priority: ticket.priority,
            assignee: ticket.assignee,
            created_at: ticket.created_at
        }
    }
}