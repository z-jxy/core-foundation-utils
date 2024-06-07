use std::{collections::HashMap, ffi::c_void, str::FromStr, time::SystemTime};

use chrono::{DateTime, Local};
use core_foundation::{
    array::{CFArrayGetCount, CFArrayGetValueAtIndex},
    base::CFCopyDescription,
    boolean::CFBoolean,
    number::{
        CFBooleanGetTypeID, CFBooleanGetValue, CFBooleanRef, CFNumber, CFNumberGetTypeID,
        CFNumberRef,
    },
    string::CFStringGetTypeID,
};
use core_foundation_utils::{from_objc::FromObjC, mds::query_file_attr, prelude::*};
use plist::Dictionary;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MetadataAttributes {
    /// The absolute path of the file.
    psid_path: String,

    /// The content type of the file.
    content_type: String,
    creation_date: plist::Date,
}

// impl From<kMDItemSDBInfo> for Option<MetadataAttributes> {
//     fn from(plist: kMDItemSDBInfo) -> Self {
//         let psid_path = plist._kMDItemSDBInfo.get("_kMDItemPSIDPath")?;
//         let content_type = plist._kMDItemSDBInfo.get("_kMDItemContentType")?;
//         // let creation_date = plist._kMDItemSDBInfo.get("_kMDItemCreationDate")?;
//         Some(MetadataAttributes {
//             psid_path: psid_path.as_string()?.to_string(),
//             content_type: content_type.as_string()?.to_string(),
//             // creation_date: creation_date.as_date()?,
//         })
//     }
// }

impl MetadataAttributes {
    fn parse_from_plist(plist: kMDItemSDBInfo) -> Self {
        let psid_path = plist._kMDItemSDBInfo.get("_kMDItemPSIDPath").unwrap();
        let content_type = plist._kMDItemSDBInfo.get("kMDItemContentType").unwrap();
        let creation_date = plist._kMDItemSDBInfo.get("_kMDItemCreationDate").unwrap();
        MetadataAttributes {
            psid_path: psid_path
                .as_string()
                .expect("value isn't a string")
                .to_string(),
            content_type: content_type
                .as_string()
                .expect("value isn't a string")
                .to_string(),
            creation_date: creation_date.as_date().expect("creation_date isn't a date"),
        }
    }
}

#[derive(Deserialize, Debug)]
struct kMDItemSDBInfo {
    _kMDItemSDBInfo: Dictionary,
}

fn query_md(file_name: &str) {
    let file_cfstring = CFString::new(file_name);
    let item = unsafe { MDItemCreate(kCFAllocatorDefault, file_cfstring.as_concrete_TypeRef()) };
    if item.is_null() {
        eprintln!("Failed to create MDItem");
        std::process::exit(1);
    }
    println!("metadata opened");
    let attrs = unsafe { MDItemCopyAttributeNames(item) };
    if attrs.is_null() {
        eprintln!("Failed to get attribute names");
        std::process::exit(1);
    }
    println!("attrs copied");
    let count = unsafe { CFArrayGetCount(attrs) };
    for i in 0..count {
        let attr = unsafe { CFArrayGetValueAtIndex(attrs, i) };
        let attr_str = unsafe { CFString::wrap_under_get_rule(attr as CFStringRef) };
        let attr_name = attr_str.to_string();
        // println!("attr_name: {}", attr_name);
        let value = unsafe { MDItemCopyAttribute(item, attr_str.as_concrete_TypeRef()) };
        // println!("value: {:?}", value);
        if !value.is_null() {
            let v = unsafe { CFCopyDescription(value) };
            let value_str = unsafe { CFString::wrap_under_get_rule(v as CFStringRef).to_string() };
            let type_id = unsafe { CFGetTypeID(value) };
            if type_id == unsafe { CFNumberGetTypeID() } {
                let number = unsafe { CFNumber::wrap_under_get_rule(value as CFNumberRef) };
                // let number_type = number.number_type();
                let number_value = number
                    .to_i64()
                    .expect("Failed to cast CFNumber, expected i64");
                println!("{}: ({:?})", attr_name, number_value);
            } else if type_id == unsafe { CFStringGetTypeID() } {
                let strr = unsafe { CFString::wrap_under_get_rule(value as CFStringRef) };
                println!("{}: {}", attr_name, strr);
            } else if type_id == unsafe { CFBooleanGetTypeID() } {
                let boolean = unsafe { CFBoolean::wrap_under_get_rule(value as CFBooleanRef) };
                let boolean_val = unsafe { CFBooleanGetValue(boolean.as_concrete_TypeRef()) };
                println!("{}: {:?}", attr_name, boolean_val);
            } else if type_id == unsafe { CFArrayGetTypeID() } {
                let array: CFArray<CFString> =
                    unsafe { CFArray::wrap_under_get_rule(value as CFArrayRef) };
                let array_count = array.len();
                println!("{}: [", attr_name);
                for j in 0..array_count {
                    let element = array.get(j).unwrap();
                    let element_str = element.to_string();
                    println!("  {}", element_str);
                }
                println!("]");
                // println!("[unhandled]{}: {}", attr_name, value_str);
            } else {
                println!("{}: {}", attr_name, value_str);
            }

            unsafe { CFRelease(value) };
        }
    }
}

