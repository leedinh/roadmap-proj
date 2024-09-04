use task_fs::Status;

mod task_fs;

#[tokio::main]
async fn main() {
    let mut tasks = task_fs::TaskList::load().await.unwrap();

    let arg = std::env::args().nth(1);

    match arg {
        Some(ref arg) if arg == "list" => {
            let status = std::env::args()
                .nth(2)
                .map(|status| status.parse::<Status>().expect("invalid status"));
            tasks.list_tasks(status);
        }
        Some(ref arg) if arg == "add" => {
            let description = std::env::args().nth(2).expect("description required");
            let task = task_fs::Task::new(description);
            tasks.add_task(task);
        }
        Some(ref arg) if arg == "update" => {
            let id = std::env::args()
                .nth(2)
                .expect("id required")
                .parse()
                .unwrap();
            let description = std::env::args().nth(3).expect("description required");
            let task = tasks.get_task_by_id(id).expect("task not found");
            task.update_description(description);
        }
        Some(ref arg) if arg == "remove" => {
            let id = std::env::args()
                .nth(2)
                .expect("id required")
                .parse()
                .unwrap();
            tasks.remove_task(id);
        }
        Some(ref arg) if arg == "mark-in-progress" => {
            let id = std::env::args()
                .nth(2)
                .expect("id required")
                .parse()
                .unwrap();
            let task = tasks.get_task_by_id(id).expect("task not found");
            task.update_status(task_fs::Status::InProgress);
        }
        Some(ref arg) if arg == "mark-done" => {
            let id = std::env::args()
                .nth(2)
                .expect("id required")
                .parse()
                .unwrap();
            let task = tasks.get_task_by_id(id).expect("task not found");
            task.update_status(task_fs::Status::Done);
        }

        _ => {
            println!("Usage: task-cli [list|add <description>|done <id>]");
        }
    }

    tasks.save().await.unwrap();
}
