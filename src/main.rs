use clap::{Parser, Subcommand};
mod commands;

#[derive(Parser)]
#[command(name = "levy", version = "1.0", about = "Levy CLI - A simple tool for basic tasks")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Who am i?
    Who,
    /// Greet a user with a specific name
    Greet {
        /// Name of the user to greet
        #[arg(short, long, required = true)]
        name: String,
    },
    /// Perform a simple addition operation
    Calculate {
        /// First number
        #[arg(short, long)]
        a: i32,
        /// Second number
        #[arg(short, long)]
        b: i32,
    },
    /// Perform a simple multiplication operation
    Multiply {
        /// First number
        #[arg(short, long)]
        a: i32,
        /// Second number
        #[arg(short, long)]
        b: i32,
    },
    /// Perform a simple subtraction operation
    Subtract {
        /// First number
        #[arg(short, long)]
        a: i32,
        /// Second number
        #[arg(short, long)]
        b: i32,
    },
    /// Perform a simple division operation
    Divide {
        /// First number
        #[arg(short, long)]
        a: i32,
        /// Second number
        #[arg(short, long)]
        b: i32,
    },
    #[command(external_subcommand)]
    Fallback(Vec<String>),
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Who => {
            println!("Levy CLI - Your buddy for handling basic tasks.");
            println!("Need help? Try `levy --help`.");
        }

        Commands::Greet { name } => {
            if name.trim().is_empty() {
                return Err("Name cannot be empty. Use: levy greet --name <NAME>".into());
            }
            commands::greet(&name);
        }
        Commands::Calculate { a, b } => {
            commands::calculate(a, b);
        }
        Commands::Multiply { a, b } => {
            commands::multiply(a, b);
        }
        Commands::Subtract { a, b } => {
            commands::subtract(a, b);
        }
        Commands::Divide { a, b } => {
            commands::divide(a, b)?;
        }
        Commands::Fallback(cmd) => {
            return Err(format!(
                "Unknown command '{}'. Use `levy --help` to see available commands.",
                cmd.join(" ")
            ));
        }
    }
    Ok(())
}