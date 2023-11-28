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
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

// #[derive(Debug, Serialize)]
// struct PetInfoState {
//     r#type: String,
//     name: Option<String>,
//     growth_sum: Option<String>,
//     growth_hp: Option<String>,
//     growth_atk: Option<String>,
//     growth_def: Option<String>,
//     growth_agi: Option<String>,
//     growth_sum_ex: Option<String>,
//     growth_hp_ex: Option<String>,
//     growth_atk_ex: Option<String>,
//     growth_def_ex: Option<String>,
//     growth_agi_ex: Option<String>,
// }

#[derive(Debug, Serialize)]
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

const KEYWORD_GROWTH1: &str = "转生前";
const KEYWORD_GROWTH2: &str = "转生后";
const KEYWORD_NAME: &str = "成长";

struct Reducers {}

impl Reducers {
    // 一般状态，用于判断状态转换
    fn normal(state: PetInfoState, curr: String) -> PetInfoState {
        if curr == KEYWORD_NAME {
            return PetInfoState {
                r#type: "name".to_owned(),
                ..state
            };
        }

        if curr == KEYWORD_GROWTH1 {
            return PetInfoState {
                r#type: "growth_sum".to_owned(),
                ..state
            };
        }

        if curr == KEYWORD_GROWTH2 {
            return PetInfoState {
                r#type: "growth_sum_ex".to_owned(),
                ..state
            };
        }

        return state;
    }
    // 名称状态，匹配下一个词,作为宠物名称
    fn name(state: PetInfoState, curr: String) -> PetInfoState {
        return PetInfoState {
            r#type: "normal".to_owned(),
            name: Some(curr),
            ..state
        };
    }
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
                r#type: "normal".to_owned(),
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

fn capture_num(text: &str) -> Option<(String, String)> {
    let num_re = Regex::new(r"\(([0-9]+[.][0-9]+)~([0-9]+[.][0-9]+)\)").unwrap();

    if let Some(caps) = num_re.captures(text) {
        Some((
            format!("{:.3}", caps[1].parse::<f32>().unwrap()),
            format!("{:.3}", caps[2].parse::<f32>().unwrap()),
        ))
    } else {
        None
    }
}

pub fn pet_info(text: &str) -> PetInfoDto {
    let trim_re = Regex::new(r"\s").unwrap();
    // 转 vec
    let vec = text.split("\u{21b5}").collect::<Vec<&str>>();

    // 去掉换行
    let iter = vec.iter().filter(|x| **x != "");

    // 去掉空格
    let iter = iter.map(|x| trim_re.replace_all(x, "").to_string());

    let data = PetInfoState {
        r#type: "normal".to_owned(),
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

    let res = iter.fold(data, |state, curr| {
        return match state.r#type.clone().as_str() {
            "normal" => Reducers::normal(state, curr),
            "name" => Reducers::name(state, curr),
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
            _ => Reducers::normal(state, curr),
        };
    });
    // println!("{:?}", json!(res).to_string());
    res.into()
}

mod tests {
    use super::*;

    const TEXT: &str = "飞龙 系↵↵雷 龙 系↵↵成 长↵↵邦 奇 诺↵↵总 成 长↵↵生命 成 长↵攻击 成 长↵防御 成 长↵敏捷 成 长↵↵元 素↵↵转生 前↵↵(3.091~5.154)↵(0.94~1.568)↵(1.199~1.999)↵(0.25~0.417)↵(0.702~1.17)↵↵” 抽风 内 而 出 网 古风 同和↵↵| 属性↵↵转生 后↵↵(3.553~7.214)↵(1.081~2.195)↵(1.378~2.798)↵(0.287~0.583)↵(0.807~1.638)↵↵获取 途径↵";

    #[test]
    fn it_works() {
        pet_info(TEXT);
    }

    #[test]
    fn test_cap() {
        let res = capture_num("(0.807~1.638)");
        println!("{:?}", res);
    }
}
