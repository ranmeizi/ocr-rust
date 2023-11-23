# 创建镜像
docker build -t ocr-rust-build . -f build.Dockerfile

# 运行构建 关联文件
docker run -i -v $PWD/pre-build:/build/pre-build ocr-rust-build

# # 删除container
# docker rm -f ocr-build-container

# # 删除容器
# docker rmi -f ocr-rust-build