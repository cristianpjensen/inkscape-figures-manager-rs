use cocoa::base::{id, nil, BOOL, YES};
use cocoa::foundation::{NSAutoreleasePool, NSInteger, NSString, NSUInteger};
use objc::{class, msg_send, sel, sel_impl};
use std::ffi::c_void;

/// Source: [The Clipboard implementation of the Druid
/// framework](https://github.com/linebender/druid/blob/e53a5ab72c40191b3f92edef9ebf4da07da254f3/druid-shell/src/backend/mac/clipboard.rs#L49)
pub fn copy_mime(mime: &str, data: &str) {
    unsafe {
        let pasteboard: id = msg_send![class!(NSPasteboard), generalPasteboard];
        let _: NSInteger = msg_send![pasteboard, clearContents];

        let data_type = make_nsstring(mime);
        let data = make_nsdata(data.as_bytes());
        let result: BOOL = msg_send![pasteboard, setData: data forType: data_type];

        if result != YES {
            println!("warning: failed to set clipboard contents");
        }
    }
}

fn make_nsstring(s: &str) -> id {
    unsafe { NSString::alloc(nil).init_str(s).autorelease() }
}

fn make_nsdata(bytes: &[u8]) -> id {
    let dlen = bytes.len() as NSUInteger;
    unsafe {
        msg_send![class!(NSData), dataWithBytes: bytes.as_ptr() as *const c_void length: dlen]
    }
}
