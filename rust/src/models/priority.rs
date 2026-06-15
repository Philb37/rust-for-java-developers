use std::io::Write;

use diesel::{backend::Backend, deserialize::{self, FromSql, FromSqlRow}, expression::AsExpression, pg::Pg, serialize::{IsNull, ToSql}, sql_types::Text};
use serde::{Deserialize, Serialize};

// TALK : Remove derive to add content to the TALK
// TALK : Copy needed for map_or in the list in TicketDomain
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, AsExpression, FromSqlRow)]
#[serde(rename_all = "UPPERCASE")]
#[diesel(sql_type = Text)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl ToSql<Text, Pg> for Priority {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        
        let priority = match self {
            Priority::Low => "LOW",
            Priority::Medium => "MEDIUM",
            Priority::High => "HIGH",
            Priority::Critical => "CRITICAL"
        };

        out.write_all(priority.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for Priority {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match str::from_utf8(bytes.as_bytes())? {
            "LOW" => Ok(Priority::Low),
            "MEDIUM" => Ok(Priority::Medium),
            "HIGH" => Ok(Priority::High),
            "CRITICAL" => Ok(Priority::Critical),
            other => Err(format!("Unknown priority {} in database", other).into())
        } 
    }
}