FROM boboan/ocr-rust-env

# 复制代码
COPY . /app/

# 启动 app 服务
WORKDIR /app

RUN mkdir -p /root/.cargo 
RUN echo '[source.crates-io]\n\
replace-with = "ustc"\n\
[source.ustc]\n\
registry = "https://mirrors.ustc.edu.cn/crates.io-index"' > /root/.cargo/config

RUN cargo build --release

CMD /app/target/release/ocr-rust