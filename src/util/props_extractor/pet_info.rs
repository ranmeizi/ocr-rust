use crate::util::props_extractor::boxfile::TextPos;
use opencv::core;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

// 搜索线路
const SEARCH_LINE: [&'static str; 10] = [
    "growth_sum",
    "growth_hp",
    "growth_atk",
    "growth_def",
    "growth_agi",
    "growth_sum_ex",
    "growth_hp_ex",
    "growth_atk_ex",
    "growth_def_ex",
    "growth_agi_ex",
];

pub fn get_pet_info(text: &str) -> PetInfoDto {
    let trim_re = Regex::new(r"\s").unwrap();
    // 转 vec

    let newline_re = Regex::new(r"\u21b5|\n").unwrap();

    let vec = newline_re.split(text).collect::<Vec<&str>>();

    // 去掉空元素
    let iter = vec.iter().filter(|x| **x != "");

    // 去掉空格
    let iter = iter.map(|x| trim_re.replace_all(x.trim(), "").to_string());

    let data = PetInfoState {
        r#type: "growth_sum".to_owned(),
        name: None,
        growth_sum: None,
        growth_hp: None,
        growth_atk: None,
        growth_def: None,
        growth_agi: None,
        growth_sum_ex: None,
        growth_hp_ex: None,
        growth_atk_ex: None,
        growth_def_ex: None,
        growth_agi_ex: None,
    };
    // 找第一个 num 作为总成长

    let res = iter.fold(data, |state, curr| {
        return match state.r#type.clone().as_str() {
            "growth_sum" => Reducers::growth_sum(state, curr),
            "growth_hp" => Reducers::growth_hp(state, curr),
            "growth_atk" => Reducers::growth_atk(state, curr),
            "growth_def" => Reducers::growth_def(state, curr),
            "growth_agi" => Reducers::growth_agi(state, curr),
            "growth_sum_ex" => Reducers::growth_sum_ex(state, curr),
            "growth_hp_ex" => Reducers::growth_hp_ex(state, curr),
            "growth_atk_ex" => Reducers::growth_atk_ex(state, curr),
            "growth_def_ex" => Reducers::growth_def_ex(state, curr),
            "growth_agi_ex" => Reducers::growth_agi_ex(state, curr),
            _ => state,
        };
    });

    res.into()
}

// 找 转 和 元素 确定区域
pub fn get_text_pos_vec(list: Vec<TextPos>, img_size: core::Size) -> core::Rect {
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
    } = list
        .iter()
        .fold(init, |state, curr| match curr.text.as_str() {
            "转" => handlers::zhuan(state, curr.clone()),
            "素" => handlers::su(state, curr.clone()),
            _ => handlers::default_run(state, curr),
        });

    let core::Size { width, height } = img_size;

    let zhuan = target_zhuan.unwrap();
    let yuan = target_yuan.unwrap();
    // 画图
    let top = zhuan.bottom;
    let left = yuan.left;
    let bottom = yuan.top;

    let h = top - bottom;

    let w = (2.1 * h as f64).round() as i32;

    core::Rect::new(left, height - top, w, h)
}

fn capture_num(text: &str) -> Option<(String, String)> {
    let num_re = Regex::new(r"[\({]([0-9]+[.][0-9]+)[~-]{1,2}([0-9]+[.][0-9]+)[\)}]").unwrap();

    if let Some(caps) = num_re.captures(text) {
        Some((
            format!("{:.3}", caps[1].parse::<f32>().unwrap()),
            format!("{:.3}", caps[2].parse::<f32>().unwrap()),
        ))
    } else {
        None
    }
}

struct Reducers {}

impl Reducers {
    // 总成长，匹配下一个 num 作为 总成长
    fn growth_sum(state: PetInfoState, curr: String) -> PetInfoState {
        if let Some(res) = capture_num(&curr) {
            PetInfoState {
                r#type: "growth_hp".to_owned(),
                growth_sum: Some(res),
                ..state
            }
        } else {
            state
        }
    }
    // 生命成长，匹配下一个 num 作为 攻击成长
    fn growth_hp(state: PetInfoState, curr: String) -> PetInfoState {
        if let Some(res) = capture_num(&curr) {
            PetInfoState {
                r#type: "growth_atk".to_owned(),
                growth_hp: Some(res),
                ..state
            }
        } else {
            state
        }
    }
    // 攻击成长，匹配下一个 num 作为 防御成长
    fn growth_atk(state: PetInfoState, curr: String) -> PetInfoState {
        if let Some(res) = capture_num(&curr) {
            PetInfoState {
                r#type: "growth_def".to_owned(),
                growth_atk: Some(res),
                ..state
            }
        } else {
            state
        }
    }
    // 防御成长，匹配下一个 num 作为 敏捷成长
    fn growth_def(state: PetInfoState, curr: String) -> PetInfoState {
        if let Some(res) = capture_num(&curr) {
            PetInfoState {
                r#type: "growth_agi".to_owned(),
                growth_def: Some(res),
                ..state
            }
        } else {
            state
        }
    }
    // 敏捷成长，匹配下一个 num 作为 总成长
    fn growth_agi(state: PetInfoState, curr: String) -> PetInfoState {
        if let Some(res) = capture_num(&curr) {
            PetInfoState {
                r#type: "growth_sum_ex".to_owned(),
                growth_agi: Some(res),
                ..state
            }
        } else {
            state
        }
    }

