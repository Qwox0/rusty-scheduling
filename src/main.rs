use task_scheduling::{scheduler::Scheduler, tasks::Tasks, task::Task};

fn main() {
    //let tasks = Tasks::from(vec![(1, 4), (2, 5), (2, 7)]);
    let tasks = vec![Task::new("1".to_string(),1,4),Task::new("2".to_string(),2,5),Task::new("3".to_string(),2,7)];

    let mut scheduler = Scheduler::new(false, |active,next| true);
    println!("{}", scheduler.get_scheduling(tasks, 0..=30));
}
