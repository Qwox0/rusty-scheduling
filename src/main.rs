use task_scheduling::{scheduler::Scheduler, tasks::Tasks, task::Task};

fn main() {
    let tasks1 = Tasks::from(vec![(1,4),(2,5),(2,7)]);
    let tasks2 = Tasks::from(vec![(1,3),(1,5),(2,7)]);

    let compare_deadline = |active: &Task, next: &Task, step:usize| active.until_deadline(step) <= next.until_deadline(step);
    // Earliest Due Date
    let mut edd = Scheduler::new(false, compare_deadline);
    // Earliest Deadline First
    let mut edf = Scheduler::new(true, compare_deadline);

    let tasks = tasks1;
    println!("{}", tasks);
    println!("Earliest Due Date:        {}", edd.get_scheduling(&tasks, None));
    println!("Earliest Deadline First:  {}", edf.get_scheduling(&tasks, None));
}
