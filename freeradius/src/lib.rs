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

mod errors {
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
        }
    }
}

pub use errors::*;

mod types;
pub use types::*;


use std::ffi::CString;


#[derive(Debug)]
pub struct RadiusClient {
    rc_handle: *mut radius::rc_handle
}

impl RadiusClient {
    pub fn new() -> RadiusClient {
        unsafe {
            RadiusClient {
                rc_handle: radius::rc_new()
            }
        }
    }

    pub fn from_file (file: &str) -> Result<RadiusClient> {
        unsafe {
            let result = radius::rc_read_config(CString::new(file)?.into_raw());
            if result.is_null() {
                Err(ErrorKind::ConfigFileError.into())
            } else {
                Ok(RadiusClient {
                    rc_handle: result 
                })
            }
        }
    }
}

impl Drop for RadiusClient {
    fn drop(&mut self) {
        unsafe {
            radius::rc_destroy(self.rc_handle);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::RadiusClient;
    #[test]
    fn it_works() {
        {
            let client = RadiusClient::new();
            println!("{:?}", client);
        }
    }
}
