FROM rust

WORKDIR /app

COPY . /app

CMD ["cargo","run"]