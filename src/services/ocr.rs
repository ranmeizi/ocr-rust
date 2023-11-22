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
    use opencv::imgcodecs::imread;

    #[test]
    fn test_binarization() {
        // 读取图片
        let img = imread("ocr-test/hrl.jpg", 1).unwrap();

        // 二值化

        println!("{:?}", img);
    }
}
