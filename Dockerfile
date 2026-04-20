FROM node:24-alpine AS build_ui
WORKDIR /build

RUN corepack enable

COPY ./ui ./
RUN npm ci && npm run generate


FROM rust:alpine AS build_server

WORKDIR /build
COPY ./server /build
COPY --from=build_ui /build/.output/public /build/ui
RUN apk add --no-cache musl-dev && \
    cargo build --release


FROM alpine

WORKDIR /app
COPY --from=build_server /build/target/release/magicians-server /app/magicians-server

ENTRYPOINT ["/app/magicians-server"]
