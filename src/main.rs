use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "levy", version = "1.0", about = "Simple ClI Tool to help you")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Greeting user
    Greet {
        /// name
        #[arg(short, long)]
        name: String,
    },
    /// calculate
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
    }
}

/// greeting function
fn greet(name: &str) {
    println!("Hello, {}! My name is Levy ", name);
}

/// calculate function
fn calculate(a: i32, b: i32) {
    println!("The sum of {} and {} is {}.", a, b, a + b);
}
