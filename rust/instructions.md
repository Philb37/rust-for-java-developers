cargo new tickets

cargo add diesel --features postgres_backend,time
cargo add diesel-async --features tokio-postgres,deadpool,postgres
cargo add deadpool@0.13 --features rt_tokio_1
cargo add serde --features derive

cargo add axum --features macros
cargo add tower config tracing anyhow thiserror async-trait
cargo add tower_http --features trace
cargo add tokio --features full
cargo add time --features serde,serde-well-known,macros,formatting,parsing
cargo add tracing-subscriber --features env-filter
cargo add validator --features derive

cargo add --dev serde_json http_body_util mockall