use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::io::Result;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::{BufReader, Result, Seek, SeekFrom};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
  pub text: String,

  #[serde(with = "ts_seconds")]
  pub created_at: DateTime<Utc>,
}

impl Task {
  pub fn new(text: String) -> Task {
    let created_at: DateTime<Utc> = Utc::now();
    Task { text, created_at }
  }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
  let mut file = OpenOptions::new()
  .read(true)
  .write(true)
  .create(true)
  .open(journal_path)?;

  let mut tasks: Vec<Task> = match serde_json::from_reader(&file) {
    Ok(tasks) => tasks,
    Err(e) if e.is_eof() => Vec::new(),
    Err(e) => Err(e)?,
  };

  file.seek(SeekFrom::Start(0))?;

  tasks.push(task);
  serde_json::to_writer(file, &tasks)?;
  
  Ok(())
}

// pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> { ... }

// pub fn list_tasks(journal_path: PathBuf) -> Result<()> { ... }