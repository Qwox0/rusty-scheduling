use task_scheduling::{
    scheduler::{Scheduler, SchedulerCondition::*},
    tasks::Tasks,
};

fn main() {
    #[allow(unused)]
    let tasks1 = Tasks::from(vec![(1, 4), (2, 5), (2, 7)]);
    #[allow(unused)]
    let tasks2 = Tasks::from(vec![(1, 3), (1, 5), (2, 7)]);
    #[allow(unused)]
    let tasks3 = Tasks::from(vec![(1, 3), (1, 4), (2, 5)]);

    // Earliest Due Date
    let mut edd = Scheduler::new(false, vec![MinDeadline, FCFS]);
    // Earliest Deadline First
    let mut edf = Scheduler::new(true, vec![MinDeadline, FCFS]);

    let tasks = tasks3;
    println!("{}", tasks);
    println!(
        "Earliest Due Date:        {}",
        edd.get_scheduling(&tasks, None)
    );
    println!(
        "Earliest Deadline First:  {}",
        edf.get_scheduling(&tasks, None)
    );
}
