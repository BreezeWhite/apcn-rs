use clap::{Parser, Subcommand};

/// Extremely fast Arbitrary Precision Constant Numbers calculator.
#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Option<Actions>,

    /// Precision of Pi to calculate in digits.
    #[arg(short, long, default_value_t = 1000, global = true)]
    pub digits: u32,

    /// Parallelize the calculation. Please only use under extreme precision (>100k).
    /// If enabled, half of the CPU cores will be used in computation.
    #[arg(long, default_value_t = false, global = true)]
    pub parallel: bool,

    /// Benchmark the computation time
    #[arg(short, long, default_value_t = false, global = true)]
    pub bench: bool,

    /// Just do the calculation but not format and output the number.
    /// Used for pure speed benching.
    #[arg(long, default_value_t = false, global = true)]
    pub no_print: bool,

    /// Show current backend
    #[arg(long, default_value_t = false)]
    pub backend: bool,
}

#[derive(Subcommand, Clone)]
pub enum Actions {
    /// Pi, calculated with Chudnovsky algorithm.
    Pi,
    /// Euler's number. Calculated with Taylor series expansion.
    E,
    /// Natural log 2.
    Ln2,
    /// Natural log 3. Use generic log algorithm to calculate.
    Ln3,
    /// Natural log 5. Use generic log algorithm to calculate.
    Ln5,
    /// Square root of 2. *Non-parallelizable*
    Sqrt2,
    /// Square root of 3. *Non-parallelizable*
    Sqrt3,
    /// Square root of 5. *Non-parallelizable*
    Sqrt5,
    /// Golden ratio.
    Phi,
    /// Euler-Mascheroni constant.
    Gamma,
}
