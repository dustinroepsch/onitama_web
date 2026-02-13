FROM rust:1-bullseye
COPY . /code
WORKDIR /code
RUN cargo build --release
ENTRYPOINT [ "cargo" , "run", "--release"]