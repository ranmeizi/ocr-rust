FROM boboan/ocr-rust-env

# 复制代码
COPY . /app/

# 启动 app 服务
WORKDIR /app

# RUN mkdir -p /root/.cargo 
# RUN echo '[source.crates-io]\n\
# replace-with = "tuna"\n\
# [source.tuna]\n\
# registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"' > /root/.cargo/config

RUN cargo build --release

CMD /app/target/release/ocr-rust