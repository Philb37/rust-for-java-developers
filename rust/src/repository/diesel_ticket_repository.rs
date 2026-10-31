use std::collections::HashMap;

use crate::{
    app_state::DbPool, models::{
        priority::Priority, stat::Stat, ticket::{NewTicket, Ticket, TicketUpdate}, ticket_status::TicketStatus
    }, repository::{database_error::DatabaseError, ticket_repository::TicketRepository}, schema::tickets
};

use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, dsl::{insert_into, update}, sql_query,
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

    async fn save(&self, ticket: NewTicket) -> Result<Ticket, DatabaseError> {
        
        let mut connection = self.database.get().await?;

        Ok(insert_into(tickets::table)
            .values(ticket)
            .returning(Ticket::as_returning())
            .get_result(&mut connection)
            .await?
        )
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Ticket>, DatabaseError> {
        
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
    ) -> Result<Vec<Ticket>, DatabaseError> {
        
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
            .limit(100)
            .get_results(&mut connection)
            .await?
        )
    }

    // TALK : Doing this in memory is bad practice, it should be done in SQL directly.
    // The goal is to showcase different ways to do something similar to the springboot project.
    async fn stats(&self) -> Result<(HashMap<TicketStatus, i64>, HashMap<Priority, i64>), DatabaseError> {

        let mut connection = self.database.get().await?;

        let stats: Vec<Stat> = sql_query("select status, priority, count(*) from public.tickets group by grouping sets ((status), (priority))")
            .get_results(&mut connection)
            .await?;

        // let (mut by_status, mut by_priority) = (HashMap::new(), HashMap::new());

        // for ticket in &tickets {
        //     *by_status.entry(ticket.status).or_insert(0) += 1;
        //     *by_priority.entry(ticket.priority).or_insert(0) += 1;
        // }

        // TALK : Alternative to test
        // let (by_status, by_priority) = tickets.iter()
        //     .fold((HashMap::new(), HashMap::new()), |mut acc, ticket| {
        //         *acc.0.entry(ticket.status).or_insert(0) += 1;
        //         *acc.1.entry(ticket.priority).or_insert(0) += 1;
        //         acc
        //     });

        let (mut by_status, mut by_priority) = (HashMap::new(), HashMap::new());
        
        for stat in &stats {

            match (stat.priority, stat.status) {
                (Some(priority), None) => { by_priority.insert(priority, stat.count); },
                (None, Some(status)) => { by_status.insert(status, stat.count); },
                _ => {
                    return Err(DatabaseError::InvalidStatsRow);
                }
            }
        }

        Ok((by_status, by_priority))
    }

    async fn update_status(&self, id: i32, ticket_update: TicketUpdate) -> Result<Option<Ticket>, DatabaseError> {
        
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
