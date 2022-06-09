FROM rust:1.61.0 as builder

WORKDIR /app

RUN apt update && apt install default-libmysqlclient-dev -y

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt update && apt install libmariadb-dev -y && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/todo-app-rust /usr/local/bin/

CMD ["todo-app-rust"]