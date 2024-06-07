#include <CoreFoundation/CoreFoundation.h>
#include <CoreServices/CoreServices.h>
#include <stdio.h>

void printCFTypeRef(CFTypeRef value) {
    if (CFGetTypeID(value) == CFStringGetTypeID()) {
        printf("%s", CFStringGetCStringPtr((CFStringRef)value, kCFStringEncodingUTF8));
    } else if (CFGetTypeID(value) == CFNumberGetTypeID()) {
        int64_t number;
        if (CFNumberGetValue((CFNumberRef)value, kCFNumberSInt64Type, &number)) {
            printf("%lld", number);
        } else {
            printf("(unknown number)");
        }
    } else if (CFGetTypeID(value) == CFDateGetTypeID()) {
        CFDateRef date = (CFDateRef)value;
        CFAbsoluteTime absTime = CFDateGetAbsoluteTime(date);
        CFTimeZoneRef timeZone = CFTimeZoneCopySystem();
        CFGregorianDate gregDate = CFAbsoluteTimeGetGregorianDate(absTime, timeZone);
        printf("%04d-%02d-%02d %02d:%02d:%02d +0000",
               gregDate.year, gregDate.month, gregDate.day,
               gregDate.hour, gregDate.minute, gregDate.second);
        CFRelease(timeZone);
    } else if (CFGetTypeID(value) == CFArrayGetTypeID()) {
        CFArrayRef array = (CFArrayRef)value;
        printf("(");
        for (CFIndex i = 0; i < CFArrayGetCount(array); i++) {
            if (i > 0) {
                printf(", ");
            }
            printCFTypeRef(CFArrayGetValueAtIndex(array, i));
        }
        printf(")");
    } else if (CFGetTypeID(value) == CFBooleanGetTypeID()) {
        if (CFBooleanGetValue((CFBooleanRef)value)) {
            printf("true");
        } else {
            printf("false");
        }
    } else {
        CFStringRef valueStr = CFCopyDescription(value);
        if (valueStr) {
            printf("%s", CFStringGetCStringPtr(valueStr, kCFStringEncodingUTF8));
            CFRelease(valueStr);
        } else {
            printf("(unknown type)");
        }
    }
}

void querySpecificAttribute(CFStringRef filePath, CFStringRef attributeName) {
    MDItemRef item = MDItemCreate(kCFAllocatorDefault, filePath);
    if (item == NULL) {
        printf("Failed to create MDItem for the file\n");
        return;
    }

    CFTypeRef value = MDItemCopyAttribute(item, attributeName);
    if (value) {
        CFStringRef attributeStr = CFCopyDescription(attributeName);
        if (attributeStr) {
            printf("%s: ", CFStringGetCStringPtr(attributeStr, kCFStringEncodingUTF8));
            printCFTypeRef(value);
            printf("\n");
            CFRelease(attributeStr);
        }
        CFRelease(value);
    } else {
        printf("Attribute not found or empty\n");
    }

    CFRelease(item);
}

int main(int argc, const char * argv[]) {
    if (argc != 3) {
        printf("Usage: %s <file-path> <attribute-name>\n", argv[0]);
        return 1;
    }

    CFStringRef filePath = CFStringCreateWithCString(kCFAllocatorDefault, argv[1], kCFStringEncodingUTF8);
    CFStringRef attributeName = CFStringCreateWithCString(kCFAllocatorDefault, argv[2], kCFStringEncodingUTF8);
    if (filePath == NULL || attributeName == NULL) {
        printf("Failed to create CFString for the file path or attribute name\n");
        if (filePath) CFRelease(filePath);
        if (attributeName) CFRelease(attributeName);
        return 1;
    }

    querySpecificAttribute(filePath, attributeName);
    CFRelease(filePath);
    CFRelease(attributeName);

    return 0;
}
