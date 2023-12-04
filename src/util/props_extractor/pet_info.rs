use crate::util::props_extractor::boxfile::TextPos;
use opencv::core;

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
