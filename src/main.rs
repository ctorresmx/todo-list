use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    List {
        #[arg(short, long, default_value_t = false, conflicts_with = "pending")]
        completed: bool,

        #[arg(short, long, default_value_t = false, conflicts_with = "completed")]
        pending: bool,
    },
    Add {
        #[arg(short, long)]
        item: String,
    },
    Remove {
        #[arg(short, long)]
        id: i64,
    },
    Complete {
        #[arg(short, long)]
        id: i64,
    },
    Clean,
}

fn list(completed: bool, pending: bool) {
    println!(
        "Running List command for completed '{}' and pending '{}' filters.",
        completed, pending
    )
}

fn add(item: String) {
    println!("Running Add command with item '{}'", item)
}

fn remove(id: i64) {
    println!("Running Remove command for item '{}'", id)
}

fn complete(id: i64) {
    println!("Running Complete command for item '{}'", id)
}

fn clean() {
    println!("Running Clean command.")
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::List { completed, pending } => list(completed, pending),
        Commands::Add { item } => add(item),
        Commands::Remove { id } => remove(id),
        Commands::Complete { id } => complete(id),
        Commands::Clean => clean(),
    }
}
