#!/usr/bin/env bash

cd ./rust-temp/src

# Cargo dependencies
cargo add diesel --features postgres_backend,time
cargo add diesel-async --features tokio-postgres,deadpool,postgres
cargo add deadpool@0.13 --features rt_tokio_1
cargo add serde --features derive
cargo add async-trait
cargo add thiserror
cargo add time --features formatting,macros,parsing,serde,serde-well-known
cargo add mockall --dev

# Seed main.rs with its first snippet (keep the existing content below it)
printf '// 2-main\n\n%s\n' "$(cat main.rs)" > main.rs.tmp && mv main.rs.tmp main.rs

# Creating diesel schema (seeded with the first snippet to expand)
echo "// 8-schema" > schema.rs

# Creating models files (ticket_status and stat written directly, the rest seeded with their first snippet)
mkdir -p models

echo "// 3-models-mod" > models/mod.rs
echo "// 4-models-priority-enum" > models/priority.rs
echo "// 7-models-ticket" > models/ticket.rs

# ticket_status.rs follows the exact same pattern as priority.rs (live-coded in steps 4 to 6)
cat > models/ticket_status.rs <<'RS'
use std::io::Write;

use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Text,
};
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Deserialize,
    Serialize,
    AsExpression,
    FromSqlRow
)]
#[serde(rename_all = "UPPERCASE")]
#[diesel(sql_type = Text)]
pub enum TicketStatus {
    Open,
    #[serde(rename(serialize = "IN_PROGRESS", deserialize = "IN_PROGRESS"))]
    InProgress,
    Resolved,
    Closed,
}

impl ToSql<Text, Pg> for TicketStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let status = match self {
            TicketStatus::Open => "OPEN",
            TicketStatus::InProgress => "IN_PROGRESS",
            TicketStatus::Resolved => "RESOLVED",
            TicketStatus::Closed => "CLOSED",
        };

        out.write_all(status.as_bytes())?;

        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for TicketStatus {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match str::from_utf8(bytes.as_bytes())? {
            "OPEN" => Ok(TicketStatus::Open),
            "IN_PROGRESS" => Ok(TicketStatus::InProgress),
            "RESOLVED" => Ok(TicketStatus::Resolved),
            "CLOSED" => Ok(TicketStatus::Closed),
            other => Err(format!("Unkown status {} found in database", other).into()),
        }
    }
}
RS

# stat.rs maps the rows of the stats() GROUPING SETS query (step 16)
cat > models/stat.rs <<'RS'
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
RS

# Creating repository files (mod.rs + database_error written directly, the rest seeded with their first snippet)
mkdir -p repository

printf 'pub mod database_error;\npub mod ticket_repository;\npub mod diesel_ticket_repository;\n' > repository/mod.rs

# database_error.rs is the persistence-layer error, wrapped by AppError in step 19
cat > repository/database_error.rs <<'RS'
use diesel_async::pooled_connection::deadpool::PoolError;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Connection pool error")]
    PoolError(#[from] PoolError),
    #[error("Query failed")]
    QueryError(#[from] diesel::result::Error),
    #[error("unexpected row in stats grouping set")]
    InvalidStatsRow
}
RS
echo "// 9-repository-ticket_repository" > repository/ticket_repository.rs
echo "// 10-repository-diesel_ticket_repository-struct" > repository/diesel_ticket_repository.rs