    // 转生后总成长，匹配下一个 num 作为 转生后总成长
    fn growth_sum_ex(state: PetInfoState, curr: String) -> PetInfoState {
        if let Some(res) = capture_num(&curr) {
            PetInfoState {
                r#type: "growth_hp_ex".to_owned(),
                growth_sum_ex: Some(res),
                ..state
            }
        } else {
            state
        }
    }
    // 转生后生命成长，匹配下一个 num 作为 转生后生命成长
    fn growth_hp_ex(state: PetInfoState, curr: String) -> PetInfoState {
        if let Some(res) = capture_num(&curr) {
            PetInfoState {
                r#type: "growth_atk_ex".to_owned(),
                growth_hp_ex: Some(res),
                ..state
            }
        } else {
            state
        }
    }
    // 转生后攻击成长，匹配下一个 num 作为 转生后攻击成长
    fn growth_atk_ex(state: PetInfoState, curr: String) -> PetInfoState {
        if let Some(res) = capture_num(&curr) {
            PetInfoState {
                r#type: "growth_def_ex".to_owned(),
                growth_atk_ex: Some(res),
                ..state
            }
        } else {
            state
        }
    }
    // 转生后防御成长，匹配下一个 num 作为 转生后防御成长
    fn growth_def_ex(state: PetInfoState, curr: String) -> PetInfoState {
        if let Some(res) = capture_num(&curr) {
            PetInfoState {
                r#type: "growth_agi_ex".to_owned(),
                growth_def_ex: Some(res),
                ..state
            }
        } else {
            state
        }
    }
    // 转生后敏捷成长，匹配下一个 num 作为 转生后敏捷成长
    fn growth_agi_ex(state: PetInfoState, curr: String) -> PetInfoState {
        if let Some(res) = capture_num(&curr) {
            PetInfoState {
                r#type: "normal".to_owned(),
                growth_agi_ex: Some(res),
                ..state
            }
        } else {
            state
        }
    }
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
        if state.target_yuan.is_none() {
            if let Some(last) = &state.last_char {
                if last.text.as_str() == "元" {
                    state.target_yuan = Some(last.clone());
                }
            }
        }
        let mut state = handlers::default_run(state, &curr);

        state
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

#[derive(Debug, Serialize)]
struct PetInfoState {
    r#type: String,
    name: Option<String>,
    growth_sum: Option<(String, String)>,
    growth_hp: Option<(String, String)>,
    growth_atk: Option<(String, String)>,
    growth_def: Option<(String, String)>,
    growth_agi: Option<(String, String)>,
    growth_sum_ex: Option<(String, String)>,
    growth_hp_ex: Option<(String, String)>,
    growth_atk_ex: Option<(String, String)>,
    growth_def_ex: Option<(String, String)>,
    growth_agi_ex: Option<(String, String)>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PetInfoDto {
    name: Option<String>,
    growth_sum: Option<(String, String)>,
    growth_hp: Option<(String, String)>,
    growth_atk: Option<(String, String)>,
    growth_def: Option<(String, String)>,
    growth_agi: Option<(String, String)>,
    growth_sum_ex: Option<(String, String)>,
    growth_hp_ex: Option<(String, String)>,
    growth_atk_ex: Option<(String, String)>,
    growth_def_ex: Option<(String, String)>,
    growth_agi_ex: Option<(String, String)>,
}

impl PetInfoDto {
    pub fn default() -> Self {
        Self {
            name: None,
            growth_sum: None,
            growth_hp: None,
            growth_atk: None,
            growth_def: None,
            growth_agi: None,
            growth_sum_ex: None,
            growth_hp_ex: None,
            growth_atk_ex: None,
            growth_def_ex: None,
            growth_agi_ex: None,
        }
    }
}

// 转换函数
impl From<PetInfoState> for PetInfoDto {
    fn from(value: PetInfoState) -> Self {
        Self {
            name: value.name,
            growth_sum: value.growth_sum,
            growth_hp: value.growth_hp,
            growth_atk: value.growth_atk,
            growth_def: value.growth_def,
            growth_agi: value.growth_agi,
            growth_sum_ex: value.growth_sum_ex,
            growth_hp_ex: value.growth_hp_ex,
            growth_atk_ex: value.growth_atk_ex,
            growth_def_ex: value.growth_def_ex,
            growth_agi_ex: value.growth_agi_ex,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "SARIS

    FB ARIS
    PX ARIS
    Pa AR IS
    BLE ALIS
    
    (3.091~5.154)
    (0.94~1.568)
    (1.199~1.999)
    (0.25~0.417)
    (0.702~1.17)
    
    (3.553~7.214)
    (1.081~2.195)
    (1.378~2.798)
    (0.287~0.583)
    (0.807~1.638)";

    #[test]
    fn test_get_text_pos_vec() {
        let res = get_pet_info(INPUT_STR);

        println!("res : {:?}", res);
    }
}
