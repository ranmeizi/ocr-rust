FROM rust

COPY ocr-test/* /tmp/

# 安装 ocr 环境
RUN apt-get update ; apt-get install -y tesseract-ocr

# 安装 opencv 环境
RUN apt-get update ; apt-get install -y clang libopencv-dev libclang-dev
