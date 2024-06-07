use core_foundation::string::{kCFStringEncodingUTF8, CFStringGetCString};

use crate::prelude::*;
use std::ptr;

pub struct SpotlightApi;

impl SpotlightApi {
    pub fn raw_query(query_string: impl Into<String>, item_type: KMDItemTypes) -> Vec<String> {
        let query_string = CFString::new(&query_string.into());
        let item_type = unsafe { item_type.into_ref() };

        let query = unsafe {
            MDQueryCreate(
                kCFAllocatorDefault,
                query_string.as_concrete_TypeRef(),
                ptr::null(),
                ptr::null(),
            )
        };

        if query.is_null() {
            eprintln!("Failed to create query");
            std::process::exit(1);
        }

        if unsafe { MDQueryExecute(query, K_MDQUERY_SYNCHRONOUS) } == 0 {
            eprintln!("Failed to execute query");
            std::process::exit(1);
        }

        let mut results = vec![];

        unsafe {
            let result_count = MDQueryGetResultCount(query);
            // println!("Found {} items", result_count);
            for i in 0..result_count {
                let item = MDQueryGetResultAtIndex(query, i);
                if !item.is_null() {
                    let path: CFTypeRef = MDItemCopyAttribute(item, CFStringRef::from(item_type));
                    if !path.is_null() {
                        let mut buffer = [0u8; 1024];
                        CFStringGetCString(
                            path as CFStringRef,
                            buffer.as_mut_ptr() as *mut i8,
                            buffer.len() as isize,
                            kCFStringEncodingUTF8,
                        );
                        let path_str = std::str::from_utf8(
                            std::ffi::CStr::from_ptr(buffer.as_ptr() as *const _).to_bytes(),
                        )
                        .expect("Failed to convert path to string");

                        results.push(path_str.to_string());

                        CFRelease(path);
                    }
                }
            }
        }
        results
    }

    /// Search for a file in the spotlight database.
    pub fn search(search_keyword: impl Into<String>) -> Vec<String> {
        let query_string = CFString::new(&format!(
            "kMDItemDisplayName = '*{}*'",
            search_keyword.into()
        ));
        let item_type = unsafe { KMDItemTypes::Path.into_ref() };

        let query = unsafe {
            MDQueryCreate(
                kCFAllocatorDefault,
                query_string.as_concrete_TypeRef(),
                ptr::null(),
                ptr::null(),
            )
        };

        if query.is_null() {
            eprintln!("Failed to create query");
            std::process::exit(1);
        }

        if unsafe { MDQueryExecute(query, K_MDQUERY_SYNCHRONOUS) } == 0 {
            eprintln!("Failed to execute query");
            std::process::exit(1);
        }

        let mut results = vec![];

        unsafe {
            let result_count = MDQueryGetResultCount(query);
            // println!("Found {} items", result_count);
            for i in 0..result_count {
                let item = MDQueryGetResultAtIndex(query, i);
                if !item.is_null() {
                    let path: CFTypeRef = MDItemCopyAttribute(item, CFStringRef::from(item_type));
                    if !path.is_null() {
                        let mut buffer = [0u8; 1024];
                        CFStringGetCString(
                            path as CFStringRef,
                            buffer.as_mut_ptr() as *mut i8,
                            buffer.len() as isize,
                            kCFStringEncodingUTF8,
                        );
                        let path_str = std::str::from_utf8(
                            std::ffi::CStr::from_ptr(buffer.as_ptr() as *const _).to_bytes(),
                        )
                        .expect("Failed to convert path to string");

                        results.push(path_str.to_string());

                        CFRelease(path);
                    }
                }
            }
        }
        results
    }
}
