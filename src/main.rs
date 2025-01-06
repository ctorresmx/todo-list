use clap::{Parser, Subcommand};
use todo_list::model::{Status, Todo};
use todo_list::persistance;

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

fn next_available_id(ids: &Vec<i64>) -> i64 {
    let mut sorted_ids = ids.clone();
    sorted_ids.sort();

    if ids.is_empty() {
        return 1;
    }

    let mut next_id = 1;

    for &id in ids.iter() {
        if id > next_id {
            return next_id;
        }
        next_id = id + 1;
    }

    next_id
}

fn list(completed: bool, pending: bool) {
    println!(
        "Running List command for completed '{}' and pending '{}' filters.",
        completed, pending
    );
    let todos = persistance::read();
    println!("Todos: '{:?}'", todos)
}

fn add(item: String) {
    let mut todos: Vec<Todo> = persistance::read();
    let ids: Vec<i64> = todos.iter().map(|todo| todo.id).collect();
    let id = next_available_id(&ids);

    let new_item = Todo {
        id,
        content: item,
        status: Status::Pending,
    };
    todos.push(new_item);
    persistance::write(todos);
}

fn remove(id: i64) {
    let todos = persistance::read();
    let new_todos = todos.into_iter().filter(|todo| todo.id != id).collect();
    persistance::write(new_todos);
}

fn complete(id: i64) {
    let todos = persistance::read();

    let new_todos = todos
        .into_iter()
        .map(|todo| {
            if todo.id == id {
                Todo {
                    id: todo.id,
                    content: todo.content.clone(),
                    status: Status::Completed,
                }
            } else {
                todo
            }
        })
        .collect();

    persistance::write(new_todos);
}

fn clean() {
    persistance::write(vec![]);
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
