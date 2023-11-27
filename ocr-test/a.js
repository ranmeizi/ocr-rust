let txt = '飞龙 系↵↵雷 龙 系↵↵成 长↵↵邦 奇 诺↵↵总 成 长↵↵生命 成 长↵攻击 成 长↵防御 成 长↵敏捷 成 长↵↵元 素↵↵转生 前↵↵(3.091~5.154)↵(0.94~1.568)↵(1.199~1.999)↵(0.25~0.417)↵(0.702~1.17)↵↵” 抽风 内 而 出 网 古风 同和↵↵| 属性↵↵转生 后↵↵(3.553~7.214)↵(1.081~2.195)↵(1.378~2.798)↵(0.287~0.583)↵(0.807~1.638)↵↵获取 途径↵'


txt = txt.split('\u21b5')

console.log(txt)

txt = txt.filter(item => item !== '')

console.log(txt)

txt = txt.map(item => item.replace(/\s/g, ''))

console.log(txt)

/**
 * 1.获取 “转生前” 后 匹配第一个() 作为开始的 总成长
 * 2.猜测 “成长” 后的第一个词 是 宠物名称
 */


// const keys = ['总成长', '生命成长', '攻击成长', '防御成长', '敏捷成长']
const keyword_growth1 = '转生前'
const keyword_growth2 = '转生后'
const keyword_name = '成长'

const num = /\(([0-9]+[.][0-9]+)~([0-9]+[.][0-9]+)\)/

const reducers = {
    // 一般状态，用于判断状态转换
    normal: (state, curr) => {
        if (curr === keyword_name) {
            return {
                ...state,
                type: 'name'
            }
        }

        if (curr === keyword_growth1) {
            return {
                ...state,
                type: 'growth_sum'
            }
        }

        if (curr === keyword_growth2) {
            return {
                ...state,
                type: 'growth_sum_ex'
            }
        }
    },
    // 名称状态，匹配下一个词,作为宠物名称
    name: (state, curr) => {
        return {
            ...state,
            type: 'normal',
            name: curr,
        }
    },
    // 总成长，匹配下一个 num 作为 总成长
    growth_sum: (state, curr) => {
        return {
            ...state,
            type: 'growth_hp',
            name: curr,
        }
    },
    // 生命成长，匹配下一个 num 作为 攻击成长
    growth_hp: (state, curr) => {
        
        return {
            ...state,
            type: 'growth_atk',
            growth_atk: curr
        }
    },
    // 攻击成长，匹配下一个 num 作为 防御成长
    growth_atk: () => {
        return 'growth_def'
    },
    // 防御成长，匹配下一个 num 作为 敏捷成长
    growth_def: () => {
        return 'growth_agi'
    },
    // 敏捷成长，转换为一般状态，继续匹配下一个词
    growth_agi: () => {
        return 'normal'
    },
    // 转生后总成长，匹配下一个 num 作为 转生后总成长
    growth_sum_ex: (state, curr) => {
        return 'growth_hp_ex'
    },
    // 转生后生命成长，匹配下一个 num 作为 转生后攻击成长
    growth_hp_ex: () => {
        return 'growth_atk_ex'
    },
    // 转生后攻击成长，匹配下一个 num 作为 转生后防御成长
    growth_atk_ex: () => {
        return 'growth_def_ex'
    },
    // 转生后防御成长，匹配下一个 num 作为 转生后敏捷成长
    growth_def_ex: () => {
        return 'growth_agi_ex'
    },
    // 转生后敏捷成长，转换为一般状态，继续匹配下一个词
    growth_agi_ex: () => {
        return 'normal_ex'
    }
}

const data = {
    type: 'normal',
    name: null,
    growth_sum: null,
    growth_hp: null,
    growth_atk: null,
    growth_def: null,
    growth_agi: null,
    growth_sum_ex: null,
    growth_hp_ex: null,
    growth_atk_ex: null,
    growth_def_ex: null,
    growth_agi_ex: null,
}

txt.reduce((state, curr) => {

})