/* Copyright (c) 2016 Mike	Lubinets
 * Originally written by Mike Lubinets
 *
 * See LICENSE (MIT) */

use ansi_term::Colour::*;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct TaskList {
    tasks: Vec<Task>,
}

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord)]
pub struct Task {
    complete:    bool,
    num:         u16,
    task:        String,
    description: String,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            tasks: Vec::new()
        }
    }

    pub fn push(&mut self, task: Task) {
        self.tasks.push(task);
        self.tasks.sort();
    }

    pub fn new_task(&mut self, task: &str, description: &str) {
        let num = self.next_num();
        self.push(
            Task::new(
                num,
                task,
                description,
            )
        );
        self.tasks.sort();
    }

    pub fn complete(&mut self, num: u16) {
        for task in self.tasks.iter_mut() {
            if task.num == num {
                task.complete = true;
            }
        }
        self.tasks.sort();
    }

    pub fn delete(&mut self, num: u16) {
        for i in 0..self.tasks.len() {
            if self.tasks[i].num == num {
                self.tasks.remove(i);
            }
        }
        self.tasks.sort();
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
    }

    pub fn to_short_string(&self) -> String {
        let mut string: String = "".to_owned();
        for task in &self.tasks {
            string.push_str(&task.to_short_string());
            string.push('\n');
        }
        string
    }


    pub fn to_short_colored(&self) -> String {
        let mut string: String = "".to_owned();
        for task in &self.tasks {
            string.push_str(&task.to_short_colored());
            string.push('\n');
        }
        string
    }

    pub fn to_string(&self) -> String {
        let mut string: String = "".to_owned();
        for task in &self.tasks {
            string.push_str(&task.to_string());
            string.push('\n');
        }
        string
    }

    pub fn to_colored(&self) -> String {
        let mut string: String = "".to_owned();
        for task in &self.tasks {
            string.push_str(&task.to_colored());
            string.push('\n');
        }
        string
    }

    fn next_num(&self) -> u16 {
        self.tasks.iter().fold(0, |max, ref task| if task.num > max { task.num } else { max }) + 1
    }
}

impl Task {
    pub fn new(num: u16, task: &str, description: &str) -> Task {
        Task {
            num:         num,
            task:        task.to_owned(),
            description: description.to_owned(),
            complete:    false
        }
    }

    pub fn to_short_string(&self) -> String {
        let num_str = format!("{:3}", self.num);
        format!(
            "{}: {}",
            num_str,
            self.task,
        )
    }


    pub fn to_short_colored(&self) -> String {
        let num_str = format!("{:3}", self.num);
        format!(
            "{}: {}",
            if self.complete { Green.bold().paint(num_str) }
            else             { Red.bold().paint(num_str) },
            White.underline().paint(self.task.as_ref()),
        )
    }

    pub fn to_string(&self) -> String {
        let num_str = format!("{:3}", self.num);
        format!(
            "{}: {} {} {} {}",
            num_str,
            self.task,
            if self.description.len() > 1 { "\n     "} else { "" },
            self.description,
            if self.description.len() > 1 { "\n     "} else { "" },
        )
    }

    pub fn to_colored(&self) -> String {
        let num_str = format!("{:3}", self.num);
        format!(
            "{}: {} {} {} {}",
            if self.complete { Green.bold().paint(num_str) }
            else             { Red.bold().paint(num_str) },
            White.underline().paint(self.task.as_ref()),
            if self.description.len() > 1 { "\n     "} else { "" },
            White.paint(self.description.as_ref()),
            if self.description.len() > 1 { "\n     "} else { "" },
        )
    }
}
