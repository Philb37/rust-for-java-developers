use std::collections::HashMap;
use std::sync::Arc;

use crate::repository::ticket_repository::TicketRepository;

use crate::models::priority::Priority;
use crate::models::ticket_status::TicketStatus;
use crate::schemas::app_error::AppError;
use crate::schemas::stats_response::StatsResponse;
use crate::schemas::{create_ticket_request::CreateTicketRequest, ticket_response::TicketResponse};

#[derive(Clone)]
pub struct TicketService {
    ticket_repository: Arc<dyn TicketRepository>,
}

impl TicketService {
    pub fn new(ticket_repository: Arc<dyn TicketRepository>) -> Self {
        Self { ticket_repository }
    }

    pub async fn create(&self, request: CreateTicketRequest) -> Result<TicketResponse, AppError> {
        let created_ticket = self.ticket_repository.save(request.into()).await?;

        Ok(TicketResponse::from(created_ticket))
    }

    pub async fn get_by_id(&self, id: i32) -> Result<TicketResponse, AppError> {
        let ticket = self
            .ticket_repository
            .find_by_id(id)
            .await?
            .ok_or(AppError::TicketNotFound(id))?;

        Ok(TicketResponse::from(ticket))
    }

    pub async fn list(
        &self,
        status: Option<TicketStatus>,
        priority: Option<Priority>,
    ) -> Result<Vec<TicketResponse>, AppError> {
        let tickets = self.ticket_repository.list(status, priority).await?;

        // TALK : into is not the right one here we need into_iter to take ownership
        Ok(tickets.into_iter().map(TicketResponse::from).collect())
    }

    pub async fn update_status(
        &self,
        id: i32,
        status: TicketStatus,
    ) -> Result<TicketResponse, AppError> {
        let ticket = self
            .ticket_repository
            .update_status(id, status.into())
            .await?
            .ok_or(AppError::TicketNotFound(id))?;

        Ok(TicketResponse::from(ticket))
    }

    // TALK : Doing this in memory is bad practice, it should be done in SQL directly.
    // The goal is to showcase different ways to do something similar to the springboot project.
    pub async fn stats(&self) -> Result<StatsResponse, AppError> {
        let tickets = self.ticket_repository.list(None, None).await?;

        let (mut by_status, mut by_priority) = (HashMap::new(), HashMap::new());

        for ticket in &tickets {
            *by_status.entry(ticket.status).or_insert(0) += 1;
            *by_priority.entry(ticket.priority).or_insert(0) += 1;
        }

        // TALK : Alternative to test
        // let (by_status, by_priority) = tickets.iter()
        //     .fold((HashMap::new(), HashMap::new()), |mut acc, ticket| {
        //         *acc.0.entry(ticket.status).or_insert(0) += 1;
        //         *acc.1.entry(ticket.priority).or_insert(0) += 1;
        //         acc
        //     });

        Ok(StatsResponse {
            by_status,
            by_priority,
        })
    }
}

#[cfg(test)]
mod tests {

    use mockall::predicate::eq;

    use time::macros::datetime;

    use crate::{
        models::ticket::{NewTicket, Ticket, TicketUpdate},
        repository::ticket_repository::MockTicketRepository,
    };

    use super::*;

    fn init_test() {
        let _ = tracing_subscriber::fmt().with_test_writer().try_init();
    }

    #[tokio::test]
    async fn create_ok() {
        init_test();

        let title = "Printer is on fire".to_string();
        let same_title = title.clone();

        let create_ticket_request = CreateTicketRequest {
            title: same_title.clone(),
            priority: Priority::High,
            description: None,
            assignee: None,
        };

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_save()
            .with(eq::<NewTicket>(create_ticket_request.clone().into()))
            .returning(move |_| {
                Ok(Ticket {
                    id: 1,
                    title: title.clone(),
                    description: None,
                    status: TicketStatus::Open,
                    priority: Priority::High,
                    assignee: None,
                    created_at: datetime!(2026-01-01 00:00 UTC),
                })
            });

        let ticket_service = TicketService::new(Arc::new(mock_repo));

        let result = ticket_service.create(create_ticket_request).await;

        assert!(result.is_ok());

        let ticket_response = result.unwrap();

        assert_eq!(ticket_response.title, same_title);
        assert_eq!(ticket_response.priority, Priority::High);
        assert_eq!(ticket_response.status, TicketStatus::Open);
    }

    #[tokio::test]
    async fn create_err() {
        init_test();

        let title = "Printer is on fire".to_string();

        let create_ticket_request = CreateTicketRequest {
            title: title.clone(),
            priority: Priority::High,
            description: None,
            assignee: None,
        };

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_save()
            .with(eq::<NewTicket>(create_ticket_request.clone().into()))
            .returning(|_| {
                Err(AppError::QueryError(
                    diesel::result::Error::AlreadyInTransaction,
                ))
            });

        let ticket_service = TicketService::new(Arc::new(mock_repo));

        let result = ticket_service.create(create_ticket_request).await;

        assert!(matches!(result, Err(AppError::QueryError(_))));
    }

    #[tokio::test]
    async fn get_by_id_ok() {
        init_test();

        let title = "Printer is on fire".to_string();
        let same_title = title.clone();
        let id = 1;

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(id))
            .returning(move |_| {
                Ok(Some(Ticket {
                    id,
                    title: title.clone(),
                    description: None,
                    status: TicketStatus::Open,
                    priority: Priority::High,
                    assignee: None,
                    created_at: datetime!(2026-01-01 00:00 UTC),
                }))
            });

        let ticket_service = TicketService::new(Arc::new(mock_repo));

        let result = ticket_service.get_by_id(id).await;

