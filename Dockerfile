FROM beerpsi/cargo-chef-musl-mimalloc:latest AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN apt-get update && apt-get install -y --no-install-recommends \
  openssl \
  && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM debian:trixie-slim AS runtime
WORKDIR /app
ENV NAME=dap-dau-vao-tuong
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/${NAME} /app/
COPY /assets/ /app/assets/
RUN apt-get update && apt-get install -y --no-install-recommends \
  libcurl4 \
  && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
CMD /app/${NAME}