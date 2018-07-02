FROM alpine:latest
EXPOSE 80
CMD ROCKET_ENV=production cargo run --release

WORKDIR /src
ONBUILD COPY ./ /src/


RUN apt-get update && \
apt-get upgrade && \
apt-get install sqlite3 sass -y && \
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly && \
cargo install diesel_cli && \
diesel migration run && \
sass --no-source-map /src/templates/style.scss /src/static/css/style.css