        assert!(result.is_ok());

        let ticket_response = result.unwrap();

        assert_eq!(ticket_response.id, id);
        assert_eq!(ticket_response.title, same_title);
        assert_eq!(ticket_response.priority, Priority::High);
        assert_eq!(ticket_response.status, TicketStatus::Open);
    }

    #[tokio::test]
    async fn get_by_id_err() {
        init_test();

        let id = 1;

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(id))
            .returning(move |_| Ok(None));

        let ticket_service = TicketService::new(Arc::new(mock_repo));

        let result = ticket_service.get_by_id(id).await;

        // `TicketNotFound(id)` would BIND a new `id`, not compare with ours — use a guard
        assert!(
            matches!(result, Err(AppError::TicketNotFound(not_found_id)) if not_found_id == id)
        );
    }

    #[tokio::test]
    async fn list_ok() {
        init_test();

        let title = "Printer is on fire".to_string();
        let second_title = "Printer is on fire (again)".to_string();

        let tickets = vec![
            Ticket {
                id: 1,
                title: title.clone(),
                description: None,
                status: TicketStatus::Open,
                priority: Priority::High,
                assignee: None,
                created_at: datetime!(2026-01-01 00:00 UTC),
            },
            Ticket {
                id: 2,
                title: second_title.clone(),
                description: Some("This time the flames are blue".to_string()),
                status: TicketStatus::Open,
                priority: Priority::High,
                assignee: Some("Firefighters".to_string()),
                created_at: datetime!(2026-01-02 00:00 UTC),
            },
        ];

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_list()
            .with(eq(Some(TicketStatus::Open)), eq(Some(Priority::High)))
            .returning(move |_, _| Ok(tickets.clone()));

        let ticket_service = TicketService::new(Arc::new(mock_repo));

        let result = ticket_service
            .list(Some(TicketStatus::Open), Some(Priority::High))
            .await;

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
    }

    #[tokio::test]
    async fn list_ok_empty() {
        init_test();

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_list()
            .with(eq(None), eq(None))
            .returning(move |_, _| Ok(vec![]));

        let ticket_service = TicketService::new(Arc::new(mock_repo));

        let result = ticket_service.list(None, None).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn list_err() {
        init_test();

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_list()
            .with(eq(None), eq(None))
            .returning(move |_, _| {
                Err(AppError::QueryError(
                    diesel::result::Error::AlreadyInTransaction,
                ))
            });

        let ticket_service = TicketService::new(Arc::new(mock_repo));

        let result = ticket_service.list(None, None).await;

        assert!(matches!(result, Err(AppError::QueryError(_))));
    }

    #[tokio::test]
    async fn update_status_ok() {
        init_test();

        let title = "Printer is on fire".to_string();
        let same_title = title.clone();

        let id = 1;

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_update_status()
            .with(eq(id), eq::<TicketUpdate>(TicketStatus::Closed.into()))
            .returning(move |_, _| {
                Ok(Some(Ticket {
                    id: 1,
                    title: title.clone(),
                    description: None,
                    status: TicketStatus::Closed,
                    priority: Priority::High,
                    assignee: None,
                    created_at: datetime!(2026-01-01 00:00 UTC),
                }))
            });

        let ticket_service = TicketService::new(Arc::new(mock_repo));

        let result = ticket_service.update_status(id, TicketStatus::Closed).await;

        assert!(result.is_ok());

        let ticket_response = result.unwrap();

        assert_eq!(ticket_response.id, id);
        assert_eq!(ticket_response.title, same_title);
        assert_eq!(ticket_response.priority, Priority::High);
        assert_eq!(ticket_response.status, TicketStatus::Closed);
    }

    #[tokio::test]
    async fn update_status_err() {
        init_test();

        let id = 1;

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_update_status()
            .with(eq(id), eq::<TicketUpdate>(TicketStatus::Closed.into()))
            .returning(move |_, _| Ok(None));

        let ticket_service = TicketService::new(Arc::new(mock_repo));

        let result = ticket_service.update_status(id, TicketStatus::Closed).await;

        assert!(
            matches!(result, Err(AppError::TicketNotFound(not_found_id)) if not_found_id == id)
        );
    }

    #[tokio::test]
    async fn stats_ok() {
        init_test();

        let title = "Printer is on fire".to_string();

        let tickets = vec![Ticket {
            id: 1,
            title: title.clone(),
            description: None,
            status: TicketStatus::Open,
            priority: Priority::High,
            assignee: None,
            created_at: datetime!(2026-01-01 00:00 UTC),
        }];

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_list()
            .with(eq(None), eq(None))
            .returning(move |_, _| Ok(tickets.clone()));

        let ticket_service = TicketService::new(Arc::new(mock_repo));

        let result = ticket_service.stats().await;

        assert!(result.is_ok());

        let stats_response = result.unwrap();

        assert!(stats_response.by_priority.contains_key(&Priority::High));
        assert_eq!(stats_response.by_priority.get(&Priority::High).unwrap(), &1);
        assert!(stats_response.by_status.contains_key(&TicketStatus::Open));
        assert_eq!(
            stats_response.by_status.get(&TicketStatus::Open).unwrap(),
            &1
        );
    }

    #[tokio::test]
    async fn stats_err() {
        init_test();

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_list()
            .with(eq(None), eq(None))
            .returning(move |_, _| {
                Err(AppError::QueryError(
                    diesel::result::Error::AlreadyInTransaction,
                ))
            });

        let ticket_service = TicketService::new(Arc::new(mock_repo));

        let result = ticket_service.stats().await;

        assert!(matches!(result, Err(AppError::QueryError(_))));
    }
}
