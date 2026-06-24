use clap::{Parser, Subcommand};

/// Extremely fast Arbitrary Precision Constant Numbers calculator.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Actions,

    /// Precision of Pi to calculate in digits.
    #[arg(short, long, default_value_t = 1000)]
    pub digits: u32,

    /// Parallelize the calculation. Please only use under extreme precision (>100k).
    /// If enabled, half of the CPU cores will be used in computation.
    #[arg(long, default_value_t = false)]
    pub parallel: bool,

    /// Benchmark the computation time
    #[arg(short, long, default_value_t = false)]
    pub bench: bool,
}

#[derive(Subcommand, Clone)]
pub enum Actions {
    Pi,
    E,
    Ln2,
    Ln3,
    Ln5,
    Sqrt2,
    Sqrt3,
    Sqrt5,
}
