# ocr-rust

使用 opencv 和 tesseract-ocr 搭建环境

axum 作为服务端

开发的一个 ocr 服务

## 环境

目前是运行在微信云托管上的

[环境](./env.Dockerfile):rust+opencv+tesseract
[构建](./build/build.sh):在容器内构建对应系统下的rust服务，拿到可执行文件
[部署](./Dockerfile): 云托管的部署dockerfile

## 开发

思路

- [x] 小程序客户端上传文件至oss，使用fileid请求rust的ocr服务
- [x] rust 服务使用 fileid 获取图片文件
- [ ] 裁剪出需要识别文字的区域 TODO
- [x] 图片阈值处理
- [x] 子进程调用ocr，拿到stdout数据
- [ ] 识别出的 text 的结果处理 TODO
