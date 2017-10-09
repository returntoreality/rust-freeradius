#![deny(missing_debug_implementations, missing_copy_implementations,
         trivial_casts, trivial_numeric_casts,
         unstable_features,
//       missing_docs
         unused_import_braces, unused_qualifications)]
#![recursion_limit = "1024"]

extern crate freeradius_sys as radius;
extern crate num;
extern crate try_from;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate quick_error;


mod errors {
    use std::str::Utf8Error;
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{
        foreign_links {
            CString(::std::ffi::NulError);
        }
        errors {
            ConfigFileError {
                description("Error reading radius config file")
                display("Error reading radius config file")
            }
            ValuePairConversionError {
                description("Error converting value_pair")
                display("Error converting value_pair")
                from(Utf8Error)
            }
        }
    }
}

pub use errors::*;

mod types;
mod client;

pub use types::*;
pub use client::*;

#[cfg(test)]
mod tests {
    use client::RadiusClient;
    #[test]
    fn create_new_client() {
        let client = RadiusClient::new();
        println!("{:?}", client);
    }
    fn create_from_config() {
        let client = RadiusClient::from_file("tests/resources/working.conf").unwrap();
    }
}
