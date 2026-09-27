//! 原生 AppKit 自绘当前候选；不保存已提交的帧。
use std::cell::RefCell;

use localgloss_engine::{Frame, Settings};
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send};
use objc2_app_kit::{
    NSAttributedStringNSStringDrawing, NSBezierPath, NSColor, NSFont, NSFontAttributeName,
    NSForegroundColorAttributeName, NSView,
};
use objc2_foundation::{NSAttributedString, NSDictionary, NSPoint, NSRect, NSSize, NSString};

define_class!(
    // SAFETY: NSView 自绘方法在主线程调用，状态只在主线程访问。
    #[unsafe(super(NSView))]
    #[thread_kind = MainThreadOnly]
    #[ivars = RefCell<(Frame, Settings)>]
    pub struct CandidateView;

    impl CandidateView {

        #[unsafe(method(isFlipped))]
        fn is_flipped(&self) -> bool { true }

        #[unsafe(method(drawRect:))]
        fn draw_rect(&self, _dirty: NSRect) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.draw()));
        }
    }
);

impl CandidateView {
    pub fn configure(&self, settings: Settings) {
        self.ivars().borrow_mut().1 = settings;
    }

    pub fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = mtm
            .alloc::<Self>()
            .set_ivars(RefCell::new((Frame::default(), Settings::default())));
        unsafe { msg_send![super(this), initWithFrame: NSRect::ZERO] }
    }

    pub fn set_frame(&self, frame: Frame) -> NSSize {
        let font = NSFont::systemFontOfSize(self.ivars().borrow().1.font_size as f64);
        let gloss_font = NSFont::systemFontOfSize(14.0);
        let foreground = NSColor::labelColor();
        let column = text_column(&frame, &font, &foreground);
        let desired_width = frame
            .rows
            .iter()
            .map(|row| {
                column
                    + attributed(&row.gloss, &gloss_font, &foreground)
                        .size()
                        .width
                    + 18.0
            })
            .fold(420.0, f64::max)
            .min(620.0);
        // 同一次组合只扩宽、不缩窄，避免逐键查询时窗口左右跳动。
        let width = desired_width.max(self.frame().size.width);
        let row_height = self.ivars().borrow().1.font_size as f64 + 16.0;
        let height = if let Some(detail) = &frame.detail {
            75.0 + wrapped(detail, &font, &foreground, width - 28.0).len() as f64 * row_height
        } else {
            70.0 + row_height * frame.rows.len() as f64
        };
        self.ivars().borrow_mut().0 = frame;
        self.setFrameSize(NSSize::new(width, height));
        self.setNeedsDisplay(true);
        NSSize::new(width, height)
    }

    pub fn clear(&self) {
        self.ivars().borrow_mut().0 = Frame::default();
        self.setFrameSize(NSSize::ZERO);
        self.setNeedsDisplay(true);
    }

    fn draw(&self) {
        let state = self.ivars().borrow();
        let (frame, settings) = &*state;
        NSColor::windowBackgroundColor().setFill();
        NSBezierPath::bezierPathWithRoundedRect_xRadius_yRadius(self.bounds(), 9.0, 9.0).fill();
        let normal = NSColor::labelColor();
        let secondary = NSColor::secondaryLabelColor();
        let font = NSFont::systemFontOfSize(settings.font_size as f64);
        let small = NSFont::systemFontOfSize(12.0);
        let gloss_font = NSFont::systemFontOfSize(14.0);
        let width = self.bounds().size.width;
        let page = format!("{}/{} · 本地", frame.page + 1, frame.pages.max(1));
        let page = attributed(&page, &small, &secondary);
        let page_x = width - page.size().width - 12.0;
        clipped(&frame.preedit, &small, &secondary, page_x - 28.0)
            .drawAtPoint(NSPoint::new(12.0, 10.0));
        page.drawAtPoint(NSPoint::new(page_x, 10.0));
        if let Some(detail) = &frame.detail {
            for (index, line) in wrapped(detail, &font, &normal, width - 28.0)
                .iter()
                .enumerate()
            {
                attributed(line, &font, &normal).drawAtPoint(NSPoint::new(
                    14.0,
                    37.0 + index as f64 * (settings.font_size as f64 + 16.0),
                ));
            }
            attributed(
                "F1 / Esc 返回 · Page Up / Down 详情翻页",
                &small,
                &secondary,
            )
            .drawAtPoint(NSPoint::new(12.0, self.bounds().size.height - 23.0));
            return;
        }
        let column = text_column(frame, &font, &normal);
        for (index, row) in frame.rows.iter().enumerate() {
            let y = 38.0 + index as f64 * (settings.font_size as f64 + 16.0);
            let selected = index == frame.highlighted;
            if selected {
                NSColor::selectedContentBackgroundColor().setFill();
                NSBezierPath::bezierPathWithRoundedRect_xRadius_yRadius(
                    NSRect::new(
                        NSPoint::new(6.0, y - 4.0),
                        NSSize::new(width - 12.0, settings.font_size as f64 + 15.0),
                    ),
                    5.0,
                    5.0,
                )
                .fill();
            }
            let color = if selected {
                NSColor::selectedMenuItemTextColor()
            } else {
                normal.clone()
            };
            let gloss_color = if selected {
                color.clone()
            } else {
                secondary.clone()
            };
            attributed(&(index + 1).to_string(), &small, &color)
                .drawAtPoint(NSPoint::new(14.0, y + 2.0));
            clipped(&row.text, &font, &color, column - 48.0).drawAtPoint(NSPoint::new(36.0, y));
            let gloss = if row.gloss.is_empty() {
                "暂无译词"
            } else {
                &row.gloss
            };
            clipped(gloss, &gloss_font, &gloss_color, width - column - 14.0)
                .drawAtPoint(NSPoint::new(column, y + 1.0));
        }
        let hint = if frame.rows.is_empty() {
            "暂无候选 · Enter 输入拼音 · Esc 取消"
        } else if frame
            .rows
            .get(frame.highlighted)
            .is_some_and(|row| row.gloss.is_empty())
        {
            "当前词暂无译词 · Space 中文 · Page Up / Down 翻页"
        } else if settings.tab_translation {
            "Space 中文 · Tab 英文 · F1 释义"
        } else {
            "Space 中文 · ⌥数字 英文 · F1 释义"
        };
        clipped(hint, &small, &secondary, width - 24.0)
            .drawAtPoint(NSPoint::new(12.0, self.bounds().size.height - 23.0));
    }
}

