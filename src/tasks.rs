use std::fmt::Display;

use crate::task::Task;

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
                .map(|(work, period)| Task::new(work, period))
                .collect::<Vec<Task>>(),
        )
    }
}

impl Tasks {
    pub fn get_total_period(&self) -> usize {
        self.tasks.iter().fold(1, |acc: usize, t| acc * t.period)
    }

    pub fn get_active_task(&self, step: usize) -> Option<&mut Task> {
        todo!()
    }

    pub fn test(&mut self) -> &mut Task {
        &mut self.tasks[0]
    }
}

fn get_utilization(vec: &Vec<Task>) -> f64 {
    vec.iter()
        .fold(0.0, |acc: f64, t| acc + t.get_total_utilization())
}

impl Display for Tasks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tasks:")?;
        for (i, task) in self.tasks.iter().enumerate() {
            writeln!(
                f,
                "Task {i}: {} {}",
                task.get_remaining_work(),
                task.until_deadline(0)
            )?;
        }
        Ok(())
    }
}
