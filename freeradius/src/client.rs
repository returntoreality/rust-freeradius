use std::ffi::CString;

use errors::*;
use radius;
use types::Attribute;
use std::ptr;

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

    pub fn auth(&mut self, client_port: u32, attributes: Vec<Attribute>) {
        unsafe {
            let result : *mut radius::VALUE_PAIR = ptr::null_mut();
            let list : Vec<radius::VALUE_PAIR> = attributes.map(|a| a.into());
            radius::rc_avpair_gen(self.rc_handle, list.as_ptr() as *mut VALUE_PAIR, )
            radius::rc_auth(self.rc_handle, client_port, , &result as *mut _ )
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


