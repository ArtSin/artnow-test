FROM rust:1.85-bookworm AS builder

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && mv cargo-binstall /usr/local/cargo/bin

RUN mkdir -p /app
WORKDIR /app

RUN cargo binstall cargo-nextest -y --install-path .

COPY . .

RUN ./cargo-nextest nextest archive --release --archive-file tests.tar.zst && cargo clean

FROM debian:bookworm-slim AS artnow-test
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openjdk-17-jre \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /app
WORKDIR /app

COPY --from=builder /app /app

ENV RUST_BACKTRACE=1
CMD ./cargo-nextest nextest run --profile junit --no-fail-fast -j 4 --archive-file tests.tar.zst \
    && /allure/bin/allure generate --clean --single-file target/nextest/junit/
