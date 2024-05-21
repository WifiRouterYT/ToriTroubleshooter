// this is my first Rust program so its ass.
// please suggest improvements in the Issues section if you have any 💀

use std::io;
use std::io::prelude::*;
use reqwest::Error;
use colored::Colorize;
use std::collections::HashMap;
use spinners::{Spinner, Spinners};
use std::process;
use online::check;

async fn check_website(url: &str) -> Result<bool, Error> {
    let response = reqwest::get(url).await?;
    Ok(response.status() == 200)
}

#[tokio::main]
async fn main() {
    let websites: HashMap<&str, &str> = [
        ("Tori API", "https://api.toriclient.com/status"),
        ("Tori CDN", "https://cdn.toriclient.com/test.txt"),
    ].iter().cloned().collect();

    // title
    println!("{}", "ToriTroubleshooter - A simple utility for Tori users to diagnose and fix common issues.".cyan().bold());
    println!("{}", "https://github.com/WifiRouterYT/ToriTroubleshooter\n".bright_black());
    println!("{}", "This utility will scan your system and attempt to troubleshoot common issues related to Tori Launcher or Tori Client itself. If this tool reports everything as good-to-go and something still isn't working, please open a Ticket (or return to your existing one) so we can assist you further.\n".bright_white());
    println!("{}", "What this utility will do:".bold());
    println!("{}", "   ▪ Check your internet connection".bright_black());
    println!("{}", "   ▪ Check to see if the Tori servers are accessible".bright_black());

    println!("");
    write!(io::stdout(), "Hit ENTER to continue, or CTRL+C to exit...").unwrap();
    io::stdout().flush().unwrap();
    let _ = io::stdin().read(&mut [0u8]).unwrap();

    // test if intrenet connection (you'd be surprised at how many issues this solves 😭)
    println!("{}", "\nChecking your internet connection...".bright_blue().bold());
    let mut sp = Spinner::new(Spinners::Dots, format!("{}", format!("Checking if you can access Google... (testing if you can reach IP 64.233.161.100)").bright_black()).into());
    if check(Some(10)).is_ok() {
        sp.stop_and_persist(format!("{}", "✔".green()).as_str(), format!("{}", "Internet test successful!".bright_green()).into());
    } else {
        sp.stop_and_persist(format!("{}", "✖".red()).as_str(), format!("{}", format!("Failed to access the internet!").bright_red()).into());
        println!("\nFailed to connect to the internet! Please check your connection and try again.");
        process::exit(1);
    }

    // test if tori server work
    println!("{}", "\nChecking connectivity to the Tori servers...".bright_blue().bold());
    for (name, url) in &websites {
        let mut sp = Spinner::new(Spinners::Dots, format!("{}", format!("Checking if {name} is online...").bright_black()).into());
        match check_website(url).await {
            Ok(is_online) => {
                if is_online {
                    sp.stop_and_persist(format!("{}", "✔".green()).as_str(), format!("{}", format!("{name} is online!").bright_green()).into());
                } else {
                    sp.stop_and_persist(format!("{}", "⚠".yellow()).as_str(), format!("{} {}", format!("{name} is online but experiencing some issues.").bright_yellow(), format!("(non OK HTTP response").bright_black()).into());
                }
            }
            Err(_e) => sp.stop_and_persist(format!("{}", "✖".red()).as_str(), format!("{}", format!("Failed to reach {name}!").bright_red()).into()),
        }
    }
}