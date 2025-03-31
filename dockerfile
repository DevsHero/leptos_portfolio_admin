
FROM rustlang/rust:nightly-bookworm-slim as builder

ADD https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz 
RUN cp cargo-binstall /usr/local/cargo/bin

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends clang

RUN cargo binstall cargo-leptos -y
RUN rustup target add wasm32-unknown-unknown

RUN mkdir -p /app
WORKDIR /app
COPY . .
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/leptos-portfolio-admin /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080
CMD ["/app/leptos-portfolio-admin"]
