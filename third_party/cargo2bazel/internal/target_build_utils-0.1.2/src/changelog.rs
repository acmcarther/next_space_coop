//! Project Changelog

/// Release 0.1.2 (2016-10-17)
///
/// * Now figures out target info from the rustc that is used to compile the library. This results
/// in less divergence between versions of rustc (i.e. when targets are added), but is not able to
/// provide target info for some targets on some hosts anymore. For example all `*-apple-ios`
/// targets are not available anymore on the linux host.
/// * `Error` implements `std::error::Error` trait now.
pub mod r0_1_2 {}
