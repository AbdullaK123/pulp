# base
FROM rust:slim-bookworm

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

EXPOSE 8000

CMD ["./target/release/pulp"]
