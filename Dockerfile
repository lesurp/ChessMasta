FROM alpine:latest
EXPOSE 80
CMD ROCKET_ENV=production cargo run --release

    RUN apt-get update && \
    apt-get upgrade && \
    apt-get install sqlite3 && \
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly && \
    cargo install diesel_cli && \
    diesel migration run

WORKDIR /src
ONBUILD COPY ./ /src/