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