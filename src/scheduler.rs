use std::ops::{Add, Bound, Bound::*, RangeBounds};

use crate::{task::Task, tasks::Tasks};

/// currently only Earliest Deadline First (preemptive)
/// and Earliest Due Date (non-preemptive; must finish task)
pub struct Scheduler<F>
where
    F: Fn(&Task, &Task) -> bool,
{
    is_preemptive: bool,
    replace_condition: F,
    waiting_tasks: Vec<Task>,
    active_task: Option<Task>,
}

impl<F> Scheduler<F>
where
    F: Fn(&Task, &Task) -> bool,
{
    pub fn new(is_preemptive: bool, replace_condition: F) -> Self {
        Self {
            is_preemptive,
            replace_condition,
            waiting_tasks: vec![],
            active_task: None,
        }
    }

    pub fn get_scheduling<R>(&mut self, tasks: Vec<Task>, cycle_range: R) -> String
    where
        R: RangeBounds<usize> + Iterator,
    {
        //let total_period = tasks.get_total_period();
        //let range = 0..cap_bound(cycle_range.end_bound(), total_period);
        let mut str = "".to_string();

        for step in 0..30 {
            self.refresh_tasks(step, &tasks);
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

        str
    }

    fn refresh_tasks(&mut self, step: usize, tasks: &Vec<Task>) {
        tasks.iter().for_each(|t: &Task| {
            if step % t.period == 0 {
                self.waiting_tasks.push(t.clone())
            }
        });
    }

    fn find_active_task(&mut self, step:usize) {
        (self.active_task, self.waiting_tasks) = find_active_task(self.active_task.clone(), self.waiting_tasks.clone(), step)
    }
}

fn cap_bound<T>(bound: Bound<&T>, max: T) -> T
where
    T: Copy + PartialOrd + Add<usize, Output = T>,
{
    match bound {
        Included(x) => {
            if x > &max {
                max + 1
            } else {
                *x + 1
            }
        }
        Excluded(x) => {
            if x > &max {
                max
            } else {
                *x
            }
        }
        Unbounded => max,
    }
}

fn find_active_task(
    active_task: Option<Task>,
    waiting_tasks: Vec<Task>,
    step: usize,
) -> (Option<Task>, Vec<Task>)
{
    waiting_tasks
        .into_iter()
        .fold(
            (active_task, vec![]),
            |(active,mut waiting): (Option<Task>, Vec<Task>), task: Task| match active {
                //Some(active) if !replace_condition(&active, &task) => {
                Some(a) if a.until_deadline(step) >= task.until_deadline(step) => {
                    waiting.push(task);
                    (Some(a),waiting)
                },
                Some(a) => {
                    waiting.push(a);
                    (Some(task),waiting)
                },
                None => (Some(task),waiting),
            },
        )
}
