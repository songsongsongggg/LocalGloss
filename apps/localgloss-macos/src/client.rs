//! 仅提交文字和读取光标几何、应用标识；没有剪贴板或正文读取接口。
use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_foundation::{NSDictionary, NSNotFound, NSRange, NSRect, NSString};

const NO_REPLACEMENT: NSRange = NSRange::new(NSNotFound as usize, 0);

#[derive(Clone, Copy)]
pub struct TextClient<'a> {
    object: &'a AnyObject,
}

impl<'a> TextClient<'a> {
    pub fn new(object: &'a AnyObject) -> Self {
        Self { object }
    }

    pub fn insert(&self, text: &str) {
        let string = NSString::from_str(text);
        unsafe {
            let _: () =
                msg_send![self.object, insertText: &*string, replacementRange: NO_REPLACEMENT];
        }
    }

    pub fn mark(&self, text: &str, cursor: usize) {
        let string = NSString::from_str(text);
        let cursor = NSRange::new(cursor.min(text.encode_utf16().count()), 0);
        unsafe {
            let _: () = msg_send![self.object, setMarkedText: &*string,
                selectionRange: cursor, replacementRange: NO_REPLACEMENT];
        }
    }

    pub fn application(&self) -> Option<String> {
        let id: Option<Retained<NSString>> = unsafe { msg_send![self.object, bundleIdentifier] };
        id.map(|id| id.to_string())
    }

    pub fn caret(&self) -> NSRect {
        let mut rect = NSRect::ZERO;
        unsafe {
            let _: Option<Retained<NSDictionary>> = msg_send![self.object,
                attributesForCharacterIndex: 0usize, lineHeightRectangle: &mut rect];
        }
        rect
    }
}
