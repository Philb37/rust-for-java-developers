use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// CONF : Remove derive to add content to the conf
// CONF : Copy needed for map_or in the list in TicketDomain
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, EnumIter, DeriveActiveEnum)]
#[serde(rename_all = "UPPERCASE")]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", rename_all = "UPPERCASE")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}