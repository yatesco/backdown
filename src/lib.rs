#[macro_use]
extern crate cli_log;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub mod args;
pub mod ask;
pub mod dirs;
pub mod dup;
pub mod dup_report;
pub mod ext;
pub mod file_pair;
pub mod hash;
pub mod removal_report;

pub use {
    args::*, ask::*, dirs::*, dup::*, dup_report::*, ext::*, file_pair::*, hash::*,
    removal_report::*,
};
