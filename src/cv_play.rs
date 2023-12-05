use crate::util::props_extractor::boxfile::TextPos;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::cv;
    use crate::util::props_extractor::boxfile;
    use crate::util::props_extractor::boxfile::TextPos;
    use anyhow;
    use opencv::{core, imgcodecs, imgproc, prelude::*};
    use tokio::runtime::Runtime;

    const RES_IMG_DIR: &str = "res_img/";
    const IMG_PATH_HRL: &str = "ocr-test/hrl.jpg";
    const IMG_PATH_HB: &str = "ocr-test/hb.jpg";
    const IMG_PATH_TLSYD: &str = "ocr-test/jb.jpg";
    const IMG_PATH_MN: &str = "ocr-test/mn.jpg";
    const IMG_PATH_BKR: &str = "ocr-test/bkr.jpg";

    #[test]
    fn pet_threshold() {
        let src = imgcodecs::imread(IMG_PATH_BKR, imgcodecs::IMREAD_GRAYSCALE).unwrap();

        // 应用阈值
        let threshold_value = 155.0;
        let max_binary_value = 255.0;

        let mut dst = Mat::default();

        imgproc::threshold(
            &src,
            &mut dst,
            threshold_value,
            max_binary_value,
            imgproc::THRESH_BINARY,
        ).unwrap();

        imgcodecs::imwrite(
            format!("{}threshold.jpg", RES_IMG_DIR).as_str(),
            &dst,
            &core::Vector::new(),
        )
        .unwrap();
    }
}
