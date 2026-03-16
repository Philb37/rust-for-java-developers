cargo new rust-project
cargo add axum --features macros
cargo add tower config tracing anyhow thiserror
cargo add tower_http --features trace
cargo add tokio --features full
cargo add time --features serde,serde-well-known,macros,formatting,parsing
cargo add serde --features derive
cargo add sea-orm --features sqlx-postgres,runtime-tokio,macros,mock
cargo add tracing_subscriber --features env-filter
cargo add validator --features derive
cargo add --dev serde_json http_body_util

create project structure