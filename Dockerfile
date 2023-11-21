FROM rust

WORKDIR /app

COPY Cargo.toml /app

COPY . /app

CMD ["cargo","run"]