//! 本机设置与词条草稿窗口；不读取键盘会话或剪贴板。
import AppKit

final class SettingsWindow: NSObject, NSApplicationDelegate, NSWindowDelegate, NSTableViewDataSource, NSTableViewDelegate, NSSearchFieldDelegate {
    private var window: NSWindow!
    private let font = NSPopUpButton()
    private let count = NSPopUpButton()
    private let tab = NSButton(checkboxWithTitle: "Tab / Shift＋Tab 输入译词", target: nil, action: nil)
    private let paging = NSButton(checkboxWithTitle: "− / = 翻页", target: nil, action: nil)
    private let punctuation = NSButton(checkboxWithTitle: "中文标点", target: nil, action: nil)
    private let search = NSSearchField()
    private let table = NSTableView()
    private let status = NSTextField(wrappingLabelWithString: "")
    private var selectedCode: String?
    private let emptyState = NSTextField(labelWithString: "")
    private let summary = NSTextField(labelWithString: "")
    private var draft = SettingsDraft()
    private var baseline: Data?
    private var loaded = false
    private let previewOnly = Bundle.main.object(forInfoDictionaryKey: "LocalGlossPreview") as? Bool == true
    private var visible: [Int] = []
    private var editButton: NSButton!
    private var deleteButton: NSButton!
    private var undoButton: NSButton!
    private var saveButton: NSButton!
    private var importButton: NSButton!
    private var exportButton: NSButton!

