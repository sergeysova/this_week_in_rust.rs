ARG RUST_VERSION=1.43.0

FROM rust:$RUST_VERSION as build

WORKDIR /directory

COPY ./Cargo.lock ./Cargo.toml ./src ./

RUN cargo build --release

# ----------------------------------------------------------------- #

FROM debian:9-slim

RUN seq 1 8 | xargs -I{} mkdir -p /usr/share/man/man{} && \
  apt update && \
  apt -y install libpq-dev postgresql-client ca-certificates && \
  update-ca-certificates && \
  apt clean

WORKDIR /app

COPY --from=build /directory/target/release/this_week_in_rust ./bot

CMD ["/app/bot"]
