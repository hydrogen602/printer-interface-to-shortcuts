FROM rust:slim-bookworm


RUN apt-get update && apt-get upgrade
RUN apt-get install -y pkg-config libssl-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock build.rs .env /app/

# Cache downloaded+built dependencies
RUN \
  mkdir /app/src && \
  echo 'fn main() {}' > /app/src/main.rs && \
  cargo build --release && \
  rm -vf /app/src/main.rs

# Build our actual code
COPY src /app/src
RUN \
  touch src/main.rs && \
  cargo build --release

EXPOSE 3000

# CMD cd /app && cargo run --release
ENTRYPOINT ["/app/target/release/printer-actions"]
