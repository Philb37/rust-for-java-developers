use std::collections::HashMap;

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryTrait};

use crate::models::priority::Priority;
use crate::models::ticket_status::TicketStatus;
use crate::schemas::app_error::AppError;
use crate::schemas::stats_response::StatsResponse;
use crate::schemas::{create_ticket_request::CreateTicketRequest, ticket_response::TicketResponse};
use crate::models::ticket::{self, ActiveModel as Ticket};
use crate::models::ticket::Entity as TicketEntity;

#[derive(Clone)]
pub struct TicketService {
    database: DatabaseConnection
}

impl TicketService {

    pub fn new(database: DatabaseConnection) -> Self {
        Self {
            database
        }
    }

    pub async fn create(&self, request: CreateTicketRequest) -> Result<TicketResponse, AppError> {

        let ticket = Ticket::from(request)
            .insert(&self.database)
            .await?;

        Ok(TicketResponse::from(ticket))
    }

    pub async fn get_by_id(&self, id: i32) -> Result<TicketResponse, AppError> {
        TicketEntity::find_by_id(id)
            .one(&self.database)
            .await?
            .map_or(Err(AppError::TicketNotFound(id)), |t| Ok(TicketResponse::from(t)))
    }

    pub async fn list(&self, status: Option<TicketStatus>, priority: Option<Priority>) -> Result<Vec<TicketResponse>, AppError> {
        
        Ok(
            TicketEntity::find()
            .apply_if(
                status, 
                |query, status| query.filter(ticket::Column::Status.eq(status))
            )
            .apply_if(
                priority, 
                |query, priority| query.filter(ticket::Column::Priority.eq(priority))
            )
            .all(&self.database)
            .await?
            // CONF : Why not into() ?
            .into_iter()
            .map(TicketResponse::from)
            .collect()
        )
    }

    pub async fn update_status(&self, id: i32, status: TicketStatus) -> Result<TicketResponse, AppError> {

        // CONF : control flow
        let Some(ticket) = TicketEntity::find_by_id(id)
            .one(&self.database)
            .await? else {
            return Err(AppError::TicketNotFound(id));
        };

        // CONF : Cast
        let mut ticket: ticket::ActiveModel = ticket.into();

        ticket.status = sea_orm::ActiveValue::Set(status);

        let ticket = ticket.update(&self.database).await?;

        Ok(TicketResponse::from(ticket))
    }

