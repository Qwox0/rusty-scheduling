use std::ops::{Add, Bound, Bound::*, RangeBounds };

use crate::{task::Task, tasks::Tasks};

/// currently only Earliest Deadline First (preemptive)
/// and Earliest Due Date (non-preemptive; must finish task)
pub struct Scheduler {
    is_preemptive: bool,
}

impl Scheduler {
    pub fn new(is_preemptive: bool) -> Self {
        Self { is_preemptive }
    }

    pub fn get_scheduling<R>(&self, mut tasks: Tasks, cycle_range: R) -> String
    where
        R: RangeBounds<usize> + Iterator,
    {
        let total_period = tasks.get_total_period();
        let range = 0..cap_bound(cycle_range.end_bound(), total_period);

        let str = "";

        let mut active_task: Option<&mut Task> = None;

        active_task = Some(tasks.test());

        for step in range {
            match &mut active_task {
                Some(task) if !self.is_preemptive => task.execute(),
                _ => active_task = tasks.get_active_task(step),
            }
        }
        String::from(str)
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
