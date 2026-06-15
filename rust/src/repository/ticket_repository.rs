use crate::{
    models::{
        priority::Priority,
        ticket::{NewTicket, Ticket, TicketUpdate},
        ticket_status::TicketStatus,
    }, schemas::app_error::AppError
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait TicketRepository: Send + Sync {
    async fn save(&self, ticket: NewTicket) -> Result<Ticket, AppError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Ticket>, AppError>;
    async fn list(
        &self,
        status: Option<TicketStatus>,
        priority: Option<Priority>,
    ) -> Result<Vec<Ticket>, AppError>;
    async fn update_status(&self, id: i32, ticket_update: TicketUpdate) -> Result<Option<Ticket>, AppError>;
}
