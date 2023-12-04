use crate::{
    services::tesseract,
    util::props_extractor::{boxfile, pet_info},
};
use anyhow;
use opencv::{imgproc, prelude::*};

/**
 * 宠物用的二值化
 */
pub fn pet_threshold(src: &Mat) -> anyhow::Result<Mat> {
    let mut dst = Mat::default();

    imgproc::threshold(src, &mut dst, 164.0, 255.0, imgproc::THRESH_BINARY)?;

    Ok(dst)
}

pub struct PetInfo {}

impl PetInfo {
    // 截取宠物成长区域
    async fn get_pet_growth_area(src: &Mat) -> anyhow::Result<Mat> {
        let res = pet_threshold(src)?;

        let ocr_txt = tesseract::ChiSim::ocr_pos(&res).await?;

        let res = boxfile::run(ocr_txt.as_str());

        // 图片 size
        let size = src.size()?;

        // 截取区域 size
        let growth_size = pet_info::get_text_pos_vec(res, size);

        // 截取 区域
        let roi = Mat::roi(src, growth_size)?;

        Ok(roi)
    }
}
