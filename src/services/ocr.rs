// use opencv::prelude::Mat;
use opencv::imgcodecs::imread;


pub struct OCR {}

impl OCR {
    /** 二值化 */
    pub fn binarization() {
        // 读取图片
        let img = imread("ocr-test/hrl.jpg", 1).unwrap();
    }

    /** 拿着二值的图片调用ocr */
    pub fn ocr() {}

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

        core::in_range(&img, &core::Scalar::new(77.0, 77.0, 77.0, 77.0), &core::Scalar::new(255.0, 255.0, 255.0, 255.0), &mut dst);

        // 保存阈值化后的图片
        imwrite("threshold.jpg", &dst, &core::Vector::new()).unwrap();

        println!("{:?}", img);
    }

    fn test_binarization_from_binary(){
        
    }
}
