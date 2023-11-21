FROM rust

WORKDIR /app

COPY Cargo.toml /app

COPY . /app

ENV RUSTUP_DIST_SERVER "https://mirrors.ustc.edu.cn/rust-static"

ENV RUSTUP_UPDATE_ROOT "https://mirrors.ustc.edu.cn/rust-static/rustup"

CMD ["cargo","run"]