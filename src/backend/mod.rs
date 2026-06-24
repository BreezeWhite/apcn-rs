#[cfg(feature = "rug")]
mod rug;
#[cfg(feature = "rug")]
pub use rug::{BigFloat, BigInt};

#[cfg(feature = "dashu")]
mod dashu;
#[cfg(feature = "dashu")]
pub use dashu::{BigFloat, BigInt};

#[cfg(all(feature = "rug", feature = "dashu"))]
compile_error!("Features 'rug' and 'dashu' are mutually exclusive backends. Please enable only one.");

#[cfg(not(any(feature = "rug", feature = "dashu")))]
compile_error!("At least one backend feature ('rug' or 'dashu') must be enabled.");

