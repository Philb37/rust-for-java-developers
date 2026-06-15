use diesel::{
    Selectable,
    pg::Pg,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
};
use time::OffsetDateTime;

use crate::{
    models::{priority::Priority, ticket_status::TicketStatus},
    schemas::create_ticket_request::CreateTicketRequest,
};

#[derive(Queryable, Selectable, AsChangeset, Identifiable, Debug)]
#[diesel(table_name = crate::schema::tickets)]
#[diesel(check_for_backend(Pg))]
#[cfg_attr(test, derive(Clone))]
pub struct Ticket {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: TicketStatus,
    pub priority: Priority,
    pub assignee: Option<String>,
    pub created_at: OffsetDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::tickets)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct NewTicket {
    title: String,
    status: TicketStatus,
    priority: Priority,
    description: Option<String>,
    assignee: Option<String>,
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = crate::schema::tickets)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct TicketUpdate {
    status: TicketStatus,
}

impl From<CreateTicketRequest> for NewTicket {
    fn from(request: CreateTicketRequest) -> Self {
        Self {
            title: request.title,
            status: TicketStatus::Open,
            priority: request.priority,
            description: request.description,
            assignee: request.assignee,
        }
    }
}

impl From<TicketStatus> for TicketUpdate {
    fn from(status: TicketStatus) -> Self {
        Self {
            status
        }
    }
}