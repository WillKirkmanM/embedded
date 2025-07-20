const MAX_TASKS: usize = 4;
static mut TASKS: [Option<fn()>; MAX_TASKS] = [None; MAX_TASKS];
static mut CURRENT_TASK: usize = 0;

/// Add a task to the scheduler. A task is just a function pointer.
pub fn add_task(task: fn()) -> Result<(), &'static str> {
    for i in 0..MAX_TASKS {
        // SAFETY: We are in a critical section (e.g., interrupts disabled)
        // when adding tasks to prevent race conditions.
        if unsafe { TASKS[i].is_none() } {
            unsafe { TASKS[i] = Some(task) };
            return Ok(());
        }
    }
    Err("Task list is full")
}

/// Run the scheduler. This function never returns.
pub fn run_scheduler() -> ! {
    loop {
        // SAFETY: Accessing static mut variables.
        unsafe {
            // Find the next task to run
            let next_task_index = (CURRENT_TASK + 1) % MAX_TASKS;
            for i in 0..MAX_TASKS {
                let current_index = (next_task_index + i) % MAX_TASKS;
                if let Some(task) = TASKS[current_index] {
                    CURRENT_TASK = current_index;
                    task(); // Run the task
                    // When the task returns, the loop continues to the next one.
                    break;
                }
            }
        }
    }
}

// Example tasks
fn task_one() { /* Do something... */ }
fn task_two() { /* Do something else... */ }

#[cortex_m_rt::entry]
fn main() -> ! {
    add_task(task_one).ok();
    add_task(task_two).ok();
    
    // Start the OS!
    run_scheduler();
}