    private func add(_ view: NSView, _ rect: NSRect) {
        view.frame = rect; window.contentView!.addSubview(view)
    }
    private func button(_ title: String, _ action: Selector, x: CGFloat, y: CGFloat, width: CGFloat = 100) -> NSButton {
        let control = NSButton(title: title, target: self, action: action)
        add(control, NSRect(x: x, y: y, width: width, height: 32))
        return control
    }
    func applicationDidFinishLaunching(_ notification: Notification) {
        window = NSWindow(contentRect: NSRect(x: 0, y: 0, width: 800, height: 660),
                          styleMask: [.titled, .closable, .miniaturizable], backing: .buffered, defer: false)
        window.title = "LocalGloss 设置与词条"
        window.isReleasedWhenClosed = false
        window.delegate = self
        window.center()
        add(NSTextField(labelWithString: "仅保存主动设置的选项和词条；不自动学习输入，不联网、不上传。"), NSRect(x: 24, y: 614, width: 750, height: 22))
        add(NSTextField(labelWithString: "候选字号"), NSRect(x: 24, y: 576, width: 80, height: 22))
        font.addItems(withTitles: (14...22).map(String.init))
        count.addItems(withTitles: (3...9).map(String.init))
        add(font, NSRect(x: 105, y: 573, width: 85, height: 26))
        add(NSTextField(labelWithString: "每页候选数"), NSRect(x: 235, y: 576, width: 100, height: 22))
        add(count, NSRect(x: 335, y: 573, width: 85, height: 26))
        for control in [font, count] { control.target = self; control.action = #selector(preferencesChanged) }
        for (index, control) in [tab, paging, punctuation].enumerated() {
            add(control, NSRect(x: 24 + index * 255, y: 532, width: 250, height: 26))
            control.target = self; control.action = #selector(preferencesChanged)
        }
        add(summary, NSRect(x: 24, y: 487, width: 360, height: 24))
        search.placeholderString = "搜索输入码、文字或译词"
        search.delegate = self
        search.target = self; search.action = #selector(searchChanged)
        search.sendsSearchStringImmediately = true
        search.setAccessibilityLabel("搜索主动词条")
        add(search, NSRect(x: 420, y: 484, width: 355, height: 28))
        let scroll = NSScrollView(frame: NSRect(x: 24, y: 168, width: 750, height: 305))
        scroll.hasVerticalScroller = true
        scroll.borderType = .bezelBorder
        for (name, width) in [("输入码", 140.0), ("上屏文字", 200.0), ("英文译词", 380.0)] {
            let column = NSTableColumn(identifier: NSUserInterfaceItemIdentifier(name))
            column.title = name; column.width = width
            table.addTableColumn(column)
        }
        table.dataSource = self; table.delegate = self
        table.rowHeight = 28
        table.allowsMultipleSelection = false
        table.doubleAction = #selector(editTerm); table.target = self
        table.setAccessibilityLabel("主动词条列表")
        scroll.documentView = table
        window.contentView!.addSubview(scroll)
        emptyState.alignment = .center
        add(emptyState, NSRect(x: 40, y: 300, width: 710, height: 28))
        _ = button("新增", #selector(addTerm), x: 24, y: 122)
        editButton = button("编辑", #selector(editTerm), x: 130, y: 122)
        deleteButton = button("删除", #selector(deleteTerm), x: 236, y: 122)
        undoButton = button("撤销词条修改", #selector(undoTerms), x: 342, y: 122, width: 140)
        importButton = button("导入词条", #selector(importTerms), x: 550, y: 122)
        exportButton = button("导出词条", #selector(exportTerms), x: 670, y: 122)
        _ = button("放弃并重新加载", #selector(reloadSettings), x: 485, y: 78, width: 160)
        saveButton = button("保存设置", #selector(saveSettings), x: 655, y: 78, width: 120)
        add(status, NSRect(x: 24, y: 18, width: 750, height: 54))
        load()
        let menu = NSMenu()
        let appMenu = NSMenu(); let appItem = NSMenuItem(); appItem.submenu = appMenu; menu.addItem(appItem)
        appMenu.addItem(withTitle: "退出设置", action: #selector(NSApplication.terminate(_:)), keyEquivalent: "q")
        let editMenu = NSMenu(title: "编辑"); let editItem = NSMenuItem(title: "编辑", action: nil, keyEquivalent: ""); editItem.submenu = editMenu; menu.addItem(editItem)
        for (title, action, key) in [("剪切", #selector(NSText.cut(_:)), "x"), ("复制", #selector(NSText.copy(_:)), "c"), ("粘贴", #selector(NSText.paste(_:)), "v"), ("全选", #selector(NSText.selectAll(_:)), "a")] {
            editMenu.addItem(withTitle: title, action: action, keyEquivalent: key)
        }
        NSApp.mainMenu = menu
        window.makeKeyAndOrderFront(nil)
        NSApp.activate(ignoringOtherApps: true)
    }

    private func load() {
        do {
            let before = previewOnly ? nil : try SettingsModel.snapshot()
            var model = previewOnly ? SettingsModel() : try SettingsModel.load()
            if previewOnly {
                model.terms = [UserTerm(code: "kaifa", text: "开发", gloss: "develop"), UserTerm(code: "ceshi", text: "测试", gloss: "test")]
            }
            if !previewOnly {
                guard before == (try SettingsModel.snapshot()) else { throw ValidationError.invalid("加载期间设置发生变化，请重新加载。") }
            }
            baseline = before; draft = SettingsDraft(model); selectedCode = nil; loaded = true
            font.selectItem(withTitle: String(model.font_size)); count.selectItem(withTitle: String(model.page_size))
            tab.state = model.tab_translation ? .on : .off
            paging.state = model.symbol_paging ? .on : .off
            punctuation.state = model.chinese_punctuation ? .on : .off
            status.stringValue = "编辑先进入内存草稿；只有保存才写入。保存后切换一次输入法生效。"
        } catch { loaded = false; status.stringValue = "已有设置无法安全加载，已禁止编辑与覆盖。原文件保持不变。" }
        if previewOnly { status.stringValue = "隔离预览：虚构初始词条，不读取或保存个人设置；导入导出仅操作主动选择的文件。" }
        refresh()
    }
    private func refresh() {
        let code = selectedCode
        visible = draft.indices(matching: search.stringValue)
        table.deselectAll(nil); table.reloadData()
        if let row = draft.visibleRow(for: code, matching: search.stringValue) {
            table.selectRowIndexes(IndexSet(integer: row), byExtendingSelection: false)
            table.scrollRowToVisible(row)
        }
        emptyState.stringValue = draft.model.terms.isEmpty ? "尚无词条，点击「新增」添加。" : "没有匹配词条，请调整或清除搜索。"
        emptyState.isHidden = !visible.isEmpty
        summary.stringValue = "主动词条 \(visible.count) / \(draft.model.terms.count)（最多 500）\(draft.isDirty ? " · 未保存" : "")"
        window.isDocumentEdited = draft.isDirty
        saveButton.isEnabled = loaded && draft.isDirty && !previewOnly
        undoButton.isEnabled = loaded && draft.canUndo
        importButton.isEnabled = loaded
        exportButton.isEnabled = loaded
        tableViewSelectionDidChange(Notification(name: NSTableView.selectionDidChangeNotification))
    }
    private var selectedIndex: Int? { visible.indices.contains(table.selectedRow) ? visible[table.selectedRow] : nil }
    func numberOfRows(in tableView: NSTableView) -> Int { visible.count }
    func tableView(_ tableView: NSTableView, viewFor column: NSTableColumn?, row: Int) -> NSView? {
        guard visible.indices.contains(row) else { return nil }
        let term = draft.model.terms[visible[row]]
        let value: String
        switch column?.identifier.rawValue { case "输入码": value = term.code; case "上屏文字": value = term.text; default: value = term.gloss }
        let cell = NSTextField(labelWithString: value)
        cell.lineBreakMode = .byTruncatingTail; cell.toolTip = value
        return cell
    }
    func tableViewSelectionDidChange(_ notification: Notification) {
        selectedCode = selectedIndex.map { draft.model.terms[$0].code }
        editButton.isEnabled = loaded && selectedIndex != nil
        deleteButton.isEnabled = loaded && selectedIndex != nil
    }
    func controlTextDidChange(_ obj: Notification) { refresh() }
    @objc private func searchChanged() { refresh() }
    @objc private func preferencesChanged() {
        guard loaded else { return }
        draft.model.font_size = Int(font.titleOfSelectedItem ?? "16") ?? 16
        draft.model.page_size = Int(count.titleOfSelectedItem ?? "9") ?? 9
        draft.model.tab_translation = tab.state == .on
        draft.model.symbol_paging = paging.state == .on
        draft.model.chinese_punctuation = punctuation.state == .on
        refresh()
    }
    private func edit(at index: Int?) {
        guard loaded else { return }
        let used = Set(draft.model.terms.enumerated().filter { $0.offset != index }.map { $0.element.code })
        let editor = TermEditor(term: index.map { draft.model.terms[$0] }, usedCodes: used, tabTranslation: draft.model.tab_translation)
        guard let term = editor.run() else { return }
        do {
            try draft.put(term, at: index)
            selectedCode = term.code
            refresh()
            if selectedIndex == nil { status.stringValue = "词条已加入草稿；当前搜索条件隐藏了该词条，清除搜索可查看。" }
        }
        catch { status.stringValue = error.localizedDescription }
    }
    @objc private func addTerm() { edit(at: nil) }
    @objc private func editTerm() { if let index = selectedIndex { edit(at: index) } }
    @objc private func deleteTerm() { if let index = selectedIndex { draft.remove(at: index); refresh() } }
    @objc private func undoTerms() { draft.undo(); refresh() }
    @objc private func importTerms() {
        guard loaded else { return }
        let panel = NSOpenPanel()
        panel.title = "选择本地词条 JSON 文件"
        panel.canChooseDirectories = false; panel.allowsMultipleSelection = false; panel.resolvesAliases = false
        guard panel.runModal() == .OK, let url = panel.url else { return }
        do {
            let exchange = try TermsExchange.read(from: url)
            let existing = Dictionary(uniqueKeysWithValues: draft.model.terms.map { ($0.code, $0) })
            let conflicts = exchange.terms.filter { existing[$0.code] != nil && existing[$0.code] != $0 }
            let newCount = exchange.terms.filter { existing[$0.code] == nil }.count
            let duplicateCount = exchange.terms.count - conflicts.count - newCount
            let preview = NSAlert(); preview.messageText = "预览导入"
            preview.informativeText = "新增 \(newCount) 项、相同 \(duplicateCount) 项、冲突 \(conflicts.count) 项。仅加入草稿，点击保存后才生效。"
            preview.addButton(withTitle: "取消"); preview.addButton(withTitle: "继续")
            guard preview.runModal() == .alertSecondButtonReturn else { return }
            var replacements = Set<String>()
            for term in conflicts {
                let alert = NSAlert(); alert.messageText = "输入码 \(term.code) 存在冲突"
                let old = existing[term.code]!
                alert.informativeText = "原词条：\(old.text)\n原译词：\(old.gloss)\n\n导入词条：\(term.text)\n导入译词：\(term.gloss)"
                alert.addButton(withTitle: "保留原项"); alert.addButton(withTitle: "替换此项"); alert.addButton(withTitle: "取消整批")
                switch alert.runModal() {
                case .alertFirstButtonReturn: break
                case .alertSecondButtonReturn: replacements.insert(term.code)
                default: return
                }
            }
            try draft.merge(exchange, replacing: replacements)
            refresh()
            status.stringValue = "导入已处理，未保存的更改仅在草稿中；可撤销或放弃。"
        } catch let error as ValidationError { status.stringValue = error.localizedDescription }
        catch { status.stringValue = "词条文件无法解析，未导入；草稿保持不变。" }
    }
    @objc private func exportTerms() {
        guard loaded else { return }
        let warning = NSAlert(); warning.messageText = "导出当前草稿中的词条？"
        warning.informativeText = "导出为明文 JSON，仅含主动词条。包含尚未保存的词条更改，不含其他设置。云盘目录可能由其他软件同步；已有文件不会被覆盖。"
        warning.addButton(withTitle: "取消"); warning.addButton(withTitle: "选择新文件")
        guard warning.runModal() == .alertSecondButtonReturn else { return }
        let panel = NSSavePanel(); panel.nameFieldStringValue = "localgloss-terms.json"
        panel.title = "导出词条到新文件"
        guard panel.runModal() == .OK, let url = panel.url else { return }
        do {
            try TermsExchange(terms: draft.model.terms).exportNew(to: url)
            status.stringValue = "已导出词条。正式设置未改变，草稿未自动保存。"
        } catch let error as ValidationError { status.stringValue = error.localizedDescription }
        catch { status.stringValue = "导出失败，草稿与正式设置保持不变。" }
    }
    @objc private func reloadSettings() {
        if draft.isDirty {
            let alert = NSAlert(); alert.messageText = "放弃尚未保存的更改？"
            alert.addButton(withTitle: "取消"); alert.addButton(withTitle: "放弃并重新加载")
            guard alert.runModal() == .alertSecondButtonReturn else { return }
        }
        load()
    }
    @objc private func saveSettings() { _ = save() }
    private func save() -> Bool {
        guard loaded && !previewOnly else { return false }
        do {
            let model = draft.model
            try model.save(expected: baseline, checkConflict: true)
            // 保存的精确字节作新基线；后续外部修改不能被误认作本次保存。
            let encoder = JSONEncoder(); encoder.outputFormatting = [.prettyPrinted, .sortedKeys]
            baseline = try encoder.encode(model)
            draft = SettingsDraft(model); refresh()
            status.stringValue = "已保存。切换一次输入法生效；没有保存日常输入。"
            return true
        } catch let error as ValidationError { status.stringValue = error.localizedDescription }
        catch { status.stringValue = "保存失败，草稿保留。请检查设置目录；未输出词条内容。" }
        return false
    }
    private func allowClose() -> Bool {
        guard draft.isDirty else { return true }
        let alert = NSAlert(); alert.messageText = "有尚未保存的更改"
        alert.addButton(withTitle: "保存"); alert.addButton(withTitle: "放弃"); alert.addButton(withTitle: "取消")
        alert.buttons[0].isEnabled = !previewOnly
        switch alert.runModal() {
        case .alertFirstButtonReturn: return save()
        case .alertSecondButtonReturn: load(); return true
        default: return false
        }
    }
    func windowShouldClose(_ sender: NSWindow) -> Bool { allowClose() }
    func applicationShouldTerminate(_ sender: NSApplication) -> NSApplication.TerminateReply { allowClose() ? .terminateNow : .terminateCancel }
    func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool { window.makeKeyAndOrderFront(nil); return true }
}
