FROM rust:latest
WORKDIR /usr/src/meowth
COPY . .
RUN cargo build
RUN cargo install --path .
CMD ["/usr/local/cargo/bin/meowth"]
