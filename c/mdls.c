#include <CoreFoundation/CoreFoundation.h>
#include <CoreServices/CoreServices.h>
#include <stdio.h>

void printMetadata(CFStringRef filePath) {
    MDItemRef item = MDItemCreate(kCFAllocatorDefault, filePath);
    if (item == NULL) {
        printf("Failed to create MDItem for the file\n");
        return;
    }

    // List of metadata attributes you are interested in
    CFArrayRef attributes = MDItemCopyAttributeNames(item);
    if (attributes == NULL) {
        printf("Failed to retrieve attributes\n");
        CFRelease(item);
        return;
    }

    // Print each attribute
    for (CFIndex i = 0; i < CFArrayGetCount(attributes); i++) {
        CFStringRef attribute = CFArrayGetValueAtIndex(attributes, i);
        CFTypeRef value = MDItemCopyAttribute(item, attribute);
        if (value) {
            CFStringRef valueStr = CFCopyDescription(value);
            CFStringRef attributeStr = CFCopyDescription(attribute);
            if (valueStr && attributeStr) {
                printf("%s: %s\n", CFStringGetCStringPtr(attributeStr, kCFStringEncodingUTF8), CFStringGetCStringPtr(valueStr, kCFStringEncodingUTF8));
            }
            if (valueStr) CFRelease(valueStr);
            if (attributeStr) CFRelease(attributeStr);
            CFRelease(value);
        }
    }

    CFRelease(attributes);
    CFRelease(item);
}

int main(int argc, const char * argv[]) {
    if (argc != 2) {
        printf("Usage: %s <file-path>\n", argv[0]);
        return 1;
    }

    CFStringRef filePath = CFStringCreateWithCString(kCFAllocatorDefault, argv[1], kCFStringEncodingUTF8);
    if (filePath == NULL) {
        printf("Failed to create CFString for the file path\n");
        return 1;
    }

    printMetadata(filePath);
    CFRelease(filePath);

    return 0;
}
