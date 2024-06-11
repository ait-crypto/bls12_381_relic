//! Bindings for [relic]
//!
//! This crate provides bindings for the functions exposed by relic configured
//! for the use of the pairing of the pairing-friendly curve BLS12-381.
//! Additionally, the crate also provides additional wrapper functions to ease
//! the work with [relic].
//!
//! [relic]: https://github.com/relic-toolkit/relic

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init() {
        unsafe {
            assert_eq!(core_init(), RLC_OK);
            assert_eq!(core_clean(), RLC_OK);
        }
    }
}
