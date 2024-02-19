FROM rust as BUILD

WORKDIR /bookstore

COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

RUN cargo run --release

FROM debian:bullseye-slim

WORKDIR /bookstore

RUN apt-get update && apt-get -y install pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /bookstore/target/release/bookstore ./bookstore

EXPOSE 8080

CMD ["./bookstore"]
