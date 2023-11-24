use rand::distributions::Alphanumeric;
use rand::Rng;
use std::iter;

// 生成一串随机字符串
pub fn random_str(len: usize)->String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();

    s
}

mod test {
    use super::*;

    #[test]
    fn test_random_str() {
        let str = random_str(20);

        println!("{}", str);
    }
}
