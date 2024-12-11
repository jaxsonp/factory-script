use std::time::Instant;

pub mod error;
pub mod function;
pub mod pallet;
pub mod station;
pub mod util;

pub use error::{
    Error,
    ErrorType::{self, *},
};
pub use pallet::Pallet;

mod preprocessor;
mod runtime;

use function::FunctionTemplate;

pub static mut COLOR_OUTPUT: bool = false;
pub static mut DEBUG_LEVEL: u8 = 0;

pub fn run(src: &str, print_benchmark: bool) -> Result<(), Error> {
    let start_time = Instant::now();

    debug!(2, "Preprocessing...");
    let (/*mut _main, mut _function_templates*/) = preprocessor::process(src)?;

    let runtime_start_time = Instant::now();
    debug!(2, "Starting");
    let mut step_count: u64 = 0;
    //runtime::execute(&mut stations, &mut step_count)?;

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
