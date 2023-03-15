FROM rust:1.68.0 as build

RUN USER=root cargo new --bin app
WORKDIR /app

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY sqlx-data.json .
COPY ./src ./src

RUN rm ./target/release/deps/budget_api*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /app/target/release/budget-api .

CMD ["./budget-api"]

