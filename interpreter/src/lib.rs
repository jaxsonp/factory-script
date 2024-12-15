use std::time::Instant;

pub mod error;
pub mod function;
pub mod pallet;
pub mod station;
pub mod util;

mod preprocessor;
mod runtime;

pub use error::{
    Error,
    ErrorType::{self, *},
};
use function::FunctionTemplate;
pub use pallet::Pallet;

pub static MAX_RECURSION_DEPTH: u32 = 1000;
pub static mut COLOR_OUTPUT: bool = false;
pub static mut DEBUG_LEVEL: u8 = 0;

pub fn run(src: &str, print_benchmark: bool) -> Result<(), Error> {
    let start_time = Instant::now();

    let mut program = preprocessor::process(src)?;

    program.benchmark = print_benchmark;

    let runtime_start_time = Instant::now();
    let (res, step_count) = runtime::execute(&program);
    if res.is_err() {
        return res;
    }

    if print_benchmark {
        let end_time = Instant::now();
        let preprocess_duration: f64 =
            ((runtime_start_time - start_time).as_nanos() as f64) / 1000000000f64;
        let runtime_duration: f64 =
            ((end_time - runtime_start_time).as_nanos() as f64) / 1000000000f64;
        let total_duration: f64 = ((end_time - start_time).as_nanos() as f64) / 1000000000f64;

        let avg_step_duration =
            ((end_time - runtime_start_time).as_nanos() as f64) / ((1000 * step_count) as f64);

        println!("\n======Benchmark======");
        println!(" steps      {}", step_count);
        println!(" avg step   {:.2}ms", avg_step_duration);
        println!();
        println!(" preprocess {:.5}s", preprocess_duration);
        println!(" runtime    {:.5}s", runtime_duration);
        println!(" total      {:.5}s", total_duration);
        println!("=====================");
    }

    Ok(())
}

/// Represents the output of the preprocessor/input to the runtime module
pub struct FSProgram {
    main: FunctionTemplate,
    function_templates: Vec<FunctionTemplate>,
    benchmark: bool,
}
