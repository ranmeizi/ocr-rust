FROM boboan/ocr-rust-env

# 复制代码
COPY . /build/

# 构建 app
WORKDIR /build

RUN cargo build --release

CMD mv ./target/release/ocr-rust ./pre-build
