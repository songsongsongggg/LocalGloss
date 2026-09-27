//! 本机设置窗口；独立于输入法进程，不读取键盘会话或剪贴板。
import AppKit

final class SettingsWindow: NSObject, NSApplicationDelegate {
    private var window: NSWindow!
    private let font = NSPopUpButton()
    private let count = NSPopUpButton()
    private let tab = NSButton(checkboxWithTitle: "Tab / Shift＋Tab 输入译词（可关闭以保留编辑器习惯）", target: nil, action: nil)
    private let paging = NSButton(checkboxWithTitle: "− / = 翻页（关闭后仍可用 Page Up / Page Down）", target: nil, action: nil)
    private let punctuation = NSButton(checkboxWithTitle: "中文标点：，。？！；（网址、邮箱建议使用英文模式）", target: nil, action: nil)
    private let terms = NSTextView()
    private let status = NSTextField(wrappingLabelWithString: "")

    private func label(_ text: String, y: CGFloat, x: CGFloat = 24, width: CGFloat = 710) {
        let field = NSTextField(wrappingLabelWithString: text)
        field.frame = NSRect(x: x, y: y, width: width, height: 36)
        window.contentView!.addSubview(field)
    }
    func applicationDidFinishLaunching(_ notification: Notification) {
        window = NSWindow(contentRect: NSRect(x: 0, y: 0, width: 760, height: 620),
                          styleMask: [.titled, .closable, .miniaturizable], backing: .buffered, defer: false)
        window.title = "LocalGloss 设置与词条"
        window.isReleasedWhenClosed = false
        window.center()
        label("仅保存你主动设置的选项和词条；不自动学习输入，不联网、不上传。", y: 565)
        label("候选字号", y: 523, width: 80)
        label("每页候选数", y: 523, x: 250, width: 95)
        font.addItems(withTitles: (14...22).map(String.init))
        count.addItems(withTitles: (3...9).map(String.init))
        font.frame = NSRect(x: 110, y: 530, width: 85, height: 25)
        count.frame = NSRect(x: 350, y: 530, width: 85, height: 25)
        window.contentView!.addSubview(font); window.contentView!.addSubview(count)
        for (index, control) in [tab, paging, punctuation].enumerated() {
            control.frame = NSRect(x: 24, y: 479 - index * 33, width: 710, height: 26)
            window.contentView!.addSubview(control)
        }
        label("Ctrl＋Shift＋Space 切换英文直通；F1 展开完整释义。", y: 365)
        label("主动词条：每行「输入码 | 上屏文字 | 英文译词」；同码词条置顶。\n示例：kaifa | 开发 | develop（仅为格式示例，不会自动保存）", y: 312)
        let scroll = NSScrollView(frame: NSRect(x: 24, y: 130, width: 710, height: 177))
        scroll.hasVerticalScroller = true
        scroll.borderType = .bezelBorder
        terms.isRichText = false
        terms.isAutomaticQuoteSubstitutionEnabled = false
        terms.isAutomaticDashSubstitutionEnabled = false
        terms.isAutomaticTextReplacementEnabled = false
        terms.isContinuousSpellCheckingEnabled = false
        terms.font = NSFont.monospacedSystemFont(ofSize: 14, weight: .regular)
        terms.isHorizontallyResizable = false
        terms.autoresizingMask = [.width]
        terms.textContainer?.widthTracksTextView = true
        terms.frame = NSRect(x: 0, y: 0, width: 690, height: 177)
        terms.setAccessibilityLabel("主动词条，每行输入码、文字和译词，以竖线分隔")
        scroll.documentView = terms
        window.contentView!.addSubview(scroll)
        let save = NSButton(title: "保存设置", target: self, action: #selector(saveSettings))
        save.frame = NSRect(x: 600, y: 80, width: 135, height: 32)
        window.contentView!.addSubview(save)
        status.frame = NSRect(x: 24, y: 22, width: 710, height: 52)
        window.contentView!.addSubview(status)
        do { display(try SettingsModel.load()); status.stringValue = "保存后切回输入窗口生效；若未更新，请切换一次输入法。词条文件仅当前用户可读写。" }
        catch { display(SettingsModel()); save.isEnabled = false; status.stringValue = "已有设置无法加载，已禁止覆盖。请检查本机 settings.json；原文件保持不变。" }
        let menu = NSMenu()
        let appMenu = NSMenu(); let appItem = NSMenuItem(); appItem.submenu = appMenu; menu.addItem(appItem)
        appMenu.addItem(withTitle: "退出设置", action: #selector(NSApplication.terminate(_:)), keyEquivalent: "q")
        NSApp.mainMenu = menu
        window.makeKeyAndOrderFront(nil)
        NSApp.activate(ignoringOtherApps: true)
    }
    func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
        window.makeKeyAndOrderFront(nil)
        return true
    }
    private func display(_ model: SettingsModel) {
        font.selectItem(withTitle: String(model.font_size)); count.selectItem(withTitle: String(model.page_size))
        tab.state = model.tab_translation ? .on : .off
        paging.state = model.symbol_paging ? .on : .off
        punctuation.state = model.chinese_punctuation ? .on : .off
        terms.string = model.terms.map { "\($0.code) | \($0.text) | \($0.gloss)" }.joined(separator: "\n")
    }
    @objc private func saveSettings() {
        do {
            var model = SettingsModel()
            model.font_size = Int(font.titleOfSelectedItem ?? "16") ?? 16
            model.page_size = Int(count.titleOfSelectedItem ?? "9") ?? 9
            model.tab_translation = tab.state == .on
            model.symbol_paging = paging.state == .on
            model.chinese_punctuation = punctuation.state == .on
            model.terms = try terms.string.split(separator: "\n", omittingEmptySubsequences: true).map { line in
                let pieces = line.split(separator: "|", maxSplits: 2, omittingEmptySubsequences: false)
                    .map { $0.trimmingCharacters(in: .whitespaces) }
                guard pieces.count == 3 else { throw ValidationError.invalid("每行须包含两条竖线：输入码 | 文字 | 译词。译词可以为空。") }
                return UserTerm(code: pieces[0], text: pieces[1], gloss: pieces[2])
            }
            try model.save()
            status.stringValue = "已保存。切回输入窗口生效；若仍为旧设置，请切换一次输入法。没有保存日常输入。"
        } catch let error as ValidationError { status.stringValue = error.localizedDescription }
        catch { status.stringValue = "保存失败；请检查设置目录是否可写。未输出词条内容。" }
    }
}
