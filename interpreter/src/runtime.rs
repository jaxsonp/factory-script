use crate::{debug, function::*, *};

/// Spawns pallets from the start station and starts the execution loop, returns
/// the number of steps in the program
pub fn execute<'a>(program: &'a FSProgram) -> (Result<(), Error>, u64) {
    debug!(2, "Starting execution");
    let mut step_count: u64 = 0;

    // main function, "root" of the call tree
    let mut main: Function<'a> = Function::instantiate_main(&program);

    // execution loop
    'execute_loop: while !main.is_done() {
        match main.step() {
            Ok(false) => {}
            Ok(true) => {
                break 'execute_loop;
            }
            Err(e) => {
                return (Err(e), step_count);
            }
        };

        step_count += 1;
    }
    debug!(2, "Execution completed");

    return (Ok(()), step_count);
}
