FROM debian:latest
EXPOSE 80

ENV RUSTUP_HOME=/usr/local/rustup \
		CARGO_HOME=/usr/local/cargo \
		PATH=/usr/local/cargo/bin:$PATH

RUN apt-get update && \
apt-get upgrade && \
apt-get install postgresql libpq-dev sass curl build-essential -y && \
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y --no-modify-path && \
cargo install diesel_cli --no-default-features --features "postgres"

WORKDIR /app
COPY ./ /app

CMD diesel setup && \
	diesel migration run && \
	sass --sourcemap=none ./templates/style.scss ./static/css/style.css && \
	ROCKET_ENV=production cargo run --release
