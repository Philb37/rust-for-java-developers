# Live-coding cheat sheet

Scripts and snippets share one numbering, so the next number always tells you what to do.
Run scripts from the repo root (`bash scripts/NN-*.sh`). They `cd` into `rust/src` themselves.
To expand a snippet, type its prefix (`NN-…`) in the right spot and pick it from the completion list.

- **seed**: the script left a `// NN-…` line in the file. Replace that line with the snippet.
- **hint**: a previous snippet left a `// Step NN -> …` line. Expand on the empty line under it.
- **todo**: replace the `todo!()` under the `// Step NN` hint.

| #  | What                                          | File                                     | Where |
|----|-----------------------------------------------|------------------------------------------|-------|
| 01 | **script** `01-diesel-setup.sh`               | diesel deps, models/ + repository/ files | —     |
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
| 18 | **script** `18-axum-core-setup.sh`            | axum deps, rewrites `main.rs`, DTOs, extractors, `config.yml` | — |
| 19 | `19-schemas-app_error`                        | `schemas/app_error.rs`                   | seed  |
| 20 | `20-services-ticket_service-skeleton`         | `services/ticket_service.rs`             | seed  |
| 21 | `21-services-ticket_service-create`           | same                                     | todo  |
| 22 | `22-services-ticket_service-methods`: the other 4 | same (inside the impl, after `create`) | hint |
| 23 | `23-routes-ticket_routes-router`: uses + `router()` | `routes/ticket_routes.rs`          | seed  |
| 24 | `24-routes-ticket_routes-handlers`            | same                                     | hint  |
| 25 | `25-config`                                   | `config.rs`                              | seed  |
| 26 | `26-app_state`                                | `app_state.rs`                           | seed  |
| 27 | `27-main-body`, **then delete the hello-world `fn main`** | `main.rs`                    | seed  |
| 28 | **script** `28-append-tests.sh`, then `cargo test` | appends tests to service + routes   | —     |

`…` = `repository-diesel_ticket_repository`.

## Written by the scripts (not live-coded)

- **01**: `models/ticket_status.rs` (same pattern as `priority.rs`), `models/stat.rs`, `repository/mod.rs`
- **18**: `main.rs` (all `mod`s + `// 27-main-body` seed), `schemas/*` except `app_error.rs`, `extractors/*`, `services/mod.rs`, `routes.rs`, `config.yml`

## Heads-up

- Between steps 7 and 18, rust-analyzer shows errors: `models/` and `repository/` use `app_state` and `schemas`, and those only exist after script 18.
- After step 23, `router()` has errors until step 24 adds the handlers.
