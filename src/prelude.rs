pub use core_foundation::array::{CFArray, CFArrayGetTypeID, CFArrayRef};
pub use core_foundation::base::{CFGetTypeID, CFRelease, CFType, CFTypeRef, TCFType};
pub use core_foundation::string::CFString;
pub use core_foundation::string::CFStringRef;
pub use core_foundation_sys::base::{kCFAllocatorDefault, CFAllocatorRef, CFOptionFlags};

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    pub fn MDQueryCreate(
        allocator: CFAllocatorRef,
        queryString: CFStringRef,
        valueListAttrs: CFArrayRef,
        sortingAttrs: CFArrayRef,
    ) -> MDQueryRef;

    pub fn MDItemCreate(allocator: CFAllocatorRef, path: CFStringRef) -> MDItemRef;

    pub fn MDQueryExecute(query: MDQueryRef, option: CFOptionFlags) -> Boolean;
    pub fn MDQueryGetResultCount(query: MDQueryRef) -> CFIndex;
    pub fn MDQueryGetResultAtIndex(query: MDQueryRef, idx: CFIndex) -> MDItemRef;

    pub fn MDItemCopyAttribute(item: MDItemRef, name: CFStringRef) -> CFTypeRef;

    pub fn MDItemCopyAttributeNames(item: MDItemRef) -> CFArrayRef;

    pub static kMDItemUserTags: CFStringRef;
    pub static kMDItemPath: CFStringRef;
}

pub type MDQueryRef = *const __MDQuery;
pub type CFIndex = isize;
pub type Boolean = u8;
pub type MDItemRef = *const __MDItem;
pub const K_MDQUERY_ASYNCHRONOUS: CFOptionFlags = 0;
pub const K_MDQUERY_SYNCHRONOUS: CFOptionFlags = 1;

pub enum KMDItemTypes {
    UserTags,
    Path,
}

pub trait KMDItem {
    fn into_ref(&self) -> CFStringRef;
}

impl KMDItemTypes {
    pub unsafe fn into_ref(&self) -> CFStringRef {
        match &self {
            KMDItemTypes::UserTags => kMDItemUserTags,
            KMDItemTypes::Path => kMDItemPath,
        }
    }
}

#[repr(C)]
pub struct __MDQuery(u8);
#[repr(C)]
pub struct __MDItem(u8);
