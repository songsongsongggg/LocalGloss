//! 每次回调检查安全输入，定时清除无按键期间的旧候选。
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{MainThreadMarker, MainThreadOnly, define_class, msg_send, sel};
use objc2_foundation::{NSObject, NSObjectProtocol, NSTimer};

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    fn IsSecureEventInputEnabled() -> bool;
}

pub fn enabled() -> bool {
    // SAFETY: Carbon 无参数只读查询。
    unsafe { IsSecureEventInputEnabled() }
}

define_class!(
    // SAFETY: NSObject 允许子类化；全部回调在主线程执行。
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[ivars = ()]
    struct SecurityTick;

    impl SecurityTick {
        #[unsafe(method(tick:))]
        fn tick(&self, _timer: Option<&AnyObject>) {
            if std::panic::catch_unwind(std::panic::AssertUnwindSafe(crate::host::poll_security)).is_err() {
                crate::host::discard();
            }
        }
    }
    unsafe impl NSObjectProtocol for SecurityTick {}
);

pub fn start_timer(mtm: MainThreadMarker) {
    let this = mtm.alloc::<SecurityTick>().set_ivars(());
    let target: Retained<SecurityTick> = unsafe { msg_send![super(this), init] };
    // 定时器由 run loop 持有，target 无客户端或输入内容。
    let _timer = unsafe {
        NSTimer::scheduledTimerWithTimeInterval_target_selector_userInfo_repeats(
            0.1,
            &target,
            sel!(tick:),
            None,
            true,
        )
    };
}
