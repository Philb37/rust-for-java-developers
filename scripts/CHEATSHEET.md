# Live-coding cheat sheet

Scripts and snippets share one numbering, so the next number always tells you what to do.
Run scripts from the repo root (`bash scripts/NN-*.sh`). They `cd` into `rust/src` themselves.
To expand a snippet, type its prefix (`NN-…`) in the right spot and pick it from the completion list.

- **seed**: the script left a `// NN-…` line in the file. Replace that line with the snippet.
- **hint**: a previous snippet left a `// Step NN -> …` line. Expand on the empty line under it.
- **todo**: replace the `todo!()` under the `// Step NN` hint.

| #  | What                                          | File                                     | Where |
|----|-----------------------------------------------|------------------------------------------|-------|
| 01 | **script** `01-diesel-setup.sh`               | diesel deps + thiserror, models/ + repository/ files | — |
| 2  | `2-main`: schema / models / repository mods   | `main.rs`                                | seed  |
| 3  | `3-models-mod`                                | `models/mod.rs`                          | seed  |
| 4  | `4-models-priority-enum`: uses + bare enum    | `models/priority.rs`                     | seed  |
| 5  | `5-models-priority-derive`                    | `models/priority.rs` (above the enum)    | hint  |
| 6  | `6-models-priority-impl`: ToSql / FromSql     | `models/priority.rs` (below the enum)    | hint  |
| 7  | `7-models-ticket`                             | `models/ticket.rs`                       | seed  |
| 8  | `8-schema`                                    | `schema.rs`                              | seed  |
| 9  | `9-repository-ticket_repository`: trait       | `repository/ticket_repository.rs`        | seed  |
| 10 | `10-…-struct`: uses + struct                  | `repository/diesel_ticket_repository.rs` | seed  |
| 11 | `11-…-impl`: `new()`                          | same                                     | hint  |
| 12 | `12-…-trait_impl`: skeleton with `todo!()`    | same                                     | hint  |
| 13 | `13-…-save`                                   | same                                     | todo  |
| 14 | `14-…-find_by_id`                             | same                                     | todo  |
| 15 | `15-…-list`                                   | same                                     | todo  |
| 16 | `16-…-stats`                                  | same                                     | todo  |
| 17 | `17-…-update_status`                          | same                                     | todo  |
| 18 | **script** `18-axum-core-setup.sh`            | axum deps, rewrites `main.rs`, DTOs, extractors, `HttpError` / `DomainError`, `config.yml` | — |
| 19 | `19-schemas-app_error`: wraps Http / Database / Domain | `schemas/app_error.rs`          | seed  |
| 20 | `20-services-ticket_service-skeleton`         | `services/ticket_service.rs`             | seed  |
| 21 | `21-services-ticket_service-create`           | same                                     | todo  |
| 22 | `22-services-ticket_service-methods`: the other 4 | same (inside the impl, after `create`) | hint |
| 23 | `23-controllers-ticket_controller-router`: uses + `router()` | `controllers/ticket_controller.rs` | seed |
| 24 | `24-controllers-ticket_controller-handlers`: `State<TicketService>` | same               | hint  |
| 25 | `25-config`                                   | `config.rs`                              | seed  |
| 26 | `26-app_state`: `#[derive(FromRef)]`           | `app_state.rs`                           | seed  |
| 27 | `27-main-app`: `app()` router + layers + state | `main.rs` (between `main()` and `shutdown_signal()`) | seed |
| 28 | `28-main-init`: config + state + `app()` call  | `main.rs` (inside `main()`)              | seed  |
| 29 | **script** `29-append-tests.sh`, then `cargo test` | appends tests to service + controller | —   |

`…` = `repository-diesel_ticket_repository`.

## Written by the scripts (not live-coded)

- **01**: `models/ticket_status.rs` (same pattern as `priority.rs`), `models/stat.rs`, `repository/mod.rs`, `repository/database_error.rs`
- **18**: `main.rs` shell (uses, all `mod`s, tracing, listener, graceful shutdown + `// 27-main-app` / `// 28-main-init` seeds), `schemas/*` except `app_error.rs`, `extractors/*` (incl. `http_error.rs`), `services/mod.rs`, `services/domain_error.rs`, `controllers.rs`, `config.yml`

## Heads-up

- Between steps 7 and 18, rust-analyzer shows errors: `models/ticket.rs` uses `schemas` and `diesel_ticket_repository.rs` uses `app_state::DbPool`, and those only exist after script 18. (`DatabaseError` already exists: script 01 writes it.)
- Between script 18 and step 19, `extractors/http_error.rs` uses an `AppError` that does not exist yet, so it shows errors until step 19.
- After step 23, `router()` has errors until step 24 adds the handlers.
- Between script 18 and step 28, `main.rs` shows errors: `app_config` and `app` are only defined by step 28.
