FROM rust

WORKDIR /app

COPY cargo.toml /app

COPY . /app

CMD ["cargo","run"]