#!/usr/bin/env bash

cd ./rust/src

# Appends the #[cfg(test)] modules to the end of the service and controller files.
# Run this once the implementations are in place (after snippet 28).

cat >> services/ticket_service.rs <<'TESTS_EOF'

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

use mockall::predicate::eq;

    use time::macros::datetime;

    use crate::{
        models::{
            ticket::{NewTicket, Ticket, TicketUpdate},
        }, repository::{database_error::DatabaseError, ticket_repository::MockTicketRepository},
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
                Err(DatabaseError::InvalidStatsRow)
            });

        let ticket_service = TicketService::new(Arc::new(mock_repo));

        let result = ticket_service.create(create_ticket_request).await;

        assert!(matches!(result, Err(AppError::Database(_))));
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
            matches!(result, Err(AppError::Domain(DomainError::TicketNotFound(not_found_id))) if not_found_id == id)
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
                Err(DatabaseError::InvalidStatsRow)
            });

        let ticket_service = TicketService::new(Arc::new(mock_repo));

        let result = ticket_service.list(None, None).await;

        assert!(matches!(result, Err(AppError::Database(_))));
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
            matches!(result, Err(AppError::Domain(DomainError::TicketNotFound(not_found_id))) if not_found_id == id)
        );
    }

    #[tokio::test]
    async fn stats_ok() {
        init_test();

        let (mut by_status, mut by_priority) = (HashMap::new(), HashMap::new());

        by_status.insert(TicketStatus::Open, 1);
        by_priority.insert(Priority::High, 1);

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_stats()
            .with()
            .returning(move || Ok((by_status.clone(), by_priority.clone())));

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
            .expect_stats()
            .with()
            .returning(|| {
                Err(DatabaseError::InvalidStatsRow)
            });

        let ticket_service = TicketService::new(Arc::new(mock_repo));

        let result = ticket_service.stats().await;

        assert!(matches!(result, Err(AppError::Database(_))));
    }
}

TESTS_EOF

cat >> controllers/ticket_controller.rs <<'TESTS_EOF'

#[cfg(test)]
mod tests {

    use std::{collections::HashMap, sync::Arc};

    use axum::{body::Body, extract::Request, http::StatusCode};
    use http_body_util::BodyExt;
    use mockall::predicate::eq;
    use serde::Deserialize;
    use time::macros::datetime;
    use tower::ServiceExt;

    use crate::{
        app, models::{
            priority::Priority, ticket::{NewTicket, Ticket, TicketUpdate}, ticket_status::TicketStatus
        }, repository::{database_error::DatabaseError, ticket_repository::MockTicketRepository}, services::ticket_service::TicketService,
    };

    use super::*;

    fn init_test() {
        let _ = tracing_subscriber::fmt().with_test_writer().try_init();
    }

