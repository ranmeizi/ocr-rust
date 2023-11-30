FROM rust

COPY ocr-test/* /tmp/

COPY ocr-test/chi_sim.traineddata /usr/local/share/tessdata/ 
COPY ocr-test/chi_sim_vert.traineddata /usr/local/share/tessdata/ 

# 安装 ocr 环境
RUN apt-get update ; apt-get install -y tesseract-ocr

# 学习数据
ENV TESSDATA_PREFIX=/usr/local/share/tessdata

# 安装 opencv 环境
RUN apt-get update ; apt-get install -y clang libopencv-dev libclang-dev
