mod tests {
    use crate::preprocessing;
    use opencv::{core, imgcodecs, imgproc, prelude::*};

    const RES_IMG_DIR: &str = "res_img/";
    const IMG_PATH_HRL: &str = "ocr-test/hrl.jpg";
    const IMG_PATH_TLSYD: &str = "ocr-test/jb.jpg";
    const IMG_PATH_MN: &str = "ocr-test/mn.jpg";
    const IMG_PATH_BKR: &str = "ocr-test/bkr.jpg";

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
        imgcodecs::imwrite(RES_IMG_DIR + "grad_x.jpg", &grad_x, &core::Vector::new()).unwrap();
        imgcodecs::imwrite(RES_IMG_DIR + "grad_y.jpg", &grad_y, &core::Vector::new()).unwrap();
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

        imgcodecs::imwrite(RES_IMG_DIR + "roi.jpg", &roi, &core::Vector::new()).unwrap();
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
            core::Size::new(5, 5),
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

        imgcodecs::imwrite(
            RES_IMG_DIR + "morphologt_ex.jpg",
            &dst,
            &core::Vector::new(),
        )
        .unwrap();
    }

    #[test]
    fn it_canny() {
        // 从文件中加载图像
        let img = imgcodecs::imread(IMG_PATH_HRL, imgcodecs::IMREAD_GRAYSCALE).unwrap();

        let mut edges = core::Mat::default();

        // 进行 Canny 边缘检测
        imgproc::canny(&img, &mut edges, 15.0, 20.0, 3, false).unwrap();

        imgcodecs::imwrite(RES_IMG_DIR + "canny.jpg", &edges, &core::Vector::new()).unwrap();
    }

    fn get_top_left(mid: (i32, i32), width: i32, height: i32) -> (i32, i32) {
        let (x, y) = mid;

        let left = x - width / 2;
        let top = y - height / 2;

        (left, top)
    }

    #[test]
    fn middle_line() {
        // 从文件中加载图像
        let mut src = imgcodecs::imread(IMG_PATH_TLSYD, 1).unwrap();
        let size = src.size().unwrap();

        let core::Size { width, height } = size;

        let mid = width / 2;

        let vh = height / 100;
        let name_mid_offset = 6 * vh;

        let left = mid - name_mid_offset;

        // 在 src 上的 x = mid 处 ,从 0 ~ height 拉一条红线
        imgproc::line(
            &mut src,
            core::Point::new(left, 0),
            core::Point::new(left, height),
            core::Scalar::new(0.0, 0.0, 255.0, 0.0),
            1,
            imgproc::LINE_AA,
            0,
        );

        // 画文字水平中心线

        let top = 18 * vh;
        imgproc::line(
            &mut src,
            core::Point::new(0, top),
            core::Point::new(width, top),
            core::Scalar::new(0.0, 0.0, 255.0, 0.0),
            1,
            imgproc::LINE_AA,
            0,
        );

        // 画 name 区域的 rect

        let rect_h = 7 * vh;
        let rect_w = 35 * vh;

        let (left, top) = get_top_left((left, top), rect_w, rect_h);

        let name_rect = core::Rect::new(left, top, rect_w, rect_h);

        imgproc::rectangle(
            &mut src,
            name_rect,
            core::Scalar::new(0.0, 0.0, 255.0, 0.0),
            1,
            imgproc::LINE_AA,
            0,
        );

        // 找成长面板画 rect

        let panel_h = 27 * vh;
        let panel_w = 52 * vh;

        let panel_top = 20 * vh;
        let panel_left = mid + 30 * vh;

        let panel_rect = core::Rect::new(panel_left, panel_top, panel_w, panel_h);

        imgproc::rectangle(
            &mut src,
            panel_rect,
            core::Scalar::new(0.0, 255.0, 0.0, 0.0),
            1,
            imgproc::LINE_AA,
            0,
        );

        // 裁剪
        // 使用Rect获取图像的ROI
        let mut roi_name = Mat::roi(&src, name_rect).unwrap();

        let mut roi_panel = Mat::roi(&src, panel_rect).unwrap();

        let mut black = core::Mat::zeros(src.rows(), src.cols(), core::CV_8UC3)
            .unwrap()
            .to_mat()
            .unwrap();

        // 从原图像中拷贝感兴趣的区域到全黑图像中
        roi_name
            .copy_to(&mut core::Mat::roi(&black, name_rect).unwrap())
            .unwrap();
        roi_panel
            .copy_to(&mut core::Mat::roi(&black, panel_rect).unwrap())
            .unwrap();

        imgcodecs::imwrite(RES_IMG_DIR + "res.jpg", &black, &core::Vector::new()).unwrap();
    }

    #[test]
    fn it_mod() {
        // 从文件中加载图像
        let src = imgcodecs::imread(IMG_PATH_BKR, imgcodecs::IMREAD_GRAYSCALE).unwrap();

        let dst = preprocessing::pet_info::run(src).unwrap();

        imgcodecs::imwrite(RES_IMG_DIR + "threshold.jpg", &dst, &core::Vector::new()).unwrap();
    }
}
