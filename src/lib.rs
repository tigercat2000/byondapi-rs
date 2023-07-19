// Ignore style warnings for for byondapi-c bindings
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

#[cfg(not(target_pointer_width = "32"))]
compile_error!("BYOND API only functions with 32-bit targets");

#[cfg(not(target_arch = "x86"))]
compile_error!("BYOND API only functions on x86 targets");

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
compile_error!("BYOND API only supports Windows and Linux");

// Include byondapi-c bindings (generated by build.rs)
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
