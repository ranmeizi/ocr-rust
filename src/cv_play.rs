use crate::util::props_extractor::boxfile::TextPos;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::props_extractor::boxfile;
    use crate::{services::ocr::OCR, util::props_extractor::boxfile::TextPos};
    use anyhow;
    use cv_self::preprocessing;
    use opencv::{core, imgcodecs, imgproc, prelude::*};
    use tokio::runtime::Runtime;

    use super::SearchingFold;

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

        imgcodecs::imwrite("morphologt_ex.jpg", &dst, &core::Vector::new()).unwrap();
    }

    #[test]
    fn it_canny() {
        // 从文件中加载图像
        let img = imgcodecs::imread(IMG_PATH_HRL, imgcodecs::IMREAD_GRAYSCALE).unwrap();

        let mut edges = core::Mat::default();

        // 进行 Canny 边缘检测
        imgproc::canny(&img, &mut edges, 15.0, 20.0, 3, false).unwrap();

        imgcodecs::imwrite("canny.jpg", &edges, &core::Vector::new()).unwrap();
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

        imgcodecs::imwrite("res.jpg", &black, &core::Vector::new()).unwrap();
    }

    #[test]
    fn it_mod() {
        // 从文件中加载图像
        let src = imgcodecs::imread(IMG_PATH_HRL, imgcodecs::IMREAD_GRAYSCALE).unwrap();

        let dst = preprocessing::pet_info::run(src).unwrap();

        imgcodecs::imwrite(
            &format!("{}threshold.jpg", RES_IMG_DIR),
            &dst,
            &core::Vector::new(),
        )
        .unwrap();
    }

    #[test]
    fn it_mod1() {
        // 从文件中加载图像
        let src = imgcodecs::imread(IMG_PATH_BKR, imgcodecs::IMREAD_GRAYSCALE).unwrap();

        let dst = preprocessing::pet_info::threshold(src).unwrap();

        imgcodecs::imwrite(
            &format!("{}threshold.jpg", RES_IMG_DIR),
            &dst,
            &core::Vector::new(),
        )
        .unwrap();
    }

    #[test]
    fn it_harris() {
        // 从文件中载入图像
        let mut img = imgcodecs::imread(IMG_PATH_HRL, imgcodecs::IMREAD_GRAYSCALE).unwrap();
        let mut img_color = imgcodecs::imread(IMG_PATH_HRL, imgcodecs::IMREAD_COLOR).unwrap();

        // 进行 Harris 角点检测
        let mut dst = core::Mat::zeros(img.rows(), img.cols(), core::CV_32FC1)
            .unwrap()
            .to_mat()
            .unwrap();
        imgproc::corner_harris(&img, &mut dst, 2, 3, 0.04, core::BORDER_DEFAULT).unwrap();

        // 对结果进行归一化和缩放
        let mut dst_norm = core::Mat::default();
        let mut dst_norm_scaled = core::Mat::default();
        core::normalize(
            &dst,
            &mut dst_norm,
            0.0,
            255.0,
            core::NORM_MINMAX,
            core::CV_32FC1,
            &core::Mat::default(),
        )
        .unwrap();
        core::convert_scale_abs(&dst_norm, &mut dst_norm_scaled, 1.0, 0.0).unwrap();

        // 将检测到的角点绘制到原始图像上
        for j in 0..dst_norm.rows() {
            for i in 0..dst_norm.cols() {
                unsafe {
                    if dst_norm.at_2d_unchecked::<f32>(j, i).unwrap() > &100.0 {
                        imgproc::circle(
                            &mut img_color,
                            core::Point::new(i, j),
                            5,
                            core::Scalar::new(0.0, 0.0, 255.0, 0.0),
                            2,
                            8,
                            0,
                        )
                        .unwrap();
                    }
                }
            }
        }

        imgcodecs::imwrite(
            &format!("{}harris.jpg", RES_IMG_DIR),
            &img_color,
            &core::Vector::new(),
        )
        .unwrap();
    }

    #[test]
    fn it_panel_range() {
        // 从文件中载入图像
        let mut img = imgcodecs::imread(IMG_PATH_HRL, imgcodecs::IMREAD_COLOR).unwrap();

        // 将图像从 BGR 转换到 HSV
        let mut dst1 = core::Mat::default();

        imgproc::cvt_color(&img, &mut dst1, imgproc::COLOR_BGR2HSV, 0).unwrap();

        // 定义红色的 HSV 范围
        let lower_red = core::Scalar::new(0.0, 120.0, 70.0, 0.0);
        let upper_red = core::Scalar::new(10.0, 255.0, 255.0, 0.0);

        // 创建一个掩模（mask），只显示红色区域
        let mut mask = core::Mat::default();
        core::in_range(&dst1, &lower_red, &upper_red, &mut mask).unwrap();

        let mut dst2 = core::Mat::default();

        // 将掩模应用到原始图像
        core::bitwise_and(&dst1, &dst1, &mut dst2, &mask).unwrap();

        imgcodecs::imwrite(
            &format!("{}red.jpg", RES_IMG_DIR),
            &dst2,
            &core::Vector::new(),
        )
        .unwrap();
    }

    #[test]
    fn it_panel_range2() {
        // 从文件中载入图像
        let mut img = imgcodecs::imread(IMG_PATH_HRL, imgcodecs::IMREAD_GRAYSCALE).unwrap();

        // 创建一个 5x5 的结构元素
        let kernel = imgproc::get_structuring_element(
            imgproc::MORPH_RECT,
            core::Size::new(7, 7),
            core::Point::new(-1, -1),
        )
        .unwrap();

        // 对图像进行开操作
        let mut dst = core::Mat::default();
        imgproc::morphology_ex(
            &img,
            &mut dst,
            imgproc::MORPH_OPEN,
            &kernel,
            core::Point::new(-1, -1),
            2,
            core::BORDER_CONSTANT,
            imgproc::morphology_default_border_value().unwrap(),
        )
        .unwrap();

        imgcodecs::imwrite(
            &format!("{}res.jpg", RES_IMG_DIR),
            &dst,
            &core::Vector::new(),
        )
        .unwrap();
    }

    #[test]
    fn range_hrl() {
        // 从文件中载入图像
        let mut img = imgcodecs::imread(IMG_PATH_HRL, imgcodecs::IMREAD_GRAYSCALE).unwrap();

        let _res = cv_self::preprocessing::pet_info::threshold(img).unwrap();

        imgcodecs::imwrite("res.jpg", &_res, &core::Vector::new()).unwrap();
    }

    #[test]
    fn container_draw_text() {
        println!("hello");
        let mut rt = Runtime::new().unwrap();
        rt.block_on(async {
            // 在这里写你的 async 代码
            // 从文件中载入图像
            let mut img = imgcodecs::imread(IMG_PATH_HRL, imgcodecs::IMREAD_GRAYSCALE).unwrap();

            // ocr
            let _res = cv_self::preprocessing::pet_info::threshold(img).unwrap();

            let res = OCR::ocr_pos(_res).await.unwrap();

            let res = boxfile::run(res.as_str());
            let mut img = imgcodecs::imread(IMG_PATH_HRL, 1).unwrap();

            let size = img.size().unwrap();

            let core::Size { width, height } = size;

            // 画图
            for pos in res.iter() {
                let TextPos {
                    text,
                    left,
                    right,
                    top,
                    bottom,
                } = pos;

                let mut rect =
                    core::Rect::new(*left, height - *top, *right - *left, *top - *bottom);

                imgproc::rectangle(
                    &mut img,
                    rect,
                    core::Scalar::new(0.0, 0.0, 255.0, 0.0),
                    1,
                    imgproc::LINE_AA,
                    0,
                )
                .unwrap();
            }
            imgcodecs::imwrite("nb.jpg", &img, &core::Vector::new()).unwrap();
        });
    }

    #[tokio::test]
    async fn cdt1() {
        async fn run(input: &str, output: &str) {
            // 在这里写你的 async 代码
            // 从文件中载入图像
            let mut img = imgcodecs::imread(input, imgcodecs::IMREAD_GRAYSCALE).unwrap();

            // ocr
            let _res = cv_self::preprocessing::pet_info::threshold(img).unwrap();

            let res = OCR::ocr_pos(_res).await.unwrap();

            let res = boxfile::run(res.as_str());

            // 找"转" 后第一个 “总”，和 “元素”
            let init: SearchingFold = SearchingFold {
                string: String::new(),
                last_char: None,
                target_zhuan: None,
                target_yuan: None,
            };

            let SearchingFold {
                target_yuan,
                target_zhuan,
                ..
            } = res
                .iter()
                .fold(init, |state, curr| match curr.text.as_str() {
                    "转" => handlers::zhuan(state, curr.clone()),
                    "素" => handlers::su(state, curr.clone()),
                    _ => handlers::default_run(state, curr),
                });

            let mut img = imgcodecs::imread(input, 1).unwrap();

            let size = img.size().unwrap();

            let core::Size { width, height } = size;

            let zhuan = target_zhuan.unwrap();
            let yuan = target_yuan.unwrap();
            // 画图
            let top = zhuan.bottom;
            let left = yuan.left;
            let bottom = yuan.top;

            let h = top - bottom;

            let w = (2.1 * h as f64).round() as i32;

            println!("xywh:{:?}", (left, height - top, w, h));

            let mut rect = core::Rect::new(left, height - top, w, h);

            let roi = Mat::roi(&img, rect).unwrap();

            imgcodecs::imwrite(
                format!("ocr-test/{}", output).as_str(),
                &roi,
                &core::Vector::new(),
            )
            .unwrap();
        }

        run(IMG_PATH_HRL, "红人龙.jpg").await;
        run(IMG_PATH_BKR, "柏克尔.jpg").await;
        run(IMG_PATH_MN, "马年.jpg").await;
        run(IMG_PATH_TLSYD, "机暴.jpg").await;
    }

    #[test]
    fn run_fn() {
        let txt = "~ 331 371 726 379 0\n~ 321 230 736 639 0\n~ 416 96 693 116 0\n~ 433 65 695 96 0\n红 145 84 179 116 0\n魔 181 82 257 116 0\n8 225 72 243 120 0\nS 243 72 258 120 0\nP 270 85 289 111 0\nr 292 85 302 104 0\no 304 82 358 116 0\n和 321 72 336 120 0\n所 336 72 366 120 0\n锋 369 89 386 110 0\n这 399 84 416 103 0\n~ 433 55 649 65 0\n4 1063 366 1102 397 0\nA 1168 335 1191 359 0\nP 1195 326 1215 353 0\n转 1768 855 1795 882 0\n生 1798 855 1854 882 0\n前 1835 842 1858 886 0\n总 1588 799 1611 823 0\n成 1625 799 1655 823 0\n长 1654 788 1670 827 0\n( 1734 798 1738 821 0\n3 1741 802 1753 821 0\n. 1757 802 1760 805 0\n1 1765 802 1775 821 0\n9 1779 802 1790 821 0\n4 1793 803 1806 821 0\n` 1813 810 1817 813 0\n5 1823 802 1834 821 0\n. 1839 803 1842 805 0\n3 1845 802 1857 821 0\n2 1861 802 1872 821 0\n5 1875 803 1886 821 0\n) 1891 799 1895 820 0\n生 1589 756 1613 780 0\n命 1616 756 1640 780 0\n成 1655 756 1688 780 0\n长 1687 745 1723 785 0\n【 1734 756 1737 779 0\n0 1741 759 1753 778 0\n. 1757 759 1775 778 0\n9 1770 755 1783 779 0\n4 1778 759 1791 778 0\n2 1794 759 1804 778 0\n- 1811 767 1819 770 0\n1 1825 759 1834 778 0\n. 1839 759 1842 762 0\n5 1846 759 1857 778 0\n7 1860 759 1872 778 0\n1 1877 759 1886 778 0\n) 1891 755 1895 777 0\n攻 1589 713 1613 736 0\n击 1616 713 1640 737 0\n成 1654 712 1687 736 0\n长 1686 704 1713 741 0\n( 1734 713 1738 736 0\n1 1742 716 1753 735 0\n. 1757 716 1760 719 0\n1 1765 716 1775 735 0\n6 1779 716 1790 735 0\n7 1794 716 1805 735 0\n~ 1811 724 1819 727 0\n1 1824 716 1834 735 0\n. 1839 716 1842 719 0\n9 1845 716 1857 735 0\n4 1860 716 1872 735 0\n5 1875 716 1887 735 0\n) 1891 713 1895 733 0\n防 1590 669 1613 693 0\n咎 1624 669 1640 693 0\n成 1651 669 1688 693 0\n长 1687 661 1712 697 0\n( 1734 671 1737 691 0\n0 1741 673 1753 692 0\n. 1757 673 1759 676 0\n3 1764 673 1775 692 0\n4 1778 673 1790 692 0\n7 1794 673 1805 692 0\n- 1809 681 1820 684 0\n0 1823 673 1835 692 0\n. 1839 673 1841 676 0\n5 1846 673 1857 692 0\n7 1861 673 1872 692 0\n9 1875 673 1887 692 0\n) 1891 669 1895 691 0\n敏 1589 626 1613 650 0\n捷 1616 626 1640 650 0\n成 1653 626 1667 650 0\n长 1680 626 1693 650 0\n( 1749 625 1753 649 0\n0 1756 630 1768 649 0\n. 1772 630 1775 632 0\n7 1779 630 1790 648 0\n3 1794 630 1805 649 0\n8 1808 630 1820 649 0\n` 1825 639 1829 641 0\n1 1839 630 1849 648 0\n. 1854 630 1856 632 0\n2 1860 630 1872 649 0\n3 1876 630 1886 649 0\n) 1891 628 1895 648 0\n人 1593 511 1613 528 0\n国 1636 508 1644 527 0\n午 1658 506 1667 527 0\n介 1679 508 1689 527 0\n用 1699 507 1711 527 0\n山 1724 508 1734 527 0\n导 1746 507 1756 527 0\n用 1765 507 1778 527 0\n而 1787 507 1801 527 0\n人 1868 500 1894 528 0\n目 1911 500 1946 527 0\n是 1936 496 1950 532 0\n~ 1584 258 1724 460 0\n获 1792 148 1821 178 0\n取 1824 148 1854 177 0\n途 1864 147 1899 178 0\n径 1899 148 1920 176 0\n尼 1969 931 2034 958 0\n谎 2010 915 2038 976 0\n转 1997 855 2024 882 0\n生 2027 855 2083 882 0\n后 2064 842 2087 886 0\n( 1963 798 1967 820 0\n3 1971 802 1982 821 0\n. 1986 802 2004 821 0\n6 1999 798 2011 821 0\n7 2008 803 2019 821 0\n2 2023 802 2033 821 0\n- 2038 811 2049 814 0\n7 2052 803 2064 821 0\n. 2068 803 2071 805 0\n4 2074 803 2085 821 0\n5 2090 802 2101 821 0\n4 2104 802 2116 821 0\n) 2120 799 2124 821 0\n( 1963 758 1967 780 0\n1 1972 759 1981 778 0\n. 1987 759 1989 762 0\n0 1993 759 2004 778 0\n8 2008 759 2019 778 0\n3 2023 759 2034 778 0\n~ 2038 767 2048 770 0\n2 2053 759 2064 778 0\n. 2068 759 2071 762 0\n1 2076 759 2086 778 0\n9 2089 759 2101 778 0\n9 2104 759 2116 778 0\n) 2120 755 2124 779 0\n( 1963 712 1967 733 0\n1 1972 716 1982 735 0\n. 1986 716 1989 719 0\n3 1993 716 2004 735 0\n4 2007 716 2019 735 0\n2 2022 716 2034 735 0\n~ 2039 724 2046 727 0\n2 2052 716 2064 735 0\n. 2068 716 2070 719 0\n7 2075 716 2086 735 0\n2 2090 716 2101 735 0\n3 2105 716 2116 735 0\n) 2120 712 2124 735 0\n( 1978 668 1982 693 0\n0 1985 673 1997 692 0\n. 2001 673 2004 676 0\n3 2008 673 2019 692 0\n9 2023 673 2034 692 0\n9 2037 673 2049 692 0\n- 2054 681 2063 684 0\n0 2067 673 2079 692 0\n. 2083 673 2086 676 0\n8 2090 673 2101 692 0\n1 2106 673 2115 691 0\n) 2120 670 2124 693 0\n( 1963 625 1967 650 0\n0 1970 630 1982 649 0\n. 1987 630 2004 649 0\n8 1999 625 2011 650 0\n4 2007 630 2019 649 0\n8 2023 630 2034 649 0\n- 2039 639 2044 641 0\n1 2054 630 2063 648 0\n. 2068 630 2071 632 0\n7 2075 630 2086 649 0\n2 2090 630 2100 648 0\n2 2104 630 2116 649 0\n) 2120 626 2124 648 0\n";

        let res = boxfile::run(txt);

        // 找"转" 后第一个 “总”，和 “元素”
        let init: SearchingFold = SearchingFold {
            string: String::new(),
            last_char: None,
            target_zhuan: None,
            target_yuan: None,
        };

        let SearchingFold {
            target_yuan,
            target_zhuan,
            ..
        } = res
            .iter()
            .fold(init, |state, curr| match curr.text.as_str() {
                "转" => handlers::zhuan(state, curr.clone()),
                "素" => handlers::su(state, curr.clone()),
                _ => handlers::default_run(state, curr),
            });

        let mut img = imgcodecs::imread(IMG_PATH_BKR, 1).unwrap();

        let size = img.size().unwrap();

        let core::Size { width, height } = size;

        let zhuan = target_zhuan.unwrap();
        let yuan = target_yuan.unwrap();
        // 画图
        let top = zhuan.bottom;
        let left = yuan.left;
        let bottom = yuan.top;

        let h = top - bottom;

        let w = (2.1 * h as f64).round() as i32;

        println!("xywh:{:?}", (left, height - top, w, h));

        let mut rect = core::Rect::new(left, height - top, w, h);

        let roi = Mat::roi(&img, rect).unwrap();

        imgcodecs::imwrite(
            format!("ocr-test/{}", "bkr.jpg").as_str(),
            &roi,
            &core::Vector::new(),
        )
        .unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct SearchingFold {
    // 全部文字的线
    string: String,
    // 上一个文字
    last_char: Option<TextPos>,

    //
    target_zhuan: Option<TextPos>,

    target_yuan: Option<TextPos>,
}

struct handlers {}

impl handlers {
    fn default_run(state: SearchingFold, curr: &TextPos) -> SearchingFold {
        let char = &curr.text;
        let mut state = state;
        state.last_char = Some(curr.clone());
        state.string.push_str(char);
        state
    }

    fn zhuan(mut state: SearchingFold, curr: TextPos) -> SearchingFold {
        if state.target_zhuan.is_none() {
            state.target_zhuan = Some(curr.clone());
        }

        let mut state = handlers::default_run(state, &curr);

        state
    }

    fn su(mut state: SearchingFold, curr: TextPos) -> SearchingFold {
        println!("su: {:?}", state);
        if state.target_yuan.is_none() {
            if let Some(last) = &state.last_char {
                println!("确实是啊");
                if last.text.as_str() == "元" {
                    state.target_yuan = Some(last.clone());
                }
            }
        }
        let mut state = handlers::default_run(state, &curr);

        state
    }
}