// fn query_file_attr<T>(file_name: &str, attr_name: &str) -> T
// where
//     T: FromObjC,
// {
//     let file_cfstring = CFString::new(file_name);
//     let item = unsafe { MDItemCreate(kCFAllocatorDefault, file_cfstring.as_concrete_TypeRef()) };
//     if item.is_null() {
//         eprintln!("Failed to create MDItem");
//         std::process::exit(1);
//     }
//     println!("metadata opened");
//     let attrs = unsafe { MDItemCopyAttributeNames(item) };
//     if attrs.is_null() {
//         eprintln!("Failed to get attribute names");
//         std::process::exit(1);
//     }
//     println!("attrs copied");
//     let value = unsafe {
//         MDItemCopyAttribute(
//             item,
//             CFString::from_str(attr_name).unwrap().as_concrete_TypeRef(),
//         )
//     };

//     if value.is_null() {
//         eprintln!("Failed to get attribute value");
//         std::process::exit(1);
//     }

//     T::from_objc(value)

//     /*
//     let v = unsafe { CFCopyDescription(value) };

//     let count = unsafe { CFArrayGetCount(attrs) };
//     for i in 0..count {
//         let attr = unsafe { CFArrayGetValueAtIndex(attrs, i) };
//         let attr_str = unsafe { CFString::wrap_under_get_rule(attr as CFStringRef) };
//         let attr_name = attr_str.to_string();
//         // println!("attr_name: {}", attr_name);
//         let value = unsafe { MDItemCopyAttribute(item, attr_str.as_concrete_TypeRef()) };
//         // println!("value: {:?}", value);
//         if !value.is_null() {
//             let v = unsafe { CFCopyDescription(value) };
//             let value_str = unsafe { CFString::wrap_under_get_rule(v as CFStringRef).to_string() };
//             let type_id = unsafe { CFGetTypeID(value) };
//             if type_id == unsafe { CFNumberGetTypeID() } {
//                 let number = unsafe { CFNumber::wrap_under_get_rule(value as CFNumberRef) };
//                 // let number_type = number.number_type();
//                 let number_value = number
//                     .to_i64()
//                     .expect("Failed to cast CFNumber, expected i64");
//                 println!("{}: ({:?})", attr_name, number_value);
//             } else if type_id == unsafe { CFStringGetTypeID() } {
//                 let strr = unsafe { CFString::wrap_under_get_rule(value as CFStringRef) };
//                 println!("{}: {}", attr_name, strr);
//             } else if type_id == unsafe { CFBooleanGetTypeID() } {
//                 let boolean = unsafe { CFBoolean::wrap_under_get_rule(value as CFBooleanRef) };
//                 let boolean_val = unsafe { CFBooleanGetValue(boolean.as_concrete_TypeRef()) };
//                 println!("{}: {:?}", attr_name, boolean_val);
//             } else if type_id == unsafe { CFArrayGetTypeID() } {
//                 let array: CFArray<CFString> =
//                     unsafe { CFArray::wrap_under_get_rule(value as CFArrayRef) };
//                 let array_count = array.len();
//                 println!("{}: [", attr_name);
//                 for j in 0..array_count {
//                     let element = array.get(j).unwrap();
//                     let element_str = element.to_string();
//                     println!("  {}", element_str);
//                 }
//                 println!("]");
//                 // println!("[unhandled]{}: {}", attr_name, value_str);
//             } else {
//                 println!("{}: {}", attr_name, value_str);
//             }

//             unsafe { CFRelease(value) };
//         }
//     }
//      */
// }

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Usage: mdls <filename>");
        std::process::exit(1);
    }
    let file_name = &args[1];
    // kMDItemContentType
    // kMDItemContentTypeTree
    let data: Vec<String> = query_file_attr(file_name, "kMDItemContentTypeTree").unwrap();
    println!("kMDItemContentType: {:?}", data);

    // let plist: kMDItemSDBInfo = plist::from_file(file_name).expect("Failed to parse plist");
    // let attrs = MetadataAttributes::parse_from_plist(plist);
    // println!("attrs: {:?}", attrs);
}

fn system_time_to_date_time(system_time: SystemTime) -> String {
    let datetime: DateTime<Local> = system_time.into();
    datetime.format("%Y-%m-%d %I:%M:%S %p").to_string()
}
