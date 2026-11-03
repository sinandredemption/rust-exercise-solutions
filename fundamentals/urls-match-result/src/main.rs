use url::Url;

fn main() {
    match read("urls-match-result/src/data/content.txt") {
        Ok(s) => s.lines()
                        .for_each(|line|
                            {
                                if !line.is_empty() { print!("{} -> ", line);
                                if let Some(_) = parse_url(line) { println!("Is a URL"); }
                                else { println!("Not a URL") };
                            }
                        }),
        Err(_) => println!("FileReadError"),
    };
}

fn read(path: &'static str) -> Result<String, std::io::Error>
{
    let f = std::fs::read_to_string(path)?;
    Ok(f)
}

fn parse_url(input: &str) -> Option<url::Url> {
    match Url::parse(input) {
        Ok(u) => Some(u),
        Err(_) => None,
    }
}