#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Ticket {0} not found")]
    TicketNotFound(i32),
}