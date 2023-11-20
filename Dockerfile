# 先build出可执行文件

FROM rust

# opencv 依赖
# RUN apt install libopencv-dev clang libclang-dev

RUN pwd

RUN ls

# build rust
RUN cargo build --release

# 安装 ocr 环境

# FROM jitesoft/tesseract-ocr

# 启动 rust 服务

RUN ./target/release/ocr-rust