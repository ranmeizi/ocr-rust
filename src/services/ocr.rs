// use opencv::prelude::Mat;
use crate::util::random_str::random_str;
use axum::body::Bytes;
use opencv::{
    core::{in_range, Mat, Scalar, Vector},
    imgcodecs::{self, imread, imwrite},
};
use tokio::process::{Command};

pub struct OCR {}

impl OCR {
    /** 二值化 */
    pub fn binarization(img_file: Bytes) -> Mat {
        // 读取图片 mat
        let data: Vector<u8> = Vector::from_iter(img_file);

        let mat = imgcodecs::imdecode(&data, imgcodecs::IMREAD_COLOR).unwrap();

        // 二值化
        let mut dst = Mat::default();

        in_range(
            &mat,
            &Scalar::new(77.0, 77.0, 77.0, 77.0),
            &Scalar::new(255.0, 255.0, 255.0, 255.0),
            &mut dst,
        );

        dst
    }

    /** 拿着二值的图片调用ocr */
    pub async fn ocr(dst: Mat) -> Result<String,Box<dyn std::error::Error>> {
        let filename = random_str(20) + ".jpg";

        // 保存阈值化后的图片
        imwrite(&filename, &dst, &Vector::new()).unwrap();

        let output = Command::new("tesseract")
            .arg("-l")
            .arg("chi_sim")
            .arg(format!("/app/{filename}"))
            // .arg(format!("/Users/boboan/code/work/self/ocr-rust/{filename}"))
            .arg("stdout")
            .output().await.unwrap();

        let outstr: String = String::from_utf8(output.stdout).unwrap();
        println!("{}",outstr);
        Ok(outstr)
    }

    /** 拿着ocr的结果调用get_pet_info */
    pub fn get_pet_info() {}

    /** 拿着ocr的结果调用get_stats_info  */
    pub fn get_stats_info() {}
}

#[cfg(test)]
mod tests {
    use opencv::{
        core,
        imgcodecs::{imread, imwrite},
        imgproc,
    };

    #[test]
    fn test_binarization() {
        // 读取图片
        let img = imread("ocr-test/hrl.jpg", 1).unwrap();

        // 二值化
        let mut dst = core::Mat::default();

        core::in_range(
            &img,
            &core::Scalar::new(77.0, 77.0, 77.0, 77.0),
            &core::Scalar::new(255.0, 255.0, 255.0, 255.0),
            &mut dst,
        );

        // 保存阈值化后的图片
        imwrite("threshold.jpg", &dst, &core::Vector::new()).unwrap();

        println!("{:?}", img);
    }

    fn test_binarization_from_binary() {}
}
