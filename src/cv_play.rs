mod tests {
    use opencv::{core, imgcodecs, imgproc, prelude::*};

    const IMG_PATH_HRL: &str = "ocr-test/hrl.jpg";

    #[test]
    fn it_gradients() {
        let src = imgcodecs::imread(IMG_PATH_HRL, imgcodecs::IMREAD_GRAYSCALE).unwrap();

        let mut grad_x = core::Mat::default();
        let mut grad_y = core::Mat::default();

        // 计算 X 方向的梯度
        imgproc::sobel(
            &src,
            &mut grad_x,
            core::CV_32F,
            1,
            0,
            3,
            1.0,
            0.0,
            core::BORDER_DEFAULT,
        )
        .unwrap();

        // 计算 Y 方向的梯度
        imgproc::sobel(
            &src,
            &mut grad_y,
            core::CV_32F,
            0,
            1,
            3,
            1.0,
            0.0,
            core::BORDER_DEFAULT,
        )
        .unwrap();

        // 保存结果
        imgcodecs::imwrite("grad_x.jpg", &grad_x, &core::Vector::new()).unwrap();
        imgcodecs::imwrite("grad_y.jpg", &grad_y, &core::Vector::new()).unwrap();
    }

    #[test]
    fn it_roi() {
        // 从文件中加载图像
        let mut img = imgcodecs::imread(IMG_PATH_HRL, imgcodecs::IMREAD_COLOR).unwrap();

        // 创建一个Rect对象，它定义了你想要的ROI
        // 参数是：x, y, 宽度, 高度
        let rect = core::Rect::new(50, 50, 100, 100);

        // 使用Rect获取图像的ROI
        let roi = Mat::roi(&img, rect).unwrap();

        imgcodecs::imwrite("roi.jpg", &roi, &core::Vector::new()).unwrap();
    }

    #[test]
    fn it_morphologt_ex() {
        // 从文件中加载图像
        let img = imgcodecs::imread(IMG_PATH_HRL, 1).unwrap();

        // 获取用于边界插补的默认值
        let border_value = imgproc::morphology_default_border_value().unwrap();

        // 创建用于形态学操作的结构元素
        let element = imgproc::get_structuring_element(
            imgproc::MORPH_RECT,
            core::Size::new(25, 25),
            core::Point::new(-1, -1),
        )
        .unwrap();

        let mut dst = core::Mat::default();

        // 执行形态学膨胀操作
        imgproc::morphology_ex(
            &img,
            &mut dst,
            imgproc::MORPH_DILATE,
            &element,
            core::Point::new(-1, -1),
            1,
            core::BORDER_CONSTANT,
            border_value,
        )
        .unwrap();

        imgcodecs::imwrite("morphologt_ex.jpg", &dst, &core::Vector::new()).unwrap();
    }
}
