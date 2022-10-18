#[derive(Debug, Clone)]
pub struct Task {
    pub duration: usize,
    pub period: usize,
    state: TaskState,
}

impl Task {
    pub fn new(time: usize, period: usize) -> Self {
        Self {
            duration: time,
            period,
            state: TaskState::InProgress(0),
        }
    }

    pub fn get_total_utilization(&self) -> f64 {
        self.duration as f64 / self.period as f64
    }

    pub fn get_remaining_work(&self) -> usize {
        match self.state {
            TaskState::InProgress(x) => self.duration - x,
            TaskState::Done => 0,
        }
    }

    // 0 1 2 3 4 5 6 7
    // 3 2 1 3 2 1 3 2
    //%0 1 2 0 1 2 0 1

    pub fn until_deadline(&self, step:usize) -> usize {
        self.period - step % self.period
    }

    pub fn execute(&mut self) {
        match &mut self.state {
            TaskState::InProgress(x) if *x + 1 < self.duration=> *x += 1,
            s => *s = TaskState::Done,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TaskState {
    InProgress(usize),
    Done,
}