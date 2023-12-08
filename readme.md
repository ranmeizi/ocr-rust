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
- [x] 裁剪出需要识别文字的区域
- [x] 图片阈值处理
- [x] 子进程调用ocr，拿到stdout数据
- [x] 识别出的 text 的结果处理

## TODO

- [ ] 前端图片压缩，后端限制图片大小
- [x] 识别图鉴图像特征，截取对应区域
- [ ] 前端照片红框，后端对应接受图片/红框进行对应区域识别
- [ ] 倾斜的图片识别  
