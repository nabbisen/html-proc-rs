//! HTML processors as utils.
//! Each function is offered as a single `feature`, so the dependencies are kept small. (`omit_enclosure` which is used as document outline formatter is exception.)

// #[cfg(feature = "omit_attr")]
pub mod omit_attr;
pub mod omit_enclosure;
// #[cfg(feature = "path_to_url")]
pub mod path_to_url;

mod core;
