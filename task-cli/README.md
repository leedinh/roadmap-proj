# Task Tracker

Project task tracker from [roadmap.sh](https://roadmap.sh/projects/task-tracker)

## Adding a new task

To add a new task using `task-cli`, you can use the following command:

```
task-cli add "Buy groceries"
```

The output will be: `Task added successfully (ID: 1)`

## Updating and deleting tasks

To update a task, you can use the command:

```
task-cli update 1 "Buy groceries and cook dinner"
```

To delete a task, you can use the command:

```
task-cli delete 1
```

## Marking a task as in progress or done

To mark a task as in progress, you can use the command:

```
task-cli mark-in-progress 1
```

To mark a task as done, you can use the command:

```
task-cli mark-done 1
```

## Listing all tasks

To list all tasks, you can use the command:

```
task-cli list
```

## Listing tasks by status

To list tasks by status, you can use the following commands:

```
task-cli list done
task-cli list todo
task-cli list in-progress
```

## Installation and running

Clone the repository and run the following command:

```
git clone https://github.com/leedinh/roadmap-proj.git
cd roadmap-proj/task-cli

```

Please note that the `task-cli` in [Rust](https://rustup.rs/). To build and run the `task-cli` project, you can use the following commands:

```
cargo build
/target/debug/task-cli <commands>
```

or

```
cargo run -- <commands>
```
