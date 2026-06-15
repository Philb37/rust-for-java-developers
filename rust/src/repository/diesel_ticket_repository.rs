use crate::{
    app_state::DbPool, models::{
        priority::Priority,
        ticket::{NewTicket, Ticket, TicketUpdate},
        ticket_status::TicketStatus,
    }, repository::ticket_repository::TicketRepository, schema::tickets, schemas::app_error::AppError
};

use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, dsl::insert_into, dsl::update,
};
use diesel_async::RunQueryDsl;

pub struct DieselTicketRepository {
    pub database: DbPool,
}

impl DieselTicketRepository {
    
    pub fn new(database: DbPool) -> Self {
        Self { database }
    }
}

#[async_trait::async_trait]
impl TicketRepository for DieselTicketRepository {

    async fn save(&self, ticket: NewTicket) -> Result<Ticket, AppError> {
        
        let mut connection = self.database.get().await?;

        Ok(insert_into(tickets::table)
            .values(ticket)
            .returning(Ticket::as_returning())
            .get_result(&mut connection)
            .await?
        )
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Ticket>, AppError> {
        
        let mut connection = self.database.get().await?;

        Ok(
            tickets::table
            .find(id)
            .select(Ticket::as_select())
            .first(&mut connection)
            .await
            .optional()?
        )
    }

    async fn list(
        &self,
        status: Option<TicketStatus>,
        priority: Option<Priority>,
    ) -> Result<Vec<Ticket>, AppError> {
        
        let mut connection = self.database.get().await?;

        let mut query = tickets::table.into_boxed();
        
        if let Some(status) = status {
            query = query.filter(tickets::status.eq(status));
        }

        if let Some(priority) = priority {
            query = query.filter(tickets::priority.eq(priority));
        }

        Ok(
            query.select(Ticket::as_select())
            .get_results(&mut connection)
            .await?
        )
    }

    async fn update_status(&self, id: i32, ticket_update: TicketUpdate) -> Result<Option<Ticket>, AppError> {
        
        let mut connection = self.database.get().await?;

        Ok(
            update(tickets::table.find(id))
            .set(ticket_update)
            .returning(Ticket::as_returning())
            .get_result(&mut connection)
            .await
            .optional()?
        )
    }
}
