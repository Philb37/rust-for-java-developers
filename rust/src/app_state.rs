use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DbErr};

use crate::{config::AppConfig, services::ticket_service::{TicketService}};

#[derive(Clone)]
pub struct AppState {
    pub ticket_service: TicketService
}

impl AppState {

    // CONF : Pass ownership the first time to trigger the moved error
    pub async fn build(app_config: &AppConfig) -> Result<Self, DbErr> {

        let mut connect_option = ConnectOptions::new(&app_config.postgres.url);
        
        connect_option.max_connections(5)
            .connect_timeout(Duration::from_secs(10))
            .acquire_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_mins(10))
            .max_lifetime(Duration::from_mins(30))
            .set_schema_search_path(&app_config.postgres.schema);

        let database = 
            Database::connect(connect_option).await?;

        let ticket_service = TicketService::new(database);

        Ok(
            AppState {
                ticket_service
            }
        )
    }
}