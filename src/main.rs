use core_foundation::array::{CFArray, CFArrayGetTypeID, CFArrayRef};
use core_foundation::base::{CFGetTypeID, CFRelease, CFType, CFTypeRef, TCFType};
use core_foundation::string::{CFString, CFStringRef};
use core_foundation_sys::base::{kCFAllocatorDefault, CFAllocatorRef, CFOptionFlags};
use std::collections::HashSet;
use std::ptr;

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    fn MDQueryCreate(
        allocator: CFAllocatorRef,
        queryString: CFStringRef,
        valueListAttrs: CFArrayRef,
        sortingAttrs: CFArrayRef,
    ) -> MDQueryRef;

    fn MDQueryExecute(query: MDQueryRef, option: CFOptionFlags) -> Boolean;
    fn MDQueryGetResultCount(query: MDQueryRef) -> CFIndex;
    fn MDQueryGetResultAtIndex(query: MDQueryRef, idx: CFIndex) -> MDItemRef;

    fn MDItemCopyAttribute(item: MDItemRef, name: CFStringRef) -> CFTypeRef;

    static kMDItemUserTags: CFStringRef;
}

type MDQueryRef = *const __MDQuery;
type CFIndex = isize;
type Boolean = u8;
type MDItemRef = *const __MDItem;

#[repr(C)]
struct __MDQuery;
#[repr(C)]
struct __MDItem;

fn print_tags_from_mditem(item: MDItemRef) {
    unsafe {
        let tags: CFTypeRef = MDItemCopyAttribute(item, kMDItemUserTags);
        if !tags.is_null() {
            if CFGetTypeID(tags) == CFArrayGetTypeID() {
                let tags_array: CFArray<CFString> =
                    TCFType::wrap_under_create_rule(tags as CFArrayRef);
                for j in 0..tags_array.len() {
                    let tag = tags_array.get(j).unwrap();
                    println!("{}", tag.to_string());
                }
            }
            CFRelease(tags);
        }
    }
}

fn main() {
    unsafe {
        let query_string = CFString::new("kMDItemUserTags == '*'");

        let query = MDQueryCreate(
            kCFAllocatorDefault,
            query_string.as_concrete_TypeRef(),
            ptr::null(),
            ptr::null(),
        );
        if query.is_null() {
            eprintln!("Failed to create query.");
            return;
        }

        let query_started = MDQueryExecute(query, 1); // 1 for synchronous execution
        if query_started == 0 {
            eprintln!("Failed to start query.");
            return;
        }

        let result_count = MDQueryGetResultCount(query);
        println!("Total results: {}", result_count);

        let mut tag_set = HashSet::new();

        for i in 0..result_count {
            let item = MDQueryGetResultAtIndex(query, i);
            if !item.is_null() {
                let tags: CFTypeRef = MDItemCopyAttribute(item, kMDItemUserTags);
                if !tags.is_null() {
                    if CFGetTypeID(tags) == CFArrayGetTypeID() {
                        let tags_array: CFArray<CFString> =
                            TCFType::wrap_under_create_rule(tags as CFArrayRef);
                        for j in 0..tags_array.len() {
                            let tag = tags_array.get(j).unwrap();
                            tag_set.insert(tag.to_string());
                        }
                    }
                    CFRelease(tags);
                }
            }
        }

        for tag in tag_set.iter() {
            println!("{}", tag);
        }
        println!("Total tags: {}", tag_set.len());

        CFRelease(query as CFTypeRef);
    }
}
