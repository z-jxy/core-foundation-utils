use std::ffi::c_void;
use std::path::Path;
use std::str::FromStr;

use core_foundation::number::{CFBooleanGetTypeID, CFNumberGetTypeID};
use core_foundation::string::CFStringGetTypeID;

use crate::error::MetadataError;
use crate::from_objc::FromObjC;
use crate::prelude::*;
/// Metadata Service

/// Query a file attribute using the Metadata Service
pub fn query_file_attr<P, T>(file_name: P, attr_name: &str) -> Result<T, MetadataError>
where
    P: AsRef<Path>,
    T: FromObjC,
{
    let path = file_name
        .as_ref()
        .to_str()
        .expect("path is not valid unicode");
    let file_cfstring = CFString::new(path);
    let item = unsafe { MDItemCreate(kCFAllocatorDefault, file_cfstring.as_concrete_TypeRef()) };
    if item.is_null() {
        // eprintln!("Failed to create MDItem");
        return Err(MetadataError::NullItem {
            item: path.to_string(),
        });
    }

    let value = unsafe {
        MDItemCopyAttribute(
            item,
            CFString::from_str(attr_name).unwrap().as_concrete_TypeRef(),
        )
    };

    if value.is_null() {
        // eprintln!("Failed to get attribute value");
        return Err(MetadataError::NullAttribute {
            attr: attr_name.to_string(),
        });
    }

    // validate the type of the value we're casting to
    // if we don't check and the type is invalid we will crash due to ffi exception
    match T::from_objc(value) {
        Some(v) => Ok(v),
        None => {
            return Err(MetadataError::ConversionError {
                attr: attr_name.to_string(),
                // expected_type: T::type_id().to_string(),
                real_type: trace_type_id(value).to_string(),
            });
        }
    }
}

fn trace_type_id(value: *const c_void) -> &'static str {
    unsafe {
        let type_id = CFGetTypeID(value);

        if type_id == CFStringGetTypeID() {
            "CFString"
        } else if type_id == CFNumberGetTypeID() {
            "CFNumber"
        } else if type_id == CFArrayGetTypeID() {
            "CFArray"
        } else if type_id == CFBooleanGetTypeID() {
            "CFBoolean"
        } else {
            "Unknown"
        }
    }
}
