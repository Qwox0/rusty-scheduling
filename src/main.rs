use task_scheduling::{tasks::Tasks, scheduler::Scheduler};

fn main() {
    let mut tasks = Tasks::from(vec![(1,4), (2,5), (2,7)]);
    let scheduler = Scheduler::new(false);
    println!("{}", tasks);
    println!("{}", scheduler.get_scheduling(tasks, 0..=30));

}
