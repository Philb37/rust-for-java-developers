cd ./rust-temp/src

# Cargo dependencies (axum web stack + config + dev test helpers)
cargo add anyhow config thiserror tower tracing
cargo add axum --features macros
cargo add tokio --features full
cargo add tower-http --features trace
cargo add tracing-subscriber --features env-filter
cargo add validator --features derive
cargo add http-body-util --dev
cargo add serde_json --dev

# Rewriting main.rs: every module declared, seeded with the finale snippet.
# The hello-world main is kept so the crate still has an entry point until step 27 replaces it.
cat > main.rs <<'RS'
mod schema;
mod models;
mod repository;
mod app_state;
mod config;
mod extractors;
mod routes;
mod schemas;
mod services;

// 27-main-body
fn main() {
    println!("Hello, world!");
}
RS

# Creating schemas files (mod.rs + all DTOs written directly; only app_error is live-coded)
mkdir -p schemas

printf 'pub mod create_ticket_request;\npub mod ticket_response;\npub mod stats_response;\npub mod update_status_request;\npub mod app_error;\npub mod app_response;\npub mod request_params;\n' > schemas/mod.rs

cat > schemas/request_params.rs <<'RS'
use serde::Deserialize;

use crate::models::{priority::Priority, ticket_status::TicketStatus};

#[derive(Deserialize)]
pub struct RequestParams {
    pub status: Option<TicketStatus>,
    pub priority: Option<Priority>
}
RS

cat > schemas/create_ticket_request.rs <<'RS'
use serde::{Deserialize};
use validator::{Validate, ValidationError};

use crate::models::{priority::Priority};

#[derive(Deserialize, Validate)]
#[cfg_attr(test, derive(Clone, Debug))]
pub struct CreateTicketRequest {
    #[validate(custom(function = "not_blank"))]
    pub title: String,
    pub priority: Priority, // That's a leak of our domain model, but let's keep it simple for now
    pub description: Option<String>,
    pub assignee: Option<String>,
}

fn not_blank(value: &str) -> Result<(), ValidationError> {
    match value.trim().is_empty() {
        true => Err(ValidationError::new("not_blank").with_message("Title must not be blank.".into())),
        false => Ok(()),
    }
}
RS

cat > schemas/update_status_request.rs <<'RS'
use serde::Deserialize;
use validator::Validate;

use crate::models::ticket_status::TicketStatus;

#[derive(Deserialize, Validate)]
pub struct UpdateStatusRequest {
    pub status: TicketStatus
}
RS

cat > schemas/stats_response.rs <<'RS'
use std::collections::HashMap;

use serde::Serialize;

use crate::models::{priority::Priority, ticket_status::TicketStatus};

#[derive(Serialize)]
pub struct StatsResponse {
    pub by_status: HashMap<TicketStatus, i64>,
    pub by_priority: HashMap<Priority, i64>,
}
RS

cat > schemas/ticket_response.rs <<'RS'
use serde::Serialize;
use time::{OffsetDateTime, serde::rfc3339};

use crate::models::{priority::Priority, ticket::Ticket, ticket_status::TicketStatus};

#[derive(Serialize)]
pub struct TicketResponse {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: TicketStatus, // That's a leak of our domain model, but let's keep it simple for now
    pub priority: Priority,   // That's a leak of our domain model, but let's keep it simple for now
    pub assignee: Option<String>,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
}

impl From<Ticket> for TicketResponse {
    fn from(ticket: Ticket) -> Self {
        Self {
            id: ticket.id,
            title: ticket.title,
            description: ticket.description,
            status: ticket.status,
            priority: ticket.priority,
            assignee: ticket.assignee,
            created_at: ticket.created_at,
        }
    }
}
RS

# app_error.rs is the only live-coded schema (enum + IntoResponse + log middleware)
echo "// 19-schemas-app_error" > schemas/app_error.rs

# app_response.rs is written directly (AppJson + Created wrappers) to save talk time
cat > schemas/app_response.rs <<'RS'
use axum::{extract::FromRequest, http::StatusCode, response::{IntoResponse, Response}};

use crate::schemas::app_error::AppError;

// Create our own JSON extractor by wrapping `axum::Json`. This makes it easy to override the
// rejection and provide our own which formats errors to match our application.
//
// `axum::Json` responds with plain text if the input is invalid.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(
    pub T
);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

pub struct Created<T>(
    pub T
);

impl<T> IntoResponse for Created<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        (StatusCode::CREATED, AppJson(self.0)).into_response()
    }
}
RS

# Creating services files (mod.rs written directly)
mkdir -p services

printf 'pub mod ticket_service;\n' > services/mod.rs
echo "// 20-services-ticket_service-skeleton" > services/ticket_service.rs

# Creating routes files (routes.rs is the module root next to the routes/ dir, written directly)
mkdir -p routes

printf 'pub mod ticket_routes;\n' > routes.rs
echo "// 23-routes-ticket_routes-router" > routes/ticket_routes.rs

# Creating extractors files (mod.rs + both extractors written directly to save talk time)
mkdir -p extractors

printf 'pub mod validated_json;\npub mod app_query;\n' > extractors/mod.rs

cat > extractors/app_query.rs <<'RS'
use axum::extract::FromRequestParts;

use crate::schemas::app_error::AppError;

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(AppError))]
pub struct AppQuery<T>(pub T);
RS

cat > extractors/validated_json.rs <<'RS'
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::schemas::{app_error::AppError, app_response::AppJson};

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync
{
    type Rejection = AppError;

    async fn from_request(
            req: Request,
            state: &S,
        ) -> Result<Self, Self::Rejection> {

        let AppJson(value) = AppJson::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}
RS

# Creating app_state and config (both live-coded)
echo "// 25-config" > config.rs
echo "// 26-app_state" > app_state.rs

# Runtime configuration loaded by config.rs (lives at the crate root, next to Cargo.toml)
cat > ../config.yml <<'YAML'
# PostgreSQL — started via Docker Compose
postgres:
  url: postgresql://postgres:password@localhost:5432/intro-to-rust
  schema: public
  # Connection pool — kept equivalent to Spring's spring.r2dbc.pool.* settings
  pool:
    max_size: 10            # deadpool max_size  <-> spring.r2dbc.pool.max-size
    acquire_timeout_secs: 10 # deadpool wait_timeout   <-> spring.r2dbc.pool.max-acquire-time
    create_timeout_secs: 10  # deadpool create_timeout <-> spring.r2dbc.pool.max-create-connection-time

# Server
server:
  port: 1337
YAML
