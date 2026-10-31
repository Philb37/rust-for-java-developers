use std::collections::HashMap;

use crate::{
    models::{
        priority::Priority, ticket::{NewTicket, Ticket, TicketUpdate}, ticket_status::TicketStatus
    }, repository::database_error::DatabaseError,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait TicketRepository: Send + Sync {
    async fn save(&self, ticket: NewTicket) -> Result<Ticket, DatabaseError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Ticket>, DatabaseError>;
    async fn list(
        &self,
        status: Option<TicketStatus>,
        priority: Option<Priority>,
    ) -> Result<Vec<Ticket>, DatabaseError>;
    async fn stats(&self) -> Result<(HashMap<TicketStatus, i64>, HashMap<Priority, i64>), DatabaseError>;
    async fn update_status(&self, id: i32, ticket_update: TicketUpdate) -> Result<Option<Ticket>, DatabaseError>;
}
