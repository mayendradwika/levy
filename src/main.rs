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
    /// Who made this?
    Author,
    /// What time is it?
    Time,
    /// What Date is it?
    Date,
    /// Generate a random emote
    Emote,
    /// Glory, glory, Man United!
    Manutd,
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
            println!("Yo! I'm Levy CLI 🤙, your go-to homie for basic tasks. 🚀");
            println!("I can crunch numbers, say hi, and do other cool stuff.");
            println!("Stuck? Just hit me up with `levy --help`. You got this! 😎");
        }

        Commands::Greet { name } => {
            if name.trim().is_empty() {
                return Err("Name cannot be empty. Use: levy greet --name <NAME>".into());
            }
            commands::greet(&name);
        }
        Commands::Author => {
            println!("Who made this?🙄");
            println!("Github: [@mayendradwika]");
            println!("Status: Online");
            println!("IP: [Redacted]");
            println!("There's always a way in....🤙");
        }
        Commands::Time => {
            commands::show_time();
        }
        Commands::Date => {
            commands::show_date();
        }
        Commands::Emote => {
            commands::get_emote();
        }
        Commands::Manutd => {
            println!("Glory, glory, Man United!");
            println!("Glory, glory, Man United!");
            println!("Glory, glory, Man United! ");
            println!("As the Reds go marching on, on, on!🔥 🔥")
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