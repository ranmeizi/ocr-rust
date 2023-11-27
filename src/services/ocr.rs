// use opencv::prelude::Mat;
use crate::util::random_str::random_str;
use anyhow;
use anyhow::Result;
use axum::body::Bytes;
use opencv::{
    core::{in_range, Mat, Scalar, Vector},
    imgcodecs::{self, imread, imwrite},
};
use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::json;
use tokio::process::Command;

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

    /** 二值化 */
    pub fn binarization_vec(vec: Vec<u8>) -> Mat {
        // 读取图片 mat
        let data: Vector<u8> = Vector::from_iter(vec);

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
    pub async fn ocr(dst: Mat) -> anyhow::Result<String> {
        let filename = random_str(20) + ".jpg";

        // 保存阈值化后的图片
        imwrite(&filename, &dst, &Vector::new()).unwrap();

        let output = Command::new("tesseract")
            .arg("-l")
            .arg("chi_sim")
            .arg(format!("/app/{filename}"))
            // .arg(format!("/Users/boboan/code/work/self/ocr-rust/{filename}"))
            .arg("stdout")
            .output()
            .await
            .unwrap();

        let outstr: String = String::from_utf8(output.stdout).unwrap();
        println!("{}", outstr);
        Ok(outstr)
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

        println!("download_url,{}", download_url);

        Ok((download_url.to_owned()))
    }

    pub async fn download_url(url: String) -> anyhow::Result<Bytes> {
        println!("download_url,{}", url);
        let response = reqwest::get(url).await?;

        // 创建一个文件来保存图片
        let mut bytes = response.bytes().await?;

        Ok((bytes))
    }

    /** 拿着ocr的结果调用get_pet_info */
    pub fn get_pet_info() {}

    /** 拿着ocr的结果调用get_stats_info  */
    pub fn get_stats_info() {}
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
    use opencv::{
        core,
        imgcodecs::{imread, imwrite},
        imgproc,
    };
    use serde_json::json;

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

    #[test]
    fn test_serde() {
        let abc = json!({
            "env":"prod-7g2acur2ee3f2b4e",
            "file_list":[{
                "fileid":"12",
                "max_age":60*5
            }]
        });
        println!("{}", abc.to_string())
    }
}
