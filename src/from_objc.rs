use std::ffi::c_void;

use core_foundation::{
    array::{CFArray, CFArrayGetTypeID, CFArrayRef},
    base::{CFGetTypeID, TCFType},
    boolean::CFBoolean,
    number::{CFBooleanGetValue, CFBooleanRef, CFNumber, CFNumberRef},
    string::{CFString, CFStringRef},
};

/// Gets the CFTypeID of a raw pointer and compares it to expected Type ID of the type being casted to.
macro_rules! check_type {
    ($obj:expr, $type:ident) => {
        unsafe { CFGetTypeID($obj) == $type::type_id() }
    };
}

pub trait FromObjC {
    fn from_objc(obj: *const c_void) -> Option<Self>
    where
        Self: Sized;
}

impl<T> FromObjC for Vec<T>
where
    T: FromObjC,
{
    fn from_objc(obj: *const c_void) -> Option<Self> {
        if unsafe { CFGetTypeID(obj) } != unsafe { CFArrayGetTypeID() } {
            return None;
        }
        let mut v = Vec::new();
        unsafe {
            let array: CFArray<*const c_void> = CFArray::wrap_under_get_rule(obj as CFArrayRef);
            for element in array.iter() {
                if let Some(value) = T::from_objc(*element) {
                    v.push(value);
                } else {
                    // an element could not be converted
                    // return None rather than returning a partial Vec
                    eprintln!("[-] Failed to convert objc element, returning None");
                    return None;
                }
            }
        }
        Some(v)
    }
}

impl FromObjC for String {
    fn from_objc(obj: *const c_void) -> Option<Self> {
        if !check_type!(obj, CFString) {
            return None;
        }
        let cfstring = unsafe { CFString::wrap_under_get_rule(obj as CFStringRef) };
        Some(cfstring.to_string())
    }
}

impl FromObjC for i64 {
    fn from_objc(obj: *const c_void) -> Option<Self> {
        if !check_type!(obj, CFNumber) {
            return None;
        }
        Some(
            unsafe { CFNumber::wrap_under_get_rule(obj as CFNumberRef) }
                .to_i64()
                .expect("Failed to cast CFNumber, expected i64"),
        )
    }
}

impl FromObjC for i32 {
    fn from_objc(obj: *const c_void) -> Option<Self> {
        if !check_type!(obj, CFNumber) {
            return None;
        }
        Some(
            unsafe { CFNumber::wrap_under_get_rule(obj as CFNumberRef) }
                .to_i32()
                .expect("Failed to cast CFNumber, expected i32"),
        )
    }
}

impl FromObjC for f32 {
    fn from_objc(obj: *const c_void) -> Option<Self> {
        if !check_type!(obj, CFNumber) {
            return None;
        }
        Some(
            unsafe { CFNumber::wrap_under_get_rule(obj as CFNumberRef) }
                .to_f32()
                .expect("Failed to cast CFNumber, expected f32"),
        )
    }
}

impl FromObjC for f64 {
    fn from_objc(obj: *const c_void) -> Option<Self> {
        if !check_type!(obj, CFNumber) {
            return None;
        }
        Some(
            unsafe { CFNumber::wrap_under_get_rule(obj as CFNumberRef) }
                .to_f64()
                .expect("Failed to cast CFNumber, expected f64"),
        )
    }
}

impl FromObjC for bool {
    fn from_objc(obj: *const c_void) -> Option<Self> {
        if !check_type!(obj, CFBoolean) {
            return None;
        }
        Some(unsafe {
            CFBooleanGetValue(
                CFBoolean::wrap_under_get_rule(obj as CFBooleanRef).as_concrete_TypeRef(),
            )
        })
    }
}
