use diesel::{
    prelude::QueryableByName,
    sql_types::{BigInt, Nullable, Text},
};

use crate::models::{priority::Priority, ticket_status::TicketStatus};

// Result of the GROUPING SETS query: each row carries EITHER a status or a priority
// (the other column is NULL), plus its count. QueryableByName matches by column name,
// and the enums' own FromSql<Text> impls deserialize the text values directly.
#[derive(QueryableByName, Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct Stat {
    #[diesel(sql_type = Nullable<Text>)]
    pub status: Option<TicketStatus>,
    #[diesel(sql_type = Nullable<Text>)]
    pub priority: Option<Priority>,
    #[diesel(sql_type = BigInt)]
    pub count: i64
}