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

struct Reducers {}

impl Reducers {
    // 一般状态，用于判断状态转换
    fn normal(state: PetInfoState, curr: String) -> PetInfoState {

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

pub fn pet_info(text: &str) -> PetInfoDto {
    let trim_re = Regex::new(r"\s").unwrap();
    // 转 vec

    let newline_re = Regex::new(r"\u21b5|\n").unwrap();

    let vec = newline_re.split(text).collect::<Vec<&str>>();

    // 去掉空元素
    let iter = vec.iter().filter(|x| **x != "");

    // 去掉空格
    let iter = iter.map(|x| trim_re.replace_all(x.trim(), "").to_string());

    println!("看看,{:?}", iter.clone().collect::<Vec<String>>());

    let data = PetInfoState {
        r#type: "name".to_owned(),
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
    println!("{:?}", json!(res).to_string());
    res.into()
}

mod tests {
    use super::*;

    const CASE1 :&str = "帖 拉 所 伊 休

    总 成 长
    
    生命 成 长
    攻击 成 长
    防御 成 长
    敏捷 成 长
    
    转生 前
    
    (3.125-~5.211)
    (0.961-1.603)
    (1.17~1.95)
    (0.328~0.548)
    (0.666~1.11)
    
    转生 后
    
    (3.592~7.295)
    (1.105~2.244)
    
    (1.345~2.73)
    (0.377-0.767)
    (0.765-~1.554)";

    const CASE2:&str = "葛 雷 基 欧

    总 成 长
    
    生命 成 长
    攻击 成 长
    防御 成 长
    敏捷 成 长
    
    转生 前
    
    (3.194~5.325)
    (0.942-~1.571)
    {1.167~1.945)
    (0.347-0.579)
    
    {0.738-~1.23)
    
    转生 后
    
    (3.672-~7.454)
    (1.083-~2.199)
    (1.342~2.723)
    
    (0.399-~0.81)
    (0.848-~1.722)";

    #[test]
    fn it_works() {
        // pet_info(TEXT);
        pet_info(CASE2);
    }

    #[test]
    fn test_cap() {
        let res = capture_num("(0.807~1.638)");
        println!("{:?}", res);
    }
}
