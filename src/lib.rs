#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod lib_c {
    include!(concat!(env!("OUT_DIR"), "/binding.rs"));
}
