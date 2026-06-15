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

// TALK : Remove derive to add content to the TALK
// TALK : Copy needed for map_or in the list in TicketDomain
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
