use std::io::Write;
use std::time::Duration;
use std::process::Command;

#[async_std::main]
async fn main() -> Result<(), surf::Error> {
    let args: Vec<String> = std::iter::once("! site:github.com".to_string())
        .chain(std::env::args().skip(1))
        .collect();

    if args.len() == 1 {
        eprintln!(
            r#"
cloneLucky! Clone your github directly on your terminal

USAGE:
    cloneLucky tensorflow
    cloneLucky rust based reverse proxy
"#
        );
        std::process::exit(1);
    }

    let args = args.join(" ");
    let query = urlencoding::encode(args.as_str());

    let url = format!("https://duckduckgo.com/?q={}&format=json", query);
    println!("url: {}",url);
    let response = surf::get(url.as_str()).await?;
    let location = response
        .header("location")
        .map(|xs| xs.as_str().to_owned())
        .unwrap_or_else(Default::default);
    println!("location: {:?}", location);
    if location.is_empty() {
        async_std::task::sleep(Duration::from_millis(200)).await;
        std::io::stderr().write_all(b" No results.")?;
        std::process::exit(1);
    }

    // added for git clone
    // let command = Command::new("git")
    Command::new("git")
            .arg("clone")
            .arg(location + &".git".to_owned())
            .spawn()
            .expect("failed to execute process");

    Ok(())
}
