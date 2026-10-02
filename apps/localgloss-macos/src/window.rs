//! 不抢焦点的候选面板；安全边界隐藏并清除显示内容。
use localgloss_engine::Frame;
use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{
    NSBackingStoreType, NSColor, NSEvent, NSPanel, NSScreen, NSWindowCollectionBehavior,
    NSWindowStyleMask,
};
use objc2_foundation::{NSPoint, NSRect, NSSize};

use crate::view::CandidateView;

pub struct CandidateWindow {
    panel: Retained<NSPanel>,

    view: Retained<CandidateView>,

    mtm: MainThreadMarker,
}

impl CandidateWindow {
    pub fn configure(&self, settings: localgloss_engine::Settings) {
        self.view.configure(settings);
    }
    pub fn new(mtm: MainThreadMarker) -> Self {
        let view = CandidateView::new(mtm);
        let panel = build_panel(mtm, &view);
        Self { panel, view, mtm }
    }

    pub fn show(&mut self, frame: Frame, mut anchor: NSRect) {
        if frame.preedit.is_empty() {
            self.clear();
            return;
        }
        if anchor == NSRect::ZERO {
            anchor = NSRect::new(NSEvent::mouseLocation(), NSSize::new(0.0, 16.0));
        }
        let screens = NSScreen::screens(self.mtm);
        let screen = screens
            .iter()
            .find(|screen| contains(screen.frame(), anchor.origin))
            .map(|screen| screen.visibleFrame())
            .or_else(|| NSScreen::mainScreen(self.mtm).map(|screen| screen.visibleFrame()));
        let size = self
            .view
            .set_frame(frame, screen.map_or(560.0, |screen| screen.size.width));
        let origin = if let Some(screen) = screen {
            let max_x = (screen.origin.x + screen.size.width - size.width).max(screen.origin.x);
            let max_y = (screen.origin.y + screen.size.height - size.height).max(screen.origin.y);
            let below = anchor.origin.y - size.height - 4.0;
            let y = if below < screen.origin.y {
                anchor.origin.y + anchor.size.height + 4.0
            } else {
                below
            };
            NSPoint::new(
                anchor.origin.x.clamp(screen.origin.x, max_x),
                y.clamp(screen.origin.y, max_y),
            )
        } else {
            anchor.origin
        };
        self.panel.setFrame_display(NSRect::new(origin, size), true);
        self.panel.orderFrontRegardless();
        if !self.panel.isOnActiveSpace() {
            self.panel.orderOut(None);
            self.panel = build_panel(self.mtm, &self.view);
            self.panel.setFrame_display(NSRect::new(origin, size), true);
            self.panel.orderFrontRegardless();
        }
    }

    pub fn clear(&self) {
        self.panel.orderOut(None);
        self.view.clear();
    }
}

fn contains(rect: NSRect, point: NSPoint) -> bool {
    point.x >= rect.origin.x
        && point.x < rect.origin.x + rect.size.width
        && point.y >= rect.origin.y
        && point.y < rect.origin.y + rect.size.height
}

fn build_panel(mtm: MainThreadMarker, view: &CandidateView) -> Retained<NSPanel> {
    let panel = NSPanel::initWithContentRect_styleMask_backing_defer(
        mtm.alloc::<NSPanel>(),
        NSRect::new(NSPoint::ZERO, NSSize::new(300.0, 100.0)),
        NSWindowStyleMask::Borderless | NSWindowStyleMask::NonactivatingPanel,
        NSBackingStoreType::Buffered,
        false,
    );
    panel.setOpaque(false);
    panel.setBackgroundColor(Some(&NSColor::clearColor()));
    panel.setHasShadow(true);
    panel.setBecomesKeyOnlyIfNeeded(true);
    panel.setIgnoresMouseEvents(true);
    panel.setHidesOnDeactivate(false);
    panel.setCollectionBehavior(
        NSWindowCollectionBehavior::CanJoinAllSpaces
            | NSWindowCollectionBehavior::FullScreenAuxiliary
            | NSWindowCollectionBehavior::Stationary,
    );
    panel.setLevel(101);
    panel.setContentView(Some(view));
    panel
}
