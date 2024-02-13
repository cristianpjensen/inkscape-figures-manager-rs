// Only MacOS is supported, but this can easily be extended by adding another
// file

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;
