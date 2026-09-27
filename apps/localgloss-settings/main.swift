//! 设置应用入口；限制新建文件权限，不注册输入源。
import AppKit
import Darwin

umask(0o077)
let application = NSApplication.shared
let delegate = SettingsWindow()
application.delegate = delegate
application.setActivationPolicy(.regular)
application.run()
