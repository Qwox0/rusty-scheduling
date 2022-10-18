use task_scheduling::{scheduler::Scheduler, tasks::Tasks};

fn main() {
    let mut tasks = Tasks::from(vec![(1, 4), (2, 5), (2, 7)]);
    let scheduler = Scheduler::new(false);
    println!("{}", tasks);
    println!("{}", scheduler.get_scheduling(&mut tasks, 0..=30));
}
