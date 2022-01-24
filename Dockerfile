FROM rust:latest

WORKDIR /usr/src/rsc-vom
COPY . .

RUN cargo test