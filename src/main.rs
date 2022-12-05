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
lukgit! Clone your github directly on your terminal

USAGE:
    lukgit tensorflow
    lukgit rust based reverse proxy
"#
        );
        std::process::exit(1);
    }

    let args = args.join(" ");
    let query = urlencoding::encode(args.as_str());

    let url = format!("https://duckduckgo.com/?q={}&format=json", query);

    println!("url: {}",url);

    let mut response = surf::get(url.as_str()).await?;
    let git_URL = response
        .header("location")
        .map(|xs| xs.as_str().to_owned())
        .unwrap_or_else(Default::default);
    println!("git_URL: {:?}", git_URL);
    if git_URL.is_empty() {
        async_std::task::sleep(Duration::from_millis(200)).await;
        std::io::stderr().write_all(b" No results.")?;
        std::process::exit(1);
    }

    let mdurl = git_URL.replace("github.com", "raw.githubusercontent.com") + "/master/README.md";
    response = surf::get(mdurl.as_str()).await?;
    // get raw text from response
    let mdtext = response.body_string().await?;

    Command::new("git")
            .arg("clone")
            .arg(git_URL + &".git".to_owned())
            .spawn()
            .expect("failed to execute process");

    // print markdown
    Command::new("cat")
        .arg(mdtext)
        .spawn()
        .expect("failed to execute process");

    Ok(())
}
