//! 仅读取用户明确保存的设置；设置编辑器是随包的独立本机应用。
use std::io::Read;
use std::path::PathBuf;

use localgloss_engine::Settings;
use objc2::rc::Retained;
use objc2::{MainThreadMarker, sel};
use objc2_app_kit::{NSMenu, NSMenuItem};
use objc2_foundation::{NSBundle, NSString};

pub fn path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| {
        PathBuf::from(home).join("Library/Application Support/LocalGloss/settings.json")
    })
}

pub fn load() -> Result<Settings, &'static str> {
    let path = path().ok_or("用户目录不可用")?;
    let file = match std::fs::File::open(&path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Settings::default());
        }
        Err(_) => return Err("设置无法读取"),
    };
    let mut bytes = Vec::new();
    file.take(Settings::MAX_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "设置无法读取")?;
    Settings::parse(&bytes)
}

pub fn menu(mtm: MainThreadMarker) -> Retained<NSMenu> {
    let menu = NSMenu::new(mtm);
    let english = crate::host::with(|host| host.engine.english_mode()).unwrap_or(false);
    let status = NSMenuItem::new(mtm);
    status.setTitle(&NSString::from_str(if english {
        "英文直通 · Ctrl⇧Space 切换"
    } else {
        "中文译词 · Ctrl⇧Space 切换"
    }));
    status.setEnabled(false);
    menu.addItem(&status);
    let item = unsafe {
        NSMenuItem::initWithTitle_action_keyEquivalent(
            mtm.alloc(),
            &NSString::from_str("LocalGloss 设置与词条…"),
            Some(sel!(openLocalGlossSettings:)),
            &NSString::new(),
        )
    };
    menu.addItem(&item);
    menu
}

pub fn open() {
    let Some(bundle) = NSBundle::mainBundle().resourcePath() else {
        return;
    };
    let app = PathBuf::from(bundle.to_string()).join("LocalGloss Settings.app");
    if app.is_dir() {
        // 固定随包路径，无 shell、用户参数或输入文本。open 很快返回，由线程回收子进程。
        if let Ok(mut child) = std::process::Command::new("/usr/bin/open").arg(app).spawn() {
            std::thread::spawn(move || {
                let _ = child.wait();
            });
        }
    }
}
