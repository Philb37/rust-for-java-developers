use std::{sync::Arc, time::Duration};

use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{
        AsyncDieselConnectionManager,
        deadpool::{BuildError, Pool},
    },
};

use deadpool::Runtime;

use crate::{
    config::AppConfig, repository::diesel_ticket_repository::DieselTicketRepository,
    services::ticket_service::TicketService,
};

#[derive(Clone)]
pub struct AppState {
    pub ticket_service: TicketService,
}

pub type DbPool = Pool<AsyncPgConnection>;

impl AppState {
    // TALK : Pass ownership the first time to trigger the moved error
    pub async fn build(app_config: &AppConfig) -> Result<Self, BuildError> {
        let manager =
            AsyncDieselConnectionManager::<AsyncPgConnection>::new(&app_config.postgres.url);

        let pool = &app_config.postgres.pool;

        let database = Pool::builder(manager)
            .runtime(Runtime::Tokio1)
            .max_size(pool.max_size)
            .create_timeout(Some(Duration::from_secs(pool.create_timeout_secs)))
            .wait_timeout(Some(Duration::from_secs(pool.acquire_timeout_secs)))
            .recycle_timeout(Some(Duration::from_mins(10)))
            .build()?;

        let ticket_repository = DieselTicketRepository::new(database);

        let ticket_service = TicketService::new(Arc::new(ticket_repository));

        Ok(Self { ticket_service })
    }
}
