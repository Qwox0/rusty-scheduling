use std::ops::{Add, Bound, Bound::*, RangeBounds};

use crate::{task::Task, tasks::Tasks};

/// currently only Earliest Deadline First (preemptive)
/// and Earliest Due Date (non-preemptive; must finish task)
pub struct Scheduler<F>
where
    F: Fn(&Task, &Task, usize) -> bool,
{
    is_preemptive: bool,
    replace_condition: F,
    waiting_tasks: Vec<Task>,
    active_task: Option<Task>,
}

impl<F> Scheduler<F>
where
    F: Fn(&Task, &Task, usize) -> bool,
{
    /// Returns a new scheduling algorithm
    ///
    /// # Arguments
    ///
    /// * `is_preemptive` - Whether the algorithm checks `replace_condition` every step
    /// * `replace_condition` - A clossure which takes the active Task and a new Task and returns
    /// wheather the new Task should be selected for as the active Task
    pub fn new(is_preemptive: bool, replace_condition: F) -> Self {
        Self {
            is_preemptive,
            replace_condition,
            waiting_tasks: vec![],
            active_task: None,
        }
    }

    pub fn get_scheduling(&mut self, tasks: &Tasks, max_step: Option<usize>) -> String {
        let total_period = tasks.get_total_period();
        let max_step: usize = match max_step {
            Some(n) if n <= total_period => n,
            _ => total_period,
        };

        let mut str = "".to_string();
        for step in 0..=max_step {
            self.refresh_tasks(tasks, step);
            self.find_active_task(step);
            if let Some(active_task) = &mut self.active_task {
                let s = active_task.execute();
                str.push_str(s.as_str());
                if active_task.is_done() {
                    self.active_task = None;
                }
            } else {
                str.push_str("x");
            }
        }
        if max_step == total_period {
            str.push_str(" repeating");
        }
        str
    }

    fn refresh_tasks(&mut self, tasks: &Tasks, step: usize) {
        tasks.iter().for_each(|t: &Task| {
            if step % t.period == 0 {
                self.waiting_tasks.push(t.clone())
            }
        });
    }

    fn find_active_task(&mut self, step: usize) {
        match self.active_task {
            Some(_) if !self.is_preemptive => (),
            _ => {
                (self.active_task, self.waiting_tasks) = find_active_task(
                    self.active_task.clone(),
                    self.waiting_tasks.clone(),
                    step,
                    &self.replace_condition,
                )
            }
        }
    }
}

fn find_active_task<F>(
    active_task: Option<Task>,
    waiting_tasks: Vec<Task>,
    step: usize,
    replace_condition: &F,
) -> (Option<Task>, Vec<Task>)
where
    F: Fn(&Task, &Task, usize) -> bool,
{
    waiting_tasks.into_iter().fold(
        (active_task, vec![]),
        |(active, mut waiting): (Option<Task>, Vec<Task>), task: Task| match active {
            //Some(active) if !replace_condition(&active, &task) => {
            Some(a) if replace_condition(&a, &task, step) => {
                waiting.push(task);
                (Some(a), waiting)
            }
            Some(a) => {
                waiting.push(a);
                (Some(task), waiting)
            }
            None => (Some(task), waiting),
        },
    )
}