fn text_column(frame: &Frame, font: &NSFont, color: &NSColor) -> f64 {
    frame
        .rows
        .iter()
        .map(|row| attributed(&row.text, font, color).size().width)
        .fold(48.0, f64::max)
        .min(180.0)
        + 52.0
}

fn clipped(text: &str, font: &NSFont, color: &NSColor, width: f64) -> Retained<NSAttributedString> {
    let original = attributed(text, font, color);
    if original.size().width <= width {
        return original;
    }
    let boundaries: Vec<usize> = text.char_indices().map(|(index, _)| index).collect();
    let mut low = 0;
    let mut high = boundaries.len();
    while low < high {
        let middle = low + (high - low) / 2;
        let candidate = format!("{}…", &text[..boundaries[middle]]);
        if attributed(&candidate, font, color).size().width <= width {
            low = middle + 1;
        } else {
            high = middle;
        }
    }
    let end = low.checked_sub(1).map_or(0, |index| boundaries[index]);
    attributed(&format!("{}…", &text[..end]), font, color)
}

fn attributed(text: &str, font: &NSFont, color: &NSColor) -> Retained<NSAttributedString> {
    let keys: [&NSString; 2] = unsafe { [NSFontAttributeName, NSForegroundColorAttributeName] };
    let values: [&AnyObject; 2] = [font, color];
    let attributes = NSDictionary::from_slices(&keys, &values);
    unsafe { NSAttributedString::new_with_attributes(&NSString::from_str(text), &attributes) }
}

fn wrapped(text: &str, font: &NSFont, color: &NSColor, width: f64) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for character in text.chars() {
        if character == '\n' {
            lines.push(std::mem::take(&mut line));
            continue;
        }
        let mut next = line.clone();
        next.push(character);
        if !line.is_empty() && attributed(&next, font, color).size().width > width {
            lines.push(std::mem::take(&mut line));
        }
        line.push(character);
    }
    lines.push(line);
    lines
}
