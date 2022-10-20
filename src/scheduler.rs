use std::ops::{Add, Bound, Bound::*, RangeBounds};

use crate::{task::Task, tasks::Tasks};

/// currently only Earliest Deadline First (preemptive)
/// and Earliest Due Date (non-preemptive; must finish task)
pub struct Scheduler {
    is_preemptive: bool,
    waiting_tasks: Vec<Task>,
    active_task: Option<Task>,
}

impl Scheduler {
    pub fn new(is_preemptive: bool) -> Self {
        Self {
            is_preemptive,
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
            self.refresh_tasks(&tasks);

            match &self.active_task {
                Some(task) if task.is_done() => self.active_task = None,
                _ => (),
            }
        }

        str
    }

    fn refresh_tasks(&mut self, tasks: &Vec<Task>) {

        //self.waiting_tasks.append()
    }

    fn find_active_task(&mut self) {}
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
