use crate::util::random_str::random_str;
use anyhow::{self};
use opencv::{core::Vector, imgcodecs::imwrite, prelude::*};
use tokio::{fs, process::Command};

struct Service {

}

// ⬇ logic

// 使用临时文件 调用 tesseract 识别
async fn run_cmd<F>(src: &Mat, cmd: F) -> anyhow::Result<String>
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
            return Err(anyhow::anyhow!("run cmd error: {}", e));
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
    pub async fn ocr(src: &Mat) -> anyhow::Result<String> {
        run_cmd(src, |filename| {
            let mut cmd = Command::new("tesseract");
            cmd.arg("-l")
                .arg("chi_sim")
                .arg(format!("/app/{filename}"))
                .arg("stdout");

            cmd
        })
        .await
    }

    // 识别中文位置
    pub async fn ocr_pos(src: &Mat) -> anyhow::Result<String> {
        run_cmd(src, |filename| {
            let mut cmd = Command::new("tesseract");
            cmd.arg("-l")
                .arg("chi_sim")
                .arg("-c")
                .arg("tessedit_create_boxfile=1")
                .arg(format!("/app/{filename}"))
                .arg("stdout");

            cmd
        })
        .await
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
    pub async fn ocr(src: &Mat) -> anyhow::Result<String> {
        run_cmd(src, |filename| {
            let mut cmd = Command::new("tesseract");
            cmd.arg(format!("/app/{filename}")).arg("stdout");

            cmd
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opencv::imgcodecs;

    const IMG_PATH_HRL: &str = "ocr-test/hrl.jpg";

    #[tokio::test]
    async fn test_ocr() {
        let src = imgcodecs::imread(IMG_PATH_HRL, imgcodecs::IMREAD_GRAYSCALE).unwrap();
        let res = ChiSim::ocr(&src).await;
        println!("res: {:?}", res);
    }
}
