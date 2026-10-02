//! LocalGloss 的 Mac 入口；不注册输入源、不安装、不加载个人数据。
mod client;
mod controller;
mod host;
mod layout;
mod preferences;
mod security;
mod view;
mod window;

use localgloss_engine::OfflineEngine;
use objc2::{AnyThread, ClassType, MainThreadMarker};
use objc2_app_kit::NSApplication;
use objc2_foundation::{NSBundle, NSString};
use objc2_input_method_kit::IMKServer;

const BUNDLE_ID: &str = "local.localgloss.inputmethod";
const CONNECTION: &str = "local.localgloss.inputmethod_Connection";

fn main() {
    // panic payload 可能包含数据，只输出固定错误；回调在控制器边界捕获。
    std::panic::set_hook(Box::new(|_| {
        eprintln!("LocalGloss internal error (input omitted)")
    }));
    let mtm = MainThreadMarker::new().expect("main thread required");
    let Some(resources) = NSBundle::mainBundle().resourcePath() else {
        eprintln!("LocalGloss resources unavailable");
        return;
    };
    let resources = std::path::PathBuf::from(resources.to_string());
    let Ok(engine) = OfflineEngine::from_paths(
        &resources.join("dict.tsv"),
        &resources.join("glossary-en.tsv"),
    ) else {
        eprintln!("LocalGloss dictionary unavailable");
        return;
    };
    controller::LocalGlossInputController::class();
    host::init(mtm, engine);
    let server = unsafe {
        IMKServer::initWithName_bundleIdentifier(
            IMKServer::alloc(),
            Some(&NSString::from_str(CONNECTION)),
            Some(&NSString::from_str(BUNDLE_ID)),
        )
    };
    let Some(_server) = server else {
        eprintln!("LocalGloss input server unavailable");
        return;
    };
    NSApplication::sharedApplication(mtm).run();
}
