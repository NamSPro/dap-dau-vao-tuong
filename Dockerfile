FROM rust:latest AS builder
ENV NAME=duelists-of-eternity

# First build a dummy project with our dependencies to cache them in Docker
WORKDIR /usr/src
RUN mkdir ${NAME}
WORKDIR /usr/src/${NAME}
COPY . .
RUN rm -rf target
RUN cargo build --release 

# Second stage putting the build result into a debian trixie-slim image
FROM debian:trixie-slim
ENV NAME=duelists-of-eternity

COPY --from=builder /usr/src/${NAME}/target/release/${NAME} /usr/local/bin/${NAME}
COPY ./assets /usr/local/bin/assets
COPY ./.env /usr/local/bin/.env
WORKDIR /usr/local/bin
CMD ${NAME}
