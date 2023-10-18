use std::error::Error;
use std::fs;
use std::process::{Command, Stdio};

use clap::Parser;
use headless_chrome::Browser;
use which::which;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    file: String,
}

const URL: &str = "localhost:5000";

fn main() {
    let args = Args::parse();

    if !args.file.ends_with(".md") {
        println!("You must provide a markdown file!");
        return;
    }

    let path = which("grip");

    if path.is_err() {
        println!("Could not find grip. \
        Please install it according to their documentation: https://github.com/joeyespo/grip#installation");
        return;
    }

    println!("Grip found at {}", path.unwrap().as_path().to_str().unwrap());

    let mut command = Command::new("grip")
        .args(&[args.file.clone(), URL.to_string()])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not start grip");

    let file_path_with_extension = args.file.strip_suffix(".md");
    save_pdf(&*format!("{}.pdf", file_path_with_extension.unwrap())).unwrap();

    command.kill().unwrap();
}

fn save_pdf(filename: &str) -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;

    let tab = browser.new_tab()?;
    tab.navigate_to(&*format!("http://{}", URL))?;
    tab.wait_until_navigated()?;
    tab.wait_for_element("body")?;
    let bytes = tab.print_to_pdf(None)?;

    fs::write(filename, bytes)?;

    Ok(())
}