FROM rust:1-bookworm AS build

WORKDIR /build

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=build /build/config.yml /app/config.yml
COPY --from=build /build/target/release/tickets /app/tickets

EXPOSE 1337

ENTRYPOINT [ "/app/tickets" ]