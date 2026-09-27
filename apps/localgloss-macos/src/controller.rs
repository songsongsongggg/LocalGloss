//! IMK 回调先检查安全输入和会话身份，再映射到离线操作。
use localgloss_engine::Key;
use objc2::rc::{Allocated, Retained};
use objc2::runtime::AnyObject;
use objc2::{MainThreadMarker, define_class, msg_send};
use objc2_app_kit::{NSEvent, NSEventModifierFlags, NSEventType, NSMenu};
use objc2_foundation::NSObjectProtocol;
use objc2_input_method_kit::{IMKInputController, IMKServer};

use crate::client::TextClient;
use crate::{host, security};

define_class!(
    // SAFETY: IMKInputController 是 Apple 输入法指定的可继承入口。
    #[unsafe(super(IMKInputController))]
    #[name = "LocalGlossInputController"]
    #[ivars = ()]
    pub struct LocalGlossInputController;

    impl LocalGlossInputController {
        #[unsafe(method_id(menu))]
        fn menu(&self) -> Option<Retained<NSMenu>> {
            MainThreadMarker::new().map(crate::preferences::menu)
        }

        #[unsafe(method(openLocalGlossSettings:))]
        fn open_settings(&self, _sender: Option<&AnyObject>) {
            host::discard();
            crate::preferences::open();
        }
        #[unsafe(method_id(initWithServer:delegate:client:))]
        fn init_with_server(this: Allocated<Self>, server: Option<&IMKServer>,
            delegate: Option<&AnyObject>, client: Option<&AnyObject>) -> Option<Retained<Self>> {
            let this = this.set_ivars(());
            unsafe { msg_send![super(this), initWithServer: server, delegate: delegate, client: client] }
        }

        #[unsafe(method(handleEvent:client:))]
        fn handle_event(&self, event: Option<&NSEvent>, client: Option<&AnyObject>) -> bool {
            match (event, client) {
                (Some(event), Some(client)) => std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| self.dispatch(event, TextClient::new(client))),
                ).unwrap_or_else(|_| { host::discard(); false }),
                _ => false,
            }
        }

        #[unsafe(method(activateServer:))]
        fn activate_server(&self, sender: Option<&AnyObject>) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                host::reload_preferences();
                let blocked = security::enabled() || sender.map(TextClient::new)
                    .and_then(|client| client.application()).as_deref() == Some("com.apple.loginwindow");
                host::with(|host| {
                    host.engine.discard();
                    host.window.clear();
                    host.engine.set_blocked(blocked);
                    host.owner = Some(self.owner());
                });
            })).map_err(|_| host::discard());
        }

        #[unsafe(method(commitComposition:))]
        fn commit_composition(&self, client: Option<&AnyObject>) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if let Some(client) = client.map(TextClient::new) && self.allowed(client) {
                    let text = host::with(|host| host.engine.finish_raw()).flatten();
                    if let Some(text) = text { client.insert(&text); }
                }
                host::with(|host| {
                    if host.owner == Some(self.owner()) {
                        host.engine.discard();
                        host.window.clear();
                    }
                });
            })).map_err(|_| host::discard());
        }

        #[unsafe(method(deactivateServer:))]
        fn deactivate_server(&self, _sender: Option<&AnyObject>) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                host::with(|host| {
                    if host.owner == Some(self.owner()) {
                        host.engine.discard();
                        host.window.clear();
                        host.owner = None;
                    }
                });
            })).map_err(|_| host::discard());
        }
    }
    unsafe impl NSObjectProtocol for LocalGlossInputController {}
);

impl LocalGlossInputController {
    fn owner(&self) -> usize {
        self as *const Self as usize
    }

    fn allowed(&self, client: TextClient<'_>) -> bool {
        let blocked =
            security::enabled() || client.application().as_deref() == Some("com.apple.loginwindow");
        host::with(|host| {
            if host.owner != Some(self.owner()) {
                return false;
            }
            host.engine.set_blocked(blocked);
            if blocked {
                host.window.clear();
            }
            !blocked
        })
        .unwrap_or(false)
    }

