# 先build出可执行文件

FROM rust

WORKDIR /app

# opencv 依赖
RUN apt install libopencv-dev clang libclang-dev

# build rust
RUN cargo build --release

# 安装 ocr 环境

FROM jitesoft/tesseract-ocr

RUN docker --version

# 启动 rust 服务

RUN /app/target/release/ocr-rust