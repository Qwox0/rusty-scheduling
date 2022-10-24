use crate::{task::Task, tasks::Tasks};

/// currently only Earliest Deadline First (preemptive)
/// and Earliest Due Date (non-preemptive; must finish task)
pub struct Scheduler {
    is_preemptive: bool,
    conditions: Vec<SchedulerCondition>,
    waiting_tasks: Vec<Task>,
    active_task: Option<Task>,
}

impl Scheduler {
    /// Returns a new scheduling algorithm
    ///
    /// # Arguments
    ///
    /// * `is_preemptive` - Whether the algorithm checks `replace_condition` every step
    /// * `replace_condition` - A clossure which takes the active Task and a new Task and returns
    /// wheather the new Task should be selected for as the active Task
    pub fn new(is_preemptive: bool, conditions: Vec<SchedulerCondition>) -> Self {
        Self {
            is_preemptive,
            conditions,
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
        for step in 1..=max_step {
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
            // period == deadline. task isn't available again on the deadline (only the cycle after)
            if step % t.period == 1 {
                self.waiting_tasks.push(t.clone())
            }
        });
    }

    fn find_active_task(&mut self, step: usize) {
        if self.active_task.is_some() && !self.is_preemptive {
            return;
        }
        self.waiting_tasks =
            self.waiting_tasks
                .clone()
                .into_iter()
                .fold(vec![], |mut waiting: Vec<Task>, task| {
                    for condition in &self.conditions {
                        match condition.do_replace(&self.active_task, &task, step) {
                            ConditionsResult::Replace => {
                                if let Some(active) = self.active_task.clone() {
                                    waiting.push(active);
                                }
                                self.active_task = Some(task);
                                return waiting;
                            }
                            ConditionsResult::Keep => {
                                waiting.push(task);
                                return waiting;
                            }
                            ConditionsResult::NextCondition => continue,
                        }
                    }
                    // default: no replace (first come first serve)
                    waiting.push(task);
                    waiting
                })
    }
}

pub enum SchedulerCondition {
    MinDeadline,
    FCFS, // First Come First Serve
}

pub enum ConditionsResult {
    Replace,
    Keep,
    NextCondition,
}

impl SchedulerCondition {
    pub fn do_replace(&self, active: &Option<Task>, next: &Task, step: usize) -> ConditionsResult {
        use ConditionsResult::*;
        use SchedulerCondition::*;
        if let Some(a) = active {
            match self {
                MinDeadline => match (a.until_deadline(step), next.until_deadline(step)) {
                    (ad, nd) if ad == nd => NextCondition,
                    (ad, nd) if ad > nd => Replace,
                    (_, _) => Keep,
                },
                FCFS => Keep,
            }
        } else {
            return Replace;
        }
    }
}
