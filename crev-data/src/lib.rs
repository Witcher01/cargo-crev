//! This crate contains only code handling data types
//! used by `crev`, without getting into details
//! how actually `crev` works (where and how it manages data).







#[macro_use]
extern crate failure;




#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate derive_builder;


use common_failures::prelude::*;

pub mod review {
    pub use super::proof::review::*;
}
pub mod trust {
    pub use super::proof::trust::*;
}

pub mod id;
pub mod level;
pub mod proof;
pub mod util;

#[cfg(test)]
mod tests;
