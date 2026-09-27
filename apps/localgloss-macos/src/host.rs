//! 主线程持有离线引擎、当前控制器身份和候选窗；不保留客户端对象。
use std::cell::RefCell;

use localgloss_engine::OfflineEngine;
use objc2::MainThreadMarker;

use crate::security;
use crate::window::CandidateWindow;

pub struct Host {
    pub engine: OfflineEngine,

    pub window: CandidateWindow,

    pub owner: Option<usize>,
}

thread_local! {
    static HOST: RefCell<Option<Host>> = const { RefCell::new(None) };
}

pub fn init(mtm: MainThreadMarker, engine: OfflineEngine) {
    HOST.with(|host| {
        *host.borrow_mut() = Some(Host {
            engine,
            window: CandidateWindow::new(mtm),
            owner: None,
        })
    });
    reload_preferences();
    security::start_timer(mtm);
}

pub fn reload_preferences() {
    if let Ok(settings) = crate::preferences::load() {
        with(|host| {
            if host.engine.settings() != &settings {
                let _ = host.engine.apply_settings(settings.clone());
                host.window.configure(settings);
            }
        });
    }
}

pub fn with<R>(action: impl FnOnce(&mut Host) -> R) -> Option<R> {
    HOST.with(|host| host.borrow_mut().as_mut().map(action))
}

pub fn discard() {
    with(|host| {
        host.engine.discard();
        host.window.clear();
        host.owner = None;
    });
}

pub fn poll_security() {
    if security::enabled() {
        with(|host| {
            host.engine.set_blocked(true);
            host.window.clear();
        });
    }
}
