FROM boboan/ocr-rust-env

# 安装 vim
RUN apt-get install -y vim

# 复制代码
COPY . /app/

# 启动 app 服务
WORKDIR /app

CMD cargo run