use anyhow::{self, Ok};
use opencv::{
    photo,
    core::{self, Mat},
    imgproc,
    prelude::*,
};

pub fn threshold(src: Mat) -> anyhow::Result<Mat> {
    // 应用阈值
    let threshold_value = 164.0;
    let max_binary_value = 255.0;

    let mut dst = Mat::default();

    imgproc::threshold(
        &src,
        &mut dst,
        threshold_value,
        max_binary_value,
        imgproc::THRESH_BINARY,
    )
    .unwrap();

    // imgproc::adaptive_threshold(
    //     &src,
    //     &mut dst,
    //     255.0,
    //     imgproc::ADAPTIVE_THRESH_GAUSSIAN_C,
    //     imgproc::THRESH_BINARY_INV,
    //     5,
    //     23.0,
    // )?;

    return Ok(dst);

    // let mut denoised = Mat::default();
    // photo::fast_nl_means_denoising(&dst, &mut denoised, 15.0, 7, 21)?;


    // return Ok(denoised);

    // // 滤波
    // let mut kernel = Mat::default();

    // imgproc::laplacian(&dst, &mut kernel, -1, 5, -2.0, 0.0, core::BORDER_DEFAULT)?;

    // let mut aaa = Mat::default();

    //  // 将原图像和滤波器的结果相加，得到锐化的图像
    //  core::add(&dst, &kernel, &mut aaa, &core::no_array(), -1)?;

    // return Ok(denoised);
}

pub fn run(src: Mat) -> anyhow::Result<Mat> {
    // 应用阈值
    let dst = threshold(src)?;

    // 滤波
    let mut kernel = Mat::default();

    imgproc::laplacian(&dst, &mut kernel, -1, 3, -2.0, 0.0, core::BORDER_DEFAULT)?;

    let mut aaa = Mat::default();

    // 将原图像和滤波器的结果相加，得到锐化的图像
    core::add(&dst, &kernel, &mut aaa, &core::no_array(), -1)?;

    // 截图
    let size = dst.size()?;
    let core::Size { width, height } = size;

    // 名称区域的 rect
    let (_left, _top, _width, _height) = get_name_ract(width, height);
    let name_rect = core::Rect::new(_left, _top, _width, _height);

    println!("name_rect: {:?}", name_rect);

    // 成长面板 rect
    let (_left, _top, _width, _height) = get_panel_ract(width, height);
    let panel_rect = core::Rect::new(_left, _top, _width, _height);

    println!("panel_rect: {:?}", panel_rect);

    // 裁剪
    // 使用Rect获取图像的ROI
    let roi_name = Mat::roi(&dst, name_rect)?;

    let roi_panel = Mat::roi(&dst, panel_rect)?;

    // 黑色图像
    let black = core::Mat::zeros(dst.rows(), dst.cols(), core::CV_8U)?.to_mat()?;

    // 从原图像中拷贝感兴趣的区域到全黑图像中
    roi_name.copy_to(&mut core::Mat::roi(&black, name_rect)?)?;
    roi_panel.copy_to(&mut core::Mat::roi(&black, panel_rect)?)?;

    Ok(black)
}

fn get_top_left(mid: (i32, i32), width: i32, height: i32) -> (i32, i32) {
    let (x, y) = mid;

    let left = x - width / 2;
    let top = y - height / 2;

    (left, top)
}

fn get_name_ract(width: i32, height: i32) -> (i32, i32, i32, i32) {
    let vh = height / 100;

    let mid = width / 2; // 中轴线

    // 名称水平中心
    let mid_top = 18 * vh;

    // 名称垂直对称中心线
    let mid_left = mid - 6 * vh;

    // 名称区域的宽高
    let rect_h = 7 * vh;
    let rect_w = 35 * vh;

    let (left, top) = get_top_left((mid_left, mid_top), rect_w, rect_h);

    (left, top, rect_w, rect_h)
}

fn get_panel_ract(width: i32, height: i32) -> (i32, i32, i32, i32) {
    let vh = height / 100;

    let mid = width / 2; // 中轴线

    let top = 20 * vh;

    let left = mid + 30 * vh;

    println!("??.height{},width:{}", height, width);

    // 名称区域的宽高
    let rect_h = 27 * vh;
    let rect_w = 52 * vh;

    (left, top, rect_w, rect_h)
}
