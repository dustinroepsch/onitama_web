FROM rust:1-alpine AS build-env
COPY . /code
WORKDIR /code
RUN cargo build --release

FROM alpine:latest
COPY --from=build-env /code/target/release/onitama /
ENTRYPOINT [ "./onitama" ]