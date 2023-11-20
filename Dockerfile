# 先build出可执行文件

FROM rust:1.67

WORKDIR /app

# opencv 依赖
RUN apt update && apt upgrade && apt install libopencv-dev clang libclang-dev

# build rust
RUN cargo build --release

# 安装 ocr 环境

FROM jitesoft/tesseract-ocr:5.3.3

# 启动 rust 服务

RUN /app/target/release/ocr-rust