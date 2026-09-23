//! ## Flow
pub mod compiler;
pub mod context;
pub mod error;
pub mod instruction;
pub mod instructions;
pub mod procedure;
pub mod slots;
pub mod variable;


/*

It's likely better to follow this strategy.

Define extra notation called 'spliter' and 'merger'.

For each instruction with zero dependencies is called in it's own task.
Each JoinHandle<()> is sent to stored list
A channel is provided to each task to notify of child nodes etc.

Invoke instruction, resources in slots are guarded by Mutex<...>. MutexGuard is passed into each and it's upto instruction when to drop it.
    (This allows instruction macro to either Clone, or hold the guard, or something else)

After completion, determine which branch to follow based on output.

Determine if the branch splits, merges, exists, or next instruction

If nothing exit
If next instruction
    Execute instruction
If splits
    Create two new tasks for each split
    Send JoinHandle<> over channel
    exit
If merges
    Notify merge manager task (mapping each merge point to pair queue)
        - Merge manager task will spawn new tasks when needed

The entry task should review allocated tasks (sent over channel) for
    - timeouts
    - runtime errors etc

Once all tasks are complete (?) How would this actually work
    - Exit

*/


pub fn compile() {}

pub async fn execute() {}

#[cfg(test)]
mod tests {}
