use anyhow::Result;
use serde::Serialize;
use serde_json::json;
use std::{
  collections::HashMap,
  time::{SystemTime, UNIX_EPOCH},
};
use todo_txt::{Date, Priority, Task};

fn read_todo_txt_file() -> Result<Vec<Task>> {
  use std::{
    fs::File,
    io::{prelude::*, BufReader},
    str::FromStr,
  };

  let file = File::open("todo.txt")?;
  let reader = BufReader::new(file);
  let mut tasks = Vec::new();
  for line in reader.lines() {
    let task = Task::from_str(line?.as_str())?;
    tasks.push(task);
  }

  Ok(tasks)
}

fn main() -> Result<()> {
  println!("Scanning todo.txt file...");

  let tasks = read_todo_txt_file()?;

  let tags = generate_tags(&tasks);
  generate_task_lines(&tasks, &tags)?;

  Ok(())
}

fn generate_tags(tasks: &Vec<Task>) -> HashMap<String, String> {
  println!("Tags:");
  println!("---");
  let mut tag_map = HashMap::new();
  for task in tasks {
    let task_tags = task.projects.to_owned();
    for task_tag in &task_tags {
      let entry = tag_map.get(task_tag);
      if entry.is_none() {
        let tag_uid = random_number_generator(17);
        tag_map.insert(task_tag.to_owned(), tag_uid.to_owned());
        let json = json!({
            "color": 0,
            "icon": -1,
            "name": task_tag,
            "order": -1,
            "remoteId": tag_uid,
            "tagOrdering": "[]"
        });
        println!("{},", json);
      }
    }
  }
  tag_map
}

fn generate_task_lines(tasks: &Vec<Task>, tags: &HashMap<String, String>) -> Result<()> {
  println!("Tasks:");
  println!("---");
  for task in tasks {
    convert_to_tasks_org_task(task, tags);
  }

  Ok(())
}

fn random_number_generator(len: usize) -> String {
  use rand::Rng;
  const CHARSET: &[u8] = b"0123456789";
  let mut rng = rand::thread_rng();
  let out: String = (0..len)
    .map(|_| {
      let idx = rng.gen_range(0..CHARSET.len());
      CHARSET[idx] as char
    })
    .collect();
  out
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
struct Tag {
  name: String,
  #[serde(rename = "tagUid")]
  tag_uid: String,
}

fn extract_task_tags(task_tags: &Vec<String>, tags: &HashMap<String, String>) -> Vec<Tag> {
  let mut tags_out = Vec::new();
  for task_tag in task_tags {
    let tag_uid = tags.get(task_tag).unwrap().to_owned();

    let tag = Tag {
      name: task_tag.to_owned(),
      tag_uid,
    };
    tags_out.push(tag);
  }
  tags_out
}

fn to_priority_num(priority: &Priority) -> i32 {
  let priority_char: char = priority.to_owned().into();
  match priority_char {
    'A' => 0,
    'B' => 1,
    'C' => 2,
    _ => 3,
  }
}

fn to_unix_time(date: Option<Date>) -> i64 {
  let now = SystemTime::now();
  let unix_now = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
  let unix_now_millis = unix_now.as_millis() as i64;

  date
    .map(|t| t.and_hms_opt(12, 0, 0).unwrap().timestamp_millis())
    .unwrap_or(unix_now_millis)
}

fn to_title(subject: &str, tags: &Vec<Tag>) -> String {
  let mut title = subject.to_string();
  for tag in tags {
    title = title.replace(&format!("+{}", tag.name), "");
  }
  title.trim().to_string()
}

fn convert_to_tasks_org_task(task: &Task, tags: &HashMap<String, String>) {
  let task_tags = task.projects.to_owned();
  let out_tags = extract_task_tags(&task_tags, tags);
  let remote_id = random_number_generator(19);
  let task_creation_millis = to_unix_time(task.create_date);
  let title = to_title(&task.subject, &out_tags);
  let priority = to_priority_num(&task.priority);

  let out = json!({
          "alarms": [],
          "attachments": [],
          "comments": [],
          "geofences": [],
          "google": [],
          "locations": [],
          "tags": out_tags,
          "task": {
            "completionDate": 0,
            "creationDate": task_creation_millis,
            "deletionDate": 0,
            "dueDate": 0,
            "elapsedSeconds": 0,
            "estimatedSeconds": 0,
            "hideUntil": 0,
            "isCollapsed": false,
            "modificationDate": task_creation_millis,
            "priority": priority,
            "readOnly": false,
            "reminderLast": 0,
            "remoteId": remote_id,
            "repeatFrom": 0,
            "ringFlags": 0,
            "timerStart": 0,
            "title": title
          }
  }
  );
  println!("{},", out);
}
