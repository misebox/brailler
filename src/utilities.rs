use std::sync::LazyLock;

use std::env;

/// 環境変数 key を bool として取得する関数
/// 指定されていなかったり、解釈できない場合は false を返します
pub(crate) fn get_env_bool(key: &str) -> bool {
    env::var(key)
        .map(|s| {
            match s.to_lowercase().as_str() {
                "true" | "1" | "yes" => true,
                "false" | "0" | "no" => false,
                _ => false, // 不明な値は false とする
            }
        })
        .unwrap_or(false)
}

pub(crate) static MEASURE_TIME: LazyLock<bool> = LazyLock::new(|| get_env_bool("MEASURE_TIME"));


#[macro_export]
macro_rules! measure_time {
    ($expr:expr) => {{
        if *$crate::utilities::MEASURE_TIME {
            let start = std::time::Instant::now();
            let result = $expr;
            let duration = start.elapsed();
            eprintln!("Execution time ({}): {:?}", stringify!($expr), duration);
            result
        } else {
            $expr
        }
    }};
}
