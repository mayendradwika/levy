use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "levy", version = "1.0", about = "CLI tool to level up your life! 🚀")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Who made this?
    Author,
    /// Who Am I
    Who,
    /// Greeting user
    Greet {
        /// name
        #[arg(short, long)]
        name: String,
    },
    /// Calculate
    Calculate {
        /// first number
        #[arg(short, long)]
        a: i32,
        /// second number
        #[arg(short, long)]
        b: i32,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Greet { name } => {
            greet(&name);
        }
        Commands::Calculate { a, b } => {
            calculate(a, b);
        }
        Commands::Who => {
            who();
        }
        Commands::Author => {
            author();
        }
    }
}

fn author() {
    println!("Made by @mayendradwika 😎. Please visit https://github.com/mayendradwika/levy ");
}

fn who() {
    println!("Hey there! 🤙 I'm Levy 😁, and this CLI tool’s here to level up your life! 🚀 [@mayendradwika]");
}
/// greeting function
fn greet(name: &str) {
    println!("Hello, {}! My name is Levy ", name);
}

/// calculate function
fn calculate(a: i32, b: i32) {
    println!("The sum of {} and {} is {}.", a, b, a + b);
}
