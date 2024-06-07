use core_foundation::array::{CFArray, CFArrayGetTypeID, CFArrayRef};
use core_foundation::base::{CFGetTypeID, CFRelease, CFType, CFTypeRef, TCFType};
use core_foundation::string::{kCFStringEncodingUTF8, CFString, CFStringGetCString, CFStringRef};
use core_foundation::url::{CFURLCopyFileSystemPath, CFURLRef};
use core_foundation_sys::base::{kCFAllocatorDefault, CFAllocatorRef, CFOptionFlags};
use core_foundation_utils::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::os::fd::{AsFd, AsRawFd};
use std::os::unix::fs::FileExt;
use std::ptr;

struct SpotlightApi;

impl SpotlightApi {
    fn search(query_string: impl Into<String>, item_type: KMDItemTypes) -> Vec<String> {
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
}

use core_foundation_sys::url::CFURLCreateFromFileSystemRepresentation;

fn get_file_path(file: &File) -> std::io::Result<String> {
    let fd = file.as_raw_fd();

    unsafe {
        let file_url: CFURLRef = CFURLCreateFromFileSystemRepresentation(
            kCFAllocatorDefault,
            fd.to_string().as_bytes_mut().as_mut_ptr(),
            fd.to_string().len() as isize,
            false.into(),
        );

        if file_url.is_null() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to create URL from file descriptor",
            ));
        }

        let path: CFStringRef = CFURLCopyFileSystemPath(file_url, 0);
        CFRelease(file_url as CFTypeRef);

        if path.is_null() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to copy file system path",
            ));
        }

        let mut buffer: [u8; 1024] = [0; 1024];
        let success = CFStringGetCString(
            path,
            buffer.as_mut_ptr() as *mut i8,
            buffer.len() as isize,
            0,
        );
        CFRelease(path as CFTypeRef);

        if success == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get CString from CFString",
            ));
        }

        let c_str = std::ffi::CStr::from_ptr(buffer.as_ptr() as *const i8);
        Ok(c_str.to_string_lossy().into_owned())
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Usage: mdfind <filename>");
        std::process::exit(1);
    }

    let file_name = &args[1];
    let file = std::fs::File::open(file_name).expect("Failed to open file");
    let path = get_file_path(&file).expect("Failed to get file path");
    // let query_string = CFString::new(format!("kMDItemDisplayName = '*{file_name}*'").as_str());

    for item in SpotlightApi::search(
        format!("kMDItemDisplayName = '*{file_name}*'"),
        KMDItemTypes::Path,
    ) {
        println!("[*] {}", item);
    }
}