    fn dispatch(&self, event: &NSEvent, client: TextClient<'_>) -> bool {
        if !self.allowed(client) || event.r#type() != NSEventType::KeyDown {
            return false;
        }
        let flags = event.modifierFlags();
        let command = flags.contains(NSEventModifierFlags::Command);
        let control = flags.contains(NSEventModifierFlags::Control);
        let option = flags.contains(NSEventModifierFlags::Option);
        let shift = flags.contains(NSEventModifierFlags::Shift);
        let caps = flags.contains(NSEventModifierFlags::CapsLock);
        let code = event.keyCode();
        let (tab_translation, symbol_paging) = host::with(|host| {
            let settings = host.engine.settings();
            (settings.tab_translation, settings.symbol_paging)
        })
        .unwrap_or((true, true));
        let key = if control && shift && !command && !option && code == 49 {
            Key::ToggleEnglish
        } else if command || control || caps {
            Key::PassThrough
        } else if option {
            match digit(code) {
                Some(digit) => Key::Translation {
                    digit,
                    second: shift,
                },
                None => Key::PassThrough,
            }
        } else {
            match code {
                51 => Key::Backspace,
                49 => Key::Space,
                36 | 76 => Key::Enter,
                53 => Key::Escape,
                48 if tab_translation => Key::HighlightedTranslation { second: shift },
                122 => Key::ToggleDetails,
                126 => Key::Previous,
                125 => Key::Next,
                116 => Key::PagePrevious,
                121 => Key::PageNext,
                27 if !shift && symbol_paging => Key::PagePrevious,
                24 if !shift && symbol_paging => Key::PageNext,
                123 => Key::Left,
                124 => Key::Right,
                _ => {
                    if let Some(digit) = digit(code) {
                        if shift {
                            event
                                .characters()
                                .map(|text| text.to_string())
                                .as_deref()
                                .and_then(single_character)
                                .map(Key::Literal)
                                .unwrap_or(Key::PassThrough)
                        } else {
                            Key::Digit(digit)
                        }
                    } else {
                        let text = event.characters().map(|text| text.to_string());
                        match text.as_deref().and_then(single_character) {
                            Some(letter) if letter.is_ascii_lowercase() || letter == '\'' => {
                                Key::Letter(letter)
                            }
                            Some(character) => Key::Literal(character),
                            _ => Key::PassThrough,
                        }
                    }
                }
            }
        };
        let Some(outcome) = host::with(|host| host.engine.handle(key)) else {
            return false;
        };
        if !outcome.handled && outcome.commit.is_none() {
            return false;
        }
        // 不跨 AppKit 调用持有 Host 借用，避免客户端同步回调导致重入。
        if let Some(text) = &outcome.commit {
            client.insert(text);
        }
        if !self.allowed(client) {
            return outcome.handled;
        }
        client.mark(&outcome.frame.preedit, outcome.frame.cursor);
        if !self.allowed(client) {
            return outcome.handled;
        }
        let caret = if outcome.frame.preedit.is_empty() {
            objc2_foundation::NSRect::ZERO
        } else {
            client.caret()
        };
        host::with(|host| host.window.show(outcome.frame, caret));
        outcome.handled
    }
}

fn single_character(text: &str) -> Option<char> {
    let mut letters = text.chars();
    let letter = letters.next()?;
    letters.next().is_none().then_some(letter)
}

fn digit(code: u16) -> Option<usize> {
    Some(match code {
        18 | 83 => 1,
        19 | 84 => 2,
        20 | 85 => 3,
        21 | 86 => 4,
        23 | 87 => 5,
        22 | 88 => 6,
        26 | 89 => 7,
        28 | 91 => 8,
        25 | 92 => 9,
        _ => return None,
    })
}
