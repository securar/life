pub fn naive_dedent(text: &str) -> String {
    let mut result = String::with_capacity(text.len());

    for line in text.split('\n') {
        let line = line.trim_start_matches(' ');
        result.push_str(line);
        result.push('\n');
    }

    return result;
}

#[macro_export]
macro_rules! measure {
    ($block:block) => {{
        let start_time = std::time::Instant::now();
        let result = $block;
        let elapsed_time = start_time.elapsed();
        eprintln!(
            "[{}:{}:{}]: took {:?}",
            file!(),
            line!(),
            column!(),
            elapsed_time
        );
        result
    }};
}
