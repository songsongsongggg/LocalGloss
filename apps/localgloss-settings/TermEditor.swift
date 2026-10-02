//! 单条词条表单及只读上屏预览；内容只保存在内存。
import AppKit

final class TermEditor: NSObject, NSTextFieldDelegate {
    private let fields = [NSTextField(), NSTextField(), NSTextField()]
    private let errors = (0..<3).map { _ in NSTextField(wrappingLabelWithString: "") }
    private let preview = NSTextView()
    private let tabTranslation: Bool
    private let alert = NSAlert()
    private let usedCodes: Set<String>

    init(term: UserTerm?, usedCodes: Set<String>, tabTranslation: Bool) {
        self.tabTranslation = tabTranslation
        self.usedCodes = usedCodes
        super.init()
        alert.messageText = term == nil ? "新增词条" : "编辑词条"
        alert.informativeText = "先加入草稿；点击主窗口的保存后才会写入本机设置。"
        alert.addButton(withTitle: "加入草稿")
        alert.addButton(withTitle: "取消")
        let view = NSView(frame: NSRect(x: 0, y: 0, width: 500, height: 390))
        let names = ["输入码", "上屏文字", "英文译词（可为空）"]
        let values = [term?.code ?? "", term?.text ?? "", term?.gloss ?? ""]
        for index in 0..<3 {
            let y = CGFloat(340 - index * 80)
            let title = NSTextField(labelWithString: names[index])
            title.frame = NSRect(x: 0, y: y + 25, width: 500, height: 20)
            let field = fields[index]
            field.frame = NSRect(x: 0, y: y, width: 500, height: 24)
            field.stringValue = values[index]
            field.delegate = self
            field.setAccessibilityLabel(names[index])
            errors[index].frame = NSRect(x: 0, y: y - 29, width: 500, height: 27)
            errors[index].textColor = .systemRed
            errors[index].font = .systemFont(ofSize: 11)
            view.addSubview(title); view.addSubview(field); view.addSubview(errors[index])
        }
        let scroll = NSScrollView(frame: NSRect(x: 0, y: 0, width: 500, height: 140))
        scroll.hasVerticalScroller = true
        preview.frame = NSRect(x: 0, y: 0, width: 480, height: 140)
        preview.isEditable = false
        preview.isSelectable = true
        preview.isRichText = false
        preview.isVerticallyResizable = true
        preview.isHorizontallyResizable = false
        preview.autoresizingMask = [.width]
        preview.textContainer?.widthTracksTextView = true
        preview.font = .systemFont(ofSize: 13)
        preview.setAccessibilityLabel("完整词条预览")
        scroll.documentView = preview
        view.addSubview(scroll)
        alert.accessoryView = view
        update()
    }

    private var term: UserTerm {
        UserTerm(code: fields[0].stringValue, text: fields[1].stringValue, gloss: fields[2].stringValue)
    }

    func controlTextDidChange(_ obj: Notification) { update() }

    private func update() {
        let value = term
        var messages = value.fieldErrors
        if usedCodes.contains(value.code) { messages[0] = "输入码已存在，请编辑原词条或更换输入码。" }
        for i in 0..<3 { errors[i].stringValue = messages[i] }
        alert.buttons[0].isEnabled = messages.allSatisfy { $0.isEmpty }
        preview.string = TermPreview.text(for: value, tabTranslation: tabTranslation)
    }

    func run() -> UserTerm? {
        alert.window.initialFirstResponder = fields[0]
        return alert.runModal() == .alertFirstButtonReturn ? term : nil
    }
}
