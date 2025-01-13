# todo-list
A small app to practice and get more familiar with Rust.

For now, each command performs a different action. Therefore you will
need to run a different command to add, list, remove, etc.

## How to run it

The app has basic actions like:
- Add a todo
- List todos with filtering for complete and pending items
- Complete a todo
- Remove a todo
- Clean all todos

### Add a todo

This will add a todo with the description provided.

```sh
cargo run -- add --item "buy eggs"
```

### List todos

This command will list all todos, regardless of status.

```sh
cargo run -- list
```

#### List only completed todos
```sh
cargo run -- list --completed
```

#### List only pending todos
```sh
cargo run -- list --pending
```

### Complete todo

This will mark a todo as complete.

```sh
cargo run -- complete --id <id>
```

### Remove todo

This will remove a particular todo from the list.

```sh
cargo run -- remove --id <id>
```

### Clean todos

This will clean the list of todos.

```sh
cargo run -- clean
```
