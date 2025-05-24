# ----------
#    USER
# ----------
FROM alpine:latest AS user
RUN adduser -S -s /bin/false -D lesbin
RUN mkdir /dir

# -----------
#    BUILD
# -----------
FROM rust:1-alpine AS build
WORKDIR /build
RUN apk add --no-cache --update build-base

# Pre-cache dependencies
COPY ["Cargo.toml", "Cargo.lock", "./"]
RUN mkdir src \
    && echo "// Placeholder" > src/lib.rs \
    && cargo build --release \
    && rm src/lib.rs

# Build
ARG SQLX_OFFLINE true
COPY ./migrations ./migrations
COPY ./.sqlx ./.sqlx
COPY src ./src
RUN cargo build --release

# -----------
#   RUNTIME
# -----------
FROM scratch AS runtime
WORKDIR /opt

COPY --from=build /build/target/release/lesbin-api /usr/bin/lesbin-api

# Import and switch to non-root user.
COPY --from=user /etc/passwd /etc/passwd
COPY --from=user /bin/false /bin/false
USER lesbin
COPY --from=user --chown=lesbin /dir /srv/lesbin

ENV API_ADDRESS=0.0.0.0:8255
ENV DATABASE_URL="sqlite:///srv/lesbin/data.sqlite?mode=rwc"
ENV RUST_LOG=info
EXPOSE 8255

ENTRYPOINT ["/usr/bin/lesbin-api"]