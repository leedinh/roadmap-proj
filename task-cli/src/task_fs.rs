use std::{
    fmt::{self, Display, Formatter},
    ops::Deref,
    str::FromStr,
};

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "todo" => Ok(Self::Todo),
            "in-progress" => Ok(Self::InProgress),
            "done" => Ok(Self::Done),
            _ => Err(()),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Status::Todo => write!(f, "todo"),
            Status::InProgress => write!(f, "in-progress"),
            Status::Done => write!(f, "done"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub status: Status,
    pub created_at: String,
    pub updated_at: String,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} | {} | {} | {} | {}",
            self.id, self.description, self.status, self.created_at, self.updated_at
        )
    }
}

impl Task {
    pub fn new(description: String) -> Self {
        let now = Utc::now().to_rfc2822();
        Self {
            id: 0,
            description,
            status: Status::Todo,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn update_status(&mut self, status: Status) {
        self.status = status;
        self.updated_at = Utc::now().to_rfc2822();
    }

    pub fn update_description(&mut self, description: String) {
        self.description = description;
        self.updated_at = Utc::now().to_rfc2822();
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TaskList(pub Vec<Task>);

impl Deref for TaskList {
    type Target = Vec<Task>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for TaskList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "ID | Description | Status | Created At | Updated At")?;
        writeln!(f, "---------------------------------------------")?;
        for task in self.iter() {
            writeln!(f, "{}", task)?;
        }
        Ok(())
    }
}

impl TaskList {
    pub async fn load() -> Result<Self> {
        let mut file = match File::open("tasks.json").await {
            Ok(file) => file,
            Err(_) => {
                File::create("tasks.json")
                    .await
                    .expect("Failed to create tasks.json");
                File::open("tasks.json")
                    .await
                    .expect("Failed to open tasks.json")
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        let tasks = serde_json::from_str(&contents).unwrap_or_default();
        Ok(Self(tasks))
    }

    pub async fn save(&self) -> Result<()> {
        let mut file = File::create("tasks.json").await?;
        let tasks = serde_json::to_string(&self)?;
        file.write_all(tasks.as_bytes()).await?;
        Ok(())
    }

    pub fn add_task(&mut self, task: Task) {
        self.0.push(task);
    }

    pub fn get_task_by_id(&mut self, id: u64) -> Option<&mut Task> {
        self.0.iter_mut().find(|task| task.id == id)
    }

    pub fn remove_task(&mut self, id: u64) {
        self.0.retain(|task| task.id != id);
        self.update_task_id();
    }

    fn update_task_id(&mut self) {
        self.0.iter_mut().enumerate().for_each(|(i, task)| {
            task.id = i as u64 + 1;
        });
    }

    pub fn list_tasks(&mut self, status: Option<Status>) {
        match status {
            None => {}
            Some(status) => {
                self.0.retain_mut(|task| task.status == status);
            }
        }

        println!("{}", self);
    }
}
