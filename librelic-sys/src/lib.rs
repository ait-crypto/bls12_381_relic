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
