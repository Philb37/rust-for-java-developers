use serde::Deserialize;
use validator::Validate;

use crate::models::ticket_status::TicketStatus;

#[derive(Deserialize, Validate)]
pub struct UpdateStatusRequest {
    pub status: TicketStatus
}