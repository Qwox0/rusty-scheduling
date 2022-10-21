#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub duration: usize,
    pub period: usize,
    pub state: TaskState,
}

impl Task {
    pub fn new(name: String, duration: usize, period: usize) -> Self {
        Self {
            name,
            duration,
            period,
            state: TaskState::InProgress(0),
        }
    }

    pub fn get_remaining_work(&self) -> usize {
        match self.state {
            TaskState::InProgress(x) => self.duration - x,
            TaskState::Done => 0,
        }
    }

    pub fn get_utilization(&self) -> f64 {
        self.duration as f64 / self.period as f64
    }

    pub fn is_done(&self) -> bool {
        if let TaskState::Done = self.state {
            true
        } else {
            false
        }
    }

    pub fn until_deadline(&self, step: usize) -> usize {
        self.period - step % self.period
    }

    pub fn execute(&mut self) -> String {
        match &mut self.state {
            TaskState::InProgress(x) if *x + 1 < self.duration => *x += 1,
            s => *s = TaskState::Done,
        };
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
pub enum TaskState {
    InProgress(usize),
    Done,
}
