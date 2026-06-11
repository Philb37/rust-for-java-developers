use sea_orm::entity::prelude::*;

use crate::{models::{priority::Priority, ticket_status::TicketStatus}, schemas::create_ticket_request::CreateTicketRequest};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tickets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub description: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub status: TicketStatus,
    #[sea_orm(column_type = "Text")]
    pub priority: Priority,
    #[sea_orm(column_type = "Text")]
    pub assignee: Option<String>,
    pub created_at: TimeDateTimeWithTimeZone
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {

    // Runs before INSERT and UPDATE
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // One of the equivalent to @PrePersist
        if insert {

            self.created_at = sea_orm::ActiveValue::Set(time::OffsetDateTime::now_utc());

            if self.status.is_not_set() {
                self.status = sea_orm::ActiveValue::Set(TicketStatus::Open);
            }
        }

        Ok(self)
    }
}

impl From<CreateTicketRequest> for ActiveModel {
    fn from(request: CreateTicketRequest) -> Self {
        Self {
            title: sea_orm::ActiveValue::Set(request.title),
            description: sea_orm::ActiveValue::Set(request.description),
            priority: sea_orm::ActiveValue::Set(request.priority),
            assignee: sea_orm::ActiveValue::Set(request.assignee),
            ..Default::default()
        }
    }
}

// New type
// #[derive(Clone, Debug, PartialEq, Eq, DeriveValueType)]
// pub struct Integer(i32);