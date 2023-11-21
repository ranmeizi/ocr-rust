FROM rust

COPY ocr-test/* /tmp/

# 安装 ocr 环境
RUN apt-get update ; apt-get install -y tesseract-ocr

RUN tesseract /tmp/hrl1.jpg stdout

# 启动 app 服务
WORKDIR /app

COPY pre-build/ocr-rust /app/

CMD /app/ocr-rust