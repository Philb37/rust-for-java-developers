use axum::{
    Router, debug_handler,
    extract::{Path, State},
    routing::{get, patch},
};

use crate::{
    app_state::AppState,
    extractors::{app_query::AppQuery, validated_json::ValidatedJson},
    schemas::{
        app_error::AppError,
        app_response::{AppJson, Created},
        create_ticket_request::CreateTicketRequest,
        request_params::RequestParams,
        stats_response::StatsResponse,
        ticket_response::TicketResponse,
        update_status_request::UpdateStatusRequest,
    },
};

pub fn router() -> Router<AppState> {
    let nested_tickets_router = Router::new()
        .route("/", get(list).post(create))
        .route("/stats", get(stats))
        .route("/{id}", get(get_by_id))
        .route("/{id}/status", patch(update_status));

    Router::new().nest("/tickets", nested_tickets_router)
}

#[debug_handler]
async fn create(
    State(app_state): State<AppState>,
    ValidatedJson(request): ValidatedJson<CreateTicketRequest>,
) -> Result<Created<TicketResponse>, AppError> {
    Ok(Created(app_state.ticket_service.create(request).await?))
}

#[debug_handler]
async fn stats(State(app_state): State<AppState>) -> Result<AppJson<StatsResponse>, AppError> {
    Ok(AppJson(app_state.ticket_service.stats().await?))
}

#[debug_handler]
async fn get_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<AppJson<TicketResponse>, AppError> {
    Ok(AppJson(app_state.ticket_service.get_by_id(id).await?))
}

#[debug_handler]
async fn list(
    State(app_state): State<AppState>,
    AppQuery(params): AppQuery<RequestParams>,
) -> Result<AppJson<Vec<TicketResponse>>, AppError> {
    Ok(AppJson(
        app_state
            .ticket_service
            .list(params.status, params.priority)
            .await?,
    ))
}

#[debug_handler]
async fn update_status(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    ValidatedJson(request): ValidatedJson<UpdateStatusRequest>,
) -> Result<AppJson<TicketResponse>, AppError> {
    Ok(AppJson(
        app_state
            .ticket_service
            .update_status(id, request.status)
            .await?,
    ))
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use axum::{body::Body, extract::Request, http::StatusCode};
    use http_body_util::BodyExt;
    use mockall::predicate::eq;
    use time::macros::datetime;
    use tower::ServiceExt;

    use crate::{
        app,
        models::{
            priority::Priority, stat::Stat, ticket::{NewTicket, Ticket, TicketUpdate}, ticket_status::TicketStatus
        },
        repository::ticket_repository::MockTicketRepository,
        services::ticket_service::TicketService,
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

        let stats = vec![
            Stat {
                priority: Some(Priority::High),
                status: None,
                count: 1,
            },
            Stat {
                priority: None,
                status: Some(TicketStatus::Open),
                count: 1
            },
        ];

        let mut mock_repo = MockTicketRepository::new();
        mock_repo
            .expect_stats()
            .with()
            .returning(move || Ok(stats.clone()));

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
