FROM public.ecr.aws/docker/library/rust:1.64.0 AS base
WORKDIR /app

FROM base AS development
RUN cargo install diesel_cli --version 2.0.0 --no-default-features --features postgres && \
  cargo install cargo-watch && \
  rustup component add rustfmt
COPY . .
