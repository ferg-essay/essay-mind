mod digit;
mod gram;

pub use digit::Digit;
pub use gram::{Gram, gram};

#[cfg(test)]
mod test_digit;
#[cfg(test)]
mod test_gram;
