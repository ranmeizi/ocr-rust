FROM boboan/ocr-rust-env

# 复制代码
COPY . /app/

# 启动 app 服务
WORKDIR /app

RUN cargo build --release

CMD /app/target/release/ocr-rust