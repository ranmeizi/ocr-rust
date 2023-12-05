use crate::{
    services::cv::PetInfoService,
    util::{
        props_extractor::pet_info::{self, PetInfoDto},
        random_str::random_str,
    },
    error::OcrErr
};
use anyhow::{self};
use axum::body::Bytes;
use opencv::{
    core::Vector,
    imgcodecs::{self, imwrite},
    prelude::*,
};
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::{fs, process::Command};

// ⬇ service
pub struct TesseractService {}

impl TesseractService {
    pub async fn get_mat_from_oss(fileid: String) -> anyhow::Result<Mat> {
        let url = TesseractService::get_img_from_cloud(fileid).await?;

        let bytes = TesseractService::download_url(url).await?;

        let mat = TesseractService::get_mat_from_bytes(bytes).unwrap();

        Ok(mat)
    }

    pub async fn get_img_from_cloud(fileid: String) -> anyhow::Result<String> {
        println!("get_img_from_cloud, {}", fileid);
        let client = Client::new();
        let res = client
            .post("http://api.weixin.qq.com/tcb/batchdownloadfile")
            .body(
                json!({
                    "env":"prod-7g2acur2ee3f2b4e",
                    "file_list":[{
                        "fileid":fileid,
                        "max_age":60*5
                    }]
                })
                .to_string(),
            )
            .send()
            .await?
            .json::<BatchDownloadFileRes>()
            .await?;
        let download_url = &res.file_list[0].download_url;

        Ok(download_url.to_owned())
    }

    pub async fn download_url(url: String) -> anyhow::Result<Bytes> {
        let response = reqwest::get(url).await?;

        // 创建一个文件来保存图片
        let mut bytes = response.bytes().await?;

        Ok(bytes)
    }

    pub fn get_mat_from_bytes(bytes: Bytes) -> anyhow::Result<Mat> {
        let data: Vector<u8> = Vector::from_iter(bytes);

        let mat = imgcodecs::imdecode(&data, imgcodecs::IMREAD_GRAYSCALE).unwrap();

        Ok(mat)
    }

    /**
     * 使用灰度的原图
     */
    pub async fn ocr_pet_growth(src: Mat) -> anyhow::Result<pet_info::PetInfoDto> {
        // 获取 成长数据 区域
        let area = PetInfoService::get_pet_growth_area(src.clone()).await?;

        // 英文识别
        let txt = Eng::ocr(area.clone()).await?;

        // 提取关键字
        let growth_data = pet_info::get_pet_info(&txt);

        Ok(growth_data)
    }

}

#[derive(Debug, Serialize)]
pub struct PetInfoDto1 {
    name: String,
}

// ⬇ logic

struct OSS {}

impl OSS {}

// 使用临时文件 调用 tesseract 识别
async fn run_cmd<F>(src: Mat, cmd: F) -> anyhow::Result<String>
where
    F: FnOnce(&str) -> Command,
{
    let filename = random_str(20) + ".jpg";

    // 保存图片
    imwrite(&filename, &src, &Vector::new()).unwrap();

    let res = cmd(&filename).output().await;

    let res = match res {
        Ok(res) => {
            let outstr: String = String::from_utf8(res.stdout).unwrap();
            Ok(outstr)
        }
        Err(e) => {
            // return Err(anyhow::anyhow!("run cmd error: {}", e));
            Err(OcrErr::OcrFail.into())
        }
    };

    // 删除图片
    fs::remove_file(filename).await?;

    res
}

/**
 * 中文识别
 *
 * 输入一个 cv 处理后的 mat ,返回识别后的文字
 */
pub struct ChiSim {}

impl ChiSim {
    pub async fn ocr(src: Mat) -> anyhow::Result<String> {
        let res = run_cmd(src.clone(), |filename| {
            let mut cmd = Command::new("tesseract");
            cmd.arg("-l")
                .arg("chi_sim")
                .arg(format!("/app/{filename}"))
                .arg("stdout");

            cmd
        })
        .await?;

        Ok(res)
    }

    // 识别中文位置
    pub async fn ocr_pos(src: Mat) -> anyhow::Result<String> {
        let res = run_cmd(src.clone(), |filename| {
            let mut cmd = Command::new("tesseract");
            cmd.arg("-l")
                .arg("chi_sim")
                .arg("-c")
                .arg("tessedit_create_boxfile=1")
                .arg(format!("/app/{filename}"))
                .arg("stdout");

            cmd
        })
        .await?;

        Ok(res)
    }
}

/**
 * 英文识别
 *
 * 输入一个 cv 处理后的 mat ,返回识别后的文字
 */
pub struct Eng {}

impl Eng {
    // 默认识别英文 (在识别数字时，选择英文识别)
    pub async fn ocr(src: Mat) -> anyhow::Result<String> {
        let res = run_cmd(src.clone(), |filename| {
            let mut cmd = Command::new("tesseract");
            cmd.arg(format!("/app/{filename}")).arg("stdout");

            cmd
        })
        .await?;

        Ok(res)
    }
}

#[derive(Debug, Deserialize)]
struct BatchDownloadFileRes {
    errcode: i32,
    errmsg: String,
    file_list: Vec<FileList>,
}

#[derive(Debug, Deserialize)]
struct FileList {
    fileid: String,
    download_url: String,
    status: i32,
    errmsg: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use opencv::imgcodecs;

    const IMG_PATH_HRL: &str = "ocr-test/hrl.jpg";

    #[tokio::test]
    async fn test_ocr() {
        let src = imgcodecs::imread(IMG_PATH_HRL, imgcodecs::IMREAD_GRAYSCALE).unwrap();
        let res = ChiSim::ocr(src.clone()).await;
        println!("res: {:?}", res);
    }
}
