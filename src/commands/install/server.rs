use anyhow::{Context, Result};
use clap::Args;
use console::style;
use dialoguer::{Input, theme::ColorfulTheme};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

#[derive(Args)]
pub struct ServerInstallArgs {
    /// Install in development mode
    #[arg(long)]
    dev: bool,

    /// Installation directory (default: ~/.kuken)
    install_dir: Option<PathBuf>,
}

const REPO_URL: &str = "https://github.com/devnatan/kuken.git";
const KUKEN_ORANGE: (u8, u8, u8) = (255, 116, 56); // #ff7438

pub async fn execute(args: ServerInstallArgs) -> Result<()> {
    print_fancy_header();

    let multi = MultiProgress::new();
    // check_java(&multi);
    print_success_message();

    Ok(())
}

fn print_fancy_header() {
    let orange = custom_color(KUKEN_ORANGE);

    println!();
    println!(
        "{}",
        style("╔═════════════════════════════════════════════════════════════╗").fg(orange)
    );
    println!(
        "{}",
        style("║                                                             ║").fg(orange)
    );
    println!(
        "{}{}{}",
        style("║           ").fg(orange),
        style("       🐤 Küken Installer").bold().fg(orange),
        style("                  ║").fg(orange)
    );
    println!(
        "{}",
        style("║                                                             ║").fg(orange)
    );
    println!(
        "{}",
        style("╚═════════════════════════════════════════════════════════════╝").fg(orange)
    );
    println!();
}

fn print_box(message: &str) {
    let orange = custom_color(KUKEN_ORANGE);
    let width = 61;
    let padding = (width - console::measure_text_width(message)) / 2;

    println!(
        "{}",
        style("┌─────────────────────────────────────────────────────────────┐").fg(orange)
    );
    println!(
        "{}{}{}",
        style("│").fg(orange),
        " ".repeat(padding)
            + message
            + &" ".repeat(width - padding - console::measure_text_width(message)),
        style("│").fg(orange)
    );
    println!(
        "{}",
        style("└─────────────────────────────────────────────────────────────┘").fg(orange)
    );
}

fn print_success_message() {
    let orange = custom_color(KUKEN_ORANGE);

    println!();
    println!(
        "{}",
        style("╔═════════════════════════════════════════════════════════════╗")
            .fg(orange)
            .bold()
    );
    println!(
        "{}",
        style("║                                                             ║").fg(orange)
    );
    println!(
        "{}{}{}",
        style("║     ").fg(orange),
        style("    ⭐ Küken installed successfully! ⭐")
            .bold()
            .fg(orange),
        style("          ║").fg(orange)
    );
    println!(
        "{}",
        style("║                                                             ║").fg(orange)
    );
    println!(
        "{}",
        style("╚═════════════════════════════════════════════════════════════╝")
            .fg(orange)
            .bold()
    );
    println!();
}

fn custom_color(rgb: (u8, u8, u8)) -> console::Color {
    console::Color::Color256(16 + 36 * (rgb.0 / 51) + 6 * (rgb.1 / 51) + (rgb.2 / 51))
}

// fn check_java(multi: &MultiProgress) -> Result<()> {
//     let pb = create_spinner(multi, "Checking Java installation...");
//     let output = Command::new("java")
//         .arg("-version")
//         .output();
//
//     match output {
//         Ok(output) => {
//             let version_str = String::from_utf8_lossy(&output.stdout).trim();
//             let version_str = version_str.split(" ").nth(1).unwrap();
//             let version = version_str.parse::<i32>()?;
//
//             if version >= 24 {
//                 pb.finish_and_clear();
//                 println!("{} Java {} detected", style("✔").green(), version);
//                 Ok(())
//             } else {
//                 pb.finish_and_clear();
//                 println!("{} Java {} detected (requires Java 24+)", style("❌").red(), version);
//                 anyhow::bail!("Java 24 or higher is required. Please install Java first.")
//             }
//         }
//         Err(_) => {
//             pb.finish_and_clear();
//             println!("{} Java is not installed", style("❌").red());
//             anyhow::bail!("Java 24 or higher is required. Please install Java first.")
//         }
//     }
// }

fn create_spinner(multi: &MultiProgress, message: &str) -> ProgressBar {
    let pb = multi.add(ProgressBar::new_spinner());
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.orange} {msg}")
            .unwrap(),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}