    // CONF : Doing this in memory is bad practice, it should be done in SQL directly.
    // The goal is to showcase different ways to do something similar to the springboot project.
    pub async fn stats(&self) -> Result<StatsResponse, AppError> {

        let tickets = TicketEntity::find()
            .all(&self.database)
            .await?;

        let (mut by_status, mut by_priority) = (HashMap::new(), HashMap::new());

        for ticket in &tickets {
            *by_status.entry(ticket.status).or_insert(0) += 1;
            *by_priority.entry(ticket.priority).or_insert(0) += 1;
        }

        // CONF : Alternative to test
        // let (by_status, by_priority) = tickets.iter()
        //     .fold((HashMap::new(), HashMap::new()), |mut acc, ticket| {
        //         *acc.0.entry(ticket.status).or_insert(0) += 1;
        //         *acc.1.entry(ticket.priority).or_insert(0) += 1;
        //         acc
        //     });

        
        Ok(
            StatsResponse {
                by_status,
                by_priority
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::{DatabaseBackend, DbErr, MockDatabase, Transaction};
    use time::macros::datetime;

    use super::*;
    
    fn init_test() {
        let _ = tracing_subscriber::fmt().with_test_writer().try_init();

    }

    #[tokio::test]
    async fn create_ok() {

        init_test();

        let title = "Printer is on fire".to_string();

        let database = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([vec![ticket::Model {
                id: 1,
                title: title.clone(),
                description: None,
                status: TicketStatus::Open,
                priority: Priority::High,
                assignee: None,
                created_at: datetime!(2026-01-01 00:00 UTC),
            }]])
            .into_connection();

        let ticket_service = TicketService::new(database);

        let create_ticket_request = CreateTicketRequest {
            title: title.clone(),
            priority: Priority::High,
            description: None,
            assignee: None
        };

        let result = ticket_service.create(create_ticket_request).await;

        assert!(result.is_ok());

        let ticket_response = result.unwrap();

        assert_eq!(ticket_response.title, title);
        assert_eq!(ticket_response.priority, Priority::High);
        assert_eq!(ticket_response.status, TicketStatus::Open);
    }

    #[tokio::test]
    async fn create_err() {

        init_test();

        let title = "Printer is on fire".to_string();

        let database = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results::<ticket::Model, _, _>([vec![]])
            .into_connection();

        let ticket_service = TicketService::new(database);

        let create_ticket_request = CreateTicketRequest {
            title: title.clone(),
            priority: Priority::High,
            description: None,
            assignee: None
        };

        let result = ticket_service.create(create_ticket_request).await;

        assert!(matches!(result, Err(AppError::DatabaseError(_))));
    }

    #[tokio::test]
    async fn get_by_id_ok() {

        init_test();

        let title = "Printer is on fire".to_string();

        let database = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([vec![ticket::Model {
                id: 1,
                title: title.clone(),
                description: None,
                status: TicketStatus::Open,
                priority: Priority::High,
                assignee: None,
                created_at: datetime!(2026-01-01 00:00 UTC),
            }]])
            .into_connection();

        let ticket_service = TicketService::new(database);

        let id = 1;

        let result = ticket_service.get_by_id(id).await;

        assert!(result.is_ok());

        let ticket_response = result.unwrap();

        assert_eq!(ticket_response.id, id);
        assert_eq!(ticket_response.title, title);
        assert_eq!(ticket_response.priority, Priority::High);
        assert_eq!(ticket_response.status, TicketStatus::Open);
    }

    #[tokio::test]
    async fn get_by_id_err() {

        init_test();

        let database = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results::<ticket::Model, _, _>([vec![]])
            .into_connection();

        let ticket_service = TicketService::new(database);

        let id = 1;

        let result = ticket_service.get_by_id(id).await;

        // `TicketNotFound(id)` would BIND a new `id`, not compare with ours — use a guard
        assert!(matches!(result, Err(AppError::TicketNotFound(not_found_id)) if not_found_id == id));
    }

    #[tokio::test]
    async fn list_ok() {

        init_test();

        let title = "Printer is on fire".to_string();
        let second_title = "Printer is on fire (again)".to_string();

        let database = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([vec![ticket::Model {
                id: 1,
                title: title.clone(),
                description: None,
                status: TicketStatus::Open,
                priority: Priority::High,
                assignee: None,
                created_at: datetime!(2026-01-01 00:00 UTC),
            },
            ticket::Model {
                id: 2,
                title: "Printer is on fire (again)".to_string(),
                description: Some("This time the flames are blue".to_string()),
                status: TicketStatus::Open,
                priority: Priority::High,
                assignee: Some("Firefighters".to_string()),
                created_at: datetime!(2026-01-02 00:00 UTC),
            }]])
            .into_connection();

        // Keep a handle on the connection: a clone shares the same mock internals,
        // so we can read the transaction log after the service used it
        let ticket_service = TicketService::new(database.clone());

        let result = ticket_service.list(Some(TicketStatus::Open), Some(Priority::High)).await;

        assert!(result.is_ok());

        let ticket_responses = result.unwrap();

        assert_eq!(ticket_responses[0].id, 1);
        assert_eq!(ticket_responses[0].title, title);
        assert_eq!(ticket_responses[0].priority, Priority::High);
        assert_eq!(ticket_responses[0].status, TicketStatus::Open);

        assert_eq!(ticket_responses[1].id, 2);
        assert_eq!(ticket_responses[1].title, second_title);
        assert_eq!(ticket_responses[1].priority, Priority::High);
        assert_eq!(ticket_responses[1].status, TicketStatus::Open);

        assert_eq!(
            database.into_transaction_log(),
            [
                Transaction::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"SELECT "tickets"."id", "tickets"."title", "tickets"."description", "tickets"."status", "tickets"."priority", "tickets"."assignee", "tickets"."created_at" FROM "tickets" WHERE "tickets"."status" = $1 AND "tickets"."priority" = $2"#,
                    [TicketStatus::Open.into(), Priority::High.into()]
                )
            ]
        )
    }

    #[tokio::test]
    async fn list_ok_empty() {

        init_test();

        let database = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results::<ticket::Model, _, _>([vec![]])
            .into_connection();

        let ticket_service = TicketService::new(database);

        let result = ticket_service.list(None, None).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn list_err() {

        init_test();

        let database = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_errors([DbErr::Custom("the database is on fire".to_owned())])
            .into_connection();

        let ticket_service = TicketService::new(database);

        let result = ticket_service.list(None, None).await;

        assert!(matches!(result, Err(AppError::DatabaseError(_))));
    }

    #[tokio::test]
    async fn update_status_ok() {

        init_test();

        let title = "Printer is on fire".to_string();
        
        let id = 1;

        let database = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([vec![ticket::Model {
                id,
                title: title.clone(),
                description: None,
                status: TicketStatus::Open,
                priority: Priority::High,
                assignee: None,
                created_at: datetime!(2026-01-01 00:00 UTC),
            }],
            vec![ticket::Model {
                id,
                title: title.clone(),
                description: None,
                status: TicketStatus::Closed,
                priority: Priority::High,
                assignee: None,
                created_at: datetime!(2026-01-01 00:00 UTC),
            }]])
            .into_connection();

        let ticket_service = TicketService::new(database);

        let result = ticket_service.update_status(id, TicketStatus::Closed).await;

        assert!(result.is_ok());

        let ticket_response = result.unwrap();

        assert_eq!(ticket_response.id, id);
        assert_eq!(ticket_response.title, title);
        assert_eq!(ticket_response.priority, Priority::High);
        assert_eq!(ticket_response.status, TicketStatus::Closed);
    }

    #[tokio::test]
    async fn update_status_err() {

        init_test();
        
        let id = 1;

        let database = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results::<ticket::Model, _, _>([vec![]])
            .into_connection();

        let ticket_service = TicketService::new(database);

        let result = ticket_service.update_status(id, TicketStatus::Closed).await;

        assert!(matches!(result, Err(AppError::TicketNotFound(not_found_id)) if not_found_id == id));
    }

    #[tokio::test]
    async fn stats_ok() {

        init_test();

        let title = "Printer is on fire".to_string();
        
        let id = 1;

        let database = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([vec![ticket::Model {
                id,
                title: title.clone(),
                description: None,
                status: TicketStatus::Open,
                priority: Priority::High,
                assignee: None,
                created_at: datetime!(2026-01-01 00:00 UTC),
            }]])
            .into_connection();

        let ticket_service = TicketService::new(database);

        let result = ticket_service.stats().await;

        assert!(result.is_ok());

        let stats_response = result.unwrap();

        assert!(stats_response.by_priority.contains_key(&Priority::High));
        assert_eq!(stats_response.by_priority.get(&Priority::High).unwrap(), &1);
        assert!(stats_response.by_status.contains_key(&TicketStatus::Open));
        assert_eq!(stats_response.by_status.get(&TicketStatus::Open).unwrap(), &1);
    }

    #[tokio::test]
    async fn stats_err() {

        init_test();

        let database = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_errors([DbErr::Custom("the database is on fire".to_owned())])
            .into_connection();

        let ticket_service = TicketService::new(database);

        let result = ticket_service.stats().await;

        assert!(matches!(result, Err(AppError::DatabaseError(_))));
    }
}