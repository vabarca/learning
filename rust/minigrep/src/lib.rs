pub struct Config {
    pub filename: String,
    pub query: String,
}

pub fn print_result(result: &Vec<&str>) {
    for line in result {
        println!("{}", line);
    }
}

pub fn analyze_file<'a>(cfg: &'a Config, content: &'a String) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.contains(&cfg.query) {
            result.push(line);
        }
    }

    result
}
