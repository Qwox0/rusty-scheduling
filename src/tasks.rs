use std::fmt::Display;

use crate::task::{Task, TaskState};

#[derive(Debug, Clone)]
pub struct Tasks {
    tasks: Vec<Task>,
    pub utilization: f64,
}

impl From<Vec<Task>> for Tasks {
    fn from(tasks: Vec<Task>) -> Self {
        let utilization = get_utilization(&tasks);
        if utilization > 1.0 {
            println!("WARN: too much work! :(")
        }
        Self { tasks, utilization }
    }
}

impl From<Vec<(usize, usize)>> for Tasks {
    fn from(tasks: Vec<(usize, usize)>) -> Self {
        Self::from(
            tasks
                .into_iter()
                .enumerate()
                .map(|(i, (work, period))| Task::new(format!("{}", i), work, period))
                .collect::<Vec<Task>>(),
        )
    }
}

impl Tasks {
    pub fn get_total_period(&self) -> usize {
        self.tasks.iter().fold(1, |acc: usize, t| acc * t.period)
    }

    pub fn get_active_task(&mut self, step: usize) -> Option<&mut Task> {
        self.tasks
            .iter_mut()
            .fold(None, |selected: Option<&mut Task>, t| match (selected, t) {
                (sel, t) if t.is_done() => sel,
                (Some(t1), t2) if t1.until_deadline(step) <= t2.until_deadline(step) => Some(t1),
                (_, t) => Some(t),
            })
    }

    pub fn get(&mut self, step: usize) {
        for t in self.tasks.iter_mut() {
            if step != 0 && step % t.period == 0 {
                if !t.is_done() {
                    println!("ERROR")
                }
                t.state = TaskState::InProgress(0)
            }
        }
    }
}

fn get_utilization(vec: &Vec<Task>) -> f64 {
    vec.iter()
        .fold(0.0, |acc: f64, t| acc + t.get_total_utilization())
}

impl Display for Tasks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tasks:")?;
        for task in self.tasks.iter() {
            writeln!(
                f,
                "Task {}: {} {}",
                task.name,
                task.get_remaining_work(),
                task.until_deadline(0)
            )?;
        }
        Ok(())
    }
}
