use std::{fmt::Display, slice::Iter};

use crate::task::{Task, TaskState};

#[derive(Debug, Clone)]
pub struct Tasks {
    tasks: Vec<Task>,
}

impl From<Vec<(usize, usize)>> for Tasks {
    fn from(tasks: Vec<(usize, usize)>) -> Self {
        let s = Self {
            tasks: tasks
                .into_iter()
                .enumerate()
                .map(|(i, (work, period))| Task::new(format!("{}", i + 1), work, period))
                .collect::<Vec<Task>>(),
        };
        if s.get_total_utilization() > 1.0 {
            println!("WARN: too much work! :(");
        }
        s
    }
}

impl Tasks {
    pub fn get_total_period(&self) -> usize {
        lcm(self.tasks.iter().map(|t| t.period).collect())
    }

    pub fn get_total_utilization(&self) -> f64 {
        self.tasks.iter().fold(0.0, |acc: f64, t:&Task| {
            acc + t.get_utilization()
        })
    }

    pub fn iter(&self) -> Iter<'_, Task> {
        self.tasks.iter()
    }
}

impl Display for Tasks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tasks (utilization: {}%, total_period: {}):", self.get_total_utilization()*100.0, self.get_total_period())?;
        for task in self.tasks.iter() {
            writeln!(
                f,
                "Task {}: {}/{}",
                task.name,
                task.get_remaining_work(),
                task.until_deadline(0)
            )?;
        }
        Ok(())
    }
}

/// see: https://en.wikipedia.org/wiki/Least_common_multiple#Using_a_simple_algorithm
fn lcm(nums: Vec<usize>) -> usize {
    match nums.len() {
        0 => 0,
        _ => {
            let mut arr = nums.clone();
            loop {
                let start = (0, arr[0]);
                if arr.iter().all(|n: &usize| *n == start.1) {
                    break start.1;
                }
                let (i, _) = arr.iter().enumerate().skip(1).fold(start, |acc, (i, num)| {
                    if *num < acc.1 {
                        (i, *num)
                    } else {
                        acc
                    }
                });
                arr[i] += nums[i];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lcm() {
        let lcm = super::lcm(vec![3]);
        assert_eq!(lcm, 3);

        let lcm = super::lcm(vec![3, 5]);
        assert_eq!(lcm, 15);

        let lcm = super::lcm(vec![3, 5, 7]);
        assert_eq!(lcm, 105);

        let lcm = super::lcm(vec![3, 3, 3]);
        assert_eq!(lcm, 3);
    }
}