    #[tokio::test]
    async fn create_returns_201() {
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
                    title: "Printer is on fire".to_string(),
                    description: None,
                    status: TicketStatus::Open,
                    priority: Priority::High,
                    assignee: None,
                    created_at: datetime!(2026-01-01 00:00 UTC),
                })
            });

        let app = app(AppState {
            ticket_service: TicketService::new(Arc::new(mock_repo)),
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/tickets")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"title": "Printer is on fire", "priority": "HIGH"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(
            body,
            serde_json::json!(
                {
                    "id": 1,
                    "title": "Printer is on fire",
                    "description": null,
                    "status": "OPEN",
                    "priority": "HIGH",
                    "assignee": null,
                    "created_at": "2026-01-01T00:00:00Z",
                }
            )
        );
    }

    #[tokio::test]
    async fn create_returns_422() {
        init_test();

        let mock_repo = MockTicketRepository::new();

        let app = app(AppState {
            ticket_service: TicketService::new(Arc::new(mock_repo)),
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/tickets")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"priority": "HIGH"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn create_returns_400() {
        init_test();

        let mock_repo = MockTicketRepository::new();

        let app = app(AppState {
            ticket_service: TicketService::new(Arc::new(mock_repo)),
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/tickets")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title": "", "priority": "HIGH"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn stats_returns_200() {
        init_test();

        let (mut by_status, mut by_priority) = (HashMap::new(), HashMap::new());

        by_status.insert(TicketStatus::Open, 1);
        by_priority.insert(Priority::High, 1);

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_stats()
            .returning(move || Ok((by_status.clone(), by_priority.clone())));

        let app = app(AppState {
            ticket_service: TicketService::new(Arc::new(mock_repo)),
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/tickets/stats")
                    .header("content-type", "application/json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(
            body,
            serde_json::json!(
                {
                    "by_status": {
                        "OPEN": 1
                    },
                    "by_priority": {
                        "HIGH": 1
                    }
                }
            )
        );
    }

    #[tokio::test]
    async fn stats_returns_500() {
        init_test();

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_stats()
            .returning(move || Err(DatabaseError::InvalidStatsRow));

        let app = app(AppState {
            ticket_service: TicketService::new(Arc::new(mock_repo)),
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/tickets/stats")
                    .header("content-type", "application/json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        #[derive(Deserialize)]
        struct ErrorResponse {
            message: String
        }

        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: ErrorResponse = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(body.message, "Something went wrong in the backend");
    }

    #[tokio::test]
    async fn get_by_id_returns_200() {
        init_test();

        let title = "Printer is on fire".to_string();
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

        let app = app(AppState {
            ticket_service: TicketService::new(Arc::new(mock_repo)),
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/tickets/1")
                    .header("content-type", "application/json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(
            body,
            serde_json::json!(
                {
                    "id": 1,
                    "title": "Printer is on fire",
                    "description": null,
                    "status": "OPEN",
                    "priority": "HIGH",
                    "assignee": null,
                    "created_at": "2026-01-01T00:00:00Z",
                }
            )
        );
    }

    #[tokio::test]
    async fn get_by_id_returns_404() {
        init_test();

        let id = 42;

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(id))
            .returning(move |_| Ok(None));

        let app = app(AppState {
            ticket_service: TicketService::new(Arc::new(mock_repo)),
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/tickets/42")
                    .header("content-type", "application/json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn list_returns_200_no_params() {
        init_test();

        let title = "Printer is on fire".to_string();
        let second_title = "Printer is on fire (again)".to_string();

        let tickets = vec![
            Ticket {
                id: 1,
                title,
                description: None,
                status: TicketStatus::Open,
                priority: Priority::High,
                assignee: None,
                created_at: datetime!(2026-01-01 00:00 UTC),
            },
            Ticket {
                id: 2,
                title: second_title,
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
            .with(eq(None), eq(None))
            .returning(move |_, _| Ok(tickets.clone()));

        let app = app(AppState {
            ticket_service: TicketService::new(Arc::new(mock_repo)),
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/tickets")
                    .header("content-type", "application/json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(
            body,
            serde_json::json!(
                [
                    {
                        "id": 1,
                        "title": "Printer is on fire",
                        "description": null,
                        "status": "OPEN",
                        "priority": "HIGH",
                        "assignee": null,
                        "created_at": "2026-01-01T00:00:00Z",
                    },
                    {
                        "id": 2,
                        "title": "Printer is on fire (again)",
                        "description": "This time the flames are blue",
                        "status": "OPEN",
                        "priority": "HIGH",
                        "assignee": "Firefighters",
                        "created_at": "2026-01-02T00:00:00Z",
                    }
                ]
            )
        );
    }

    #[tokio::test]
    async fn list_returns_200() {
        init_test();

        let title = "Printer is on fire".to_string();
        let second_title = "Printer is on fire (again)".to_string();

        let tickets = vec![
            Ticket {
                id: 1,
                title,
                description: None,
                status: TicketStatus::Open,
                priority: Priority::High,
                assignee: None,
                created_at: datetime!(2026-01-01 00:00 UTC),
            },
            Ticket {
                id: 2,
                title: second_title,
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

        let app = app(AppState {
            ticket_service: TicketService::new(Arc::new(mock_repo)),
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/tickets?status=OPEN&priority=HIGH")
                    .header("content-type", "application/json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(
            body,
            serde_json::json!(
                [
                    {
                        "id": 1,
                        "title": "Printer is on fire",
                        "description": null,
                        "status": "OPEN",
                        "priority": "HIGH",
                        "assignee": null,
                        "created_at": "2026-01-01T00:00:00Z",
                    },
                    {
                        "id": 2,
                        "title": "Printer is on fire (again)",
                        "description": "This time the flames are blue",
                        "status": "OPEN",
                        "priority": "HIGH",
                        "assignee": "Firefighters",
                        "created_at": "2026-01-02T00:00:00Z",
                    }
                ]
            )
        );
    }

    #[tokio::test]
    async fn update_status_returns_200() {
        init_test();

        let title = "Printer is on fire".to_string();

        let id = 1;

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_update_status()
            .with(eq(id), eq::<TicketUpdate>(TicketStatus::InProgress.into()))
            .returning(move |_, _| {
                Ok(Some(Ticket {
                    id: 1,
                    title: title.clone(),
                    description: None,
                    status: TicketStatus::InProgress,
                    priority: Priority::High,
                    assignee: None,
                    created_at: datetime!(2026-01-01 00:00 UTC),
                }))
            });

        let app = app(AppState {
            ticket_service: TicketService::new(Arc::new(mock_repo)),
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri("/tickets/1/status")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"status": "IN_PROGRESS"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(
            body,
            serde_json::json!(
                {
                    "id": 1,
                    "title": "Printer is on fire",
                    "description": null,
                    "status": "IN_PROGRESS",
                    "priority": "HIGH",
                    "assignee": null,
                    "created_at": "2026-01-01T00:00:00Z",
                }
            )
        );
    }

    #[tokio::test]
    async fn update_status_returns_422() {
        init_test();

        let mock_repo = MockTicketRepository::new();

        let app = app(AppState {
            ticket_service: TicketService::new(Arc::new(mock_repo)),
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri("/tickets/1/status")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"status": "TEST"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}

TESTS_EOF
