//! 在隔离临时目录验证用户主动保存路径和原子替换，绝不读写真实偏好。
import Foundation
import Darwin
umask(0o077)
let directory = FileManager.default.temporaryDirectory.appendingPathComponent("localgloss-settings-test-" + UUID().uuidString)
var model = SettingsModel()
model.terms = [UserTerm(code: "kf", text: "开发", gloss: "develop")]
try model.save(to: directory)
let file = directory.appendingPathComponent("settings.json")
let first = try Data(contentsOf: file)
let restored = try SettingsModel.load(from: file)
assert(restored.terms.first?.gloss == "develop")
let mode = try FileManager.default.attributesOfItem(atPath: file.path)[.posixPermissions] as! NSNumber
assert(mode.intValue == 0o600)
model.font_size = 22
try model.save(to: directory)
let updated = try JSONDecoder().decode(SettingsModel.self, from: Data(contentsOf: file))
assert(updated.font_size == 22)
let good = try Data(contentsOf: file)
model.terms.append(model.terms[0])
do { try model.save(to: directory); fatalError("duplicate code accepted") } catch {}
let afterFailure = try Data(contentsOf: file)
assert(afterFailure == good)
let symlinkDirectory = directory.appendingPathComponent("symlink-target")
try FileManager.default.createSymbolicLink(at: symlinkDirectory, withDestinationURL: directory)
model.terms.removeLast()
do { try model.save(to: symlinkDirectory); fatalError("symlink accepted") } catch {}
let futureFile = directory.appendingPathComponent("future.json")
var object = try JSONSerialization.jsonObject(with: good) as! [String: Any]
object["future_setting"] = true
try JSONSerialization.data(withJSONObject: object).write(to: futureFile)
do { _ = try SettingsModel.load(from: futureFile); fatalError("unknown field accepted") } catch {}
print("settings tests passed: round trip, 0600 permissions, replacement, invalid-save preservation, symlink rejection, unknown-field rejection")

// 搜索结果必须映射回原始索引；撤销不影响其他偏好。
var initial = SettingsModel()
initial.terms = [UserTerm(code: "aa", text: "甲", gloss: "first"), UserTerm(code: "bb", text: "乙", gloss: "second")]
var draft = SettingsDraft(initial)
assert(!draft.isDirty)
assert(draft.indices(matching: "SECOND") == [1])
try draft.put(UserTerm(code: "bb", text: "更新", gloss: "updated"), at: draft.indices(matching: "乙").first)
assert(draft.model.terms[0] == initial.terms[0] && draft.model.terms[1].text == "更新")
assert(draft.isDirty)
draft.remove(at: 1)
assert(draft.model.terms.count == 1)
draft.undo()
assert(draft.model.terms[1].text == "更新")
draft.undo()
assert(!draft.isDirty)
do { try draft.put(initial.terms[0], at: nil); fatalError("duplicate draft accepted") } catch {}
assert(!draft.isDirty && !draft.canUndo)
draft.model.font_size = 18
try draft.put(UserTerm(code: "cc", text: "丙", gloss: ""), at: nil)
draft.undo()
assert(draft.model.font_size == 18 && draft.model.terms == initial.terms)

// 500 条上限，Unicode 标量、控制字符与输入码规则。
var limit = SettingsModel()
limit.terms = (0..<500).map { i in
    let code = String(UnicodeScalar(97 + i / 26)!) + String(UnicodeScalar(97 + i % 26)!)
    return UserTerm(code: code, text: "测试", gloss: "")
}
try limit.validate()
var limitDraft = SettingsDraft(limit)
do { try limitDraft.put(UserTerm(code: "zzzz", text: "超限", gloss: ""), at: nil); fatalError("501 accepted") } catch {}
assert(limitDraft.model == limit)
for term in [UserTerm(code: "A", text: "词", gloss: ""), UserTerm(code: "a", text: "词\n", gloss: ""), UserTerm(code: "a", text: "词", gloss: "a\t"), UserTerm(code: "a", text: String(repeating: "😀", count: 65), gloss: "")] {
    assert(term.fieldErrors.contains { !$0.isEmpty })
}
let unicode = UserTerm(code: "a", text: String(repeating: "😀", count: 64), gloss: String(repeating: "译", count: 240))
assert(unicode.fieldErrors.allSatisfy { $0.isEmpty })

// 两个窗口从相同基线编辑，第二个保存必须失败并保留首个结果。
let baseline = try SettingsModel.snapshot(of: file)
var firstWindow = try SettingsModel.load(from: file)
var secondWindow = firstWindow
firstWindow.page_size = 3
try firstWindow.save(to: directory, expected: baseline, checkConflict: true)
let firstWindowBytes = try Data(contentsOf: file)
secondWindow.page_size = 4
do { try secondWindow.save(to: directory, expected: baseline, checkConflict: true); fatalError("conflict overwritten") } catch {}
let afterConflict = try Data(contentsOf: file)
assert(afterConflict == firstWindowBytes)
let absent = directory.appendingPathComponent("new")
try initial.save(to: absent, expected: nil, checkConflict: true)
do { try initial.save(to: absent, expected: nil, checkConflict: true); fatalError("created file overwritten") } catch {}
let linkedFile = directory.appendingPathComponent("linked.json")
try FileManager.default.createSymbolicLink(at: linkedFile, withDestinationURL: file)
do { _ = try SettingsModel.load(from: linkedFile); fatalError("symlink read") } catch {}
let blocked = directory.appendingPathComponent("not-a-directory")
try Data("unchanged".utf8).write(to: blocked)
do { try initial.save(to: blocked); fatalError("invalid destination accepted") } catch {}
let afterBlocked = try Data(contentsOf: blocked)
assert(afterBlocked == Data("unchanged".utf8))
print("editor tests passed: filtered target, draft undo, limits, Unicode, duplicate rejection, stale baseline, created-file conflict, symlink read rejection, write failure preservation")

// 预览必须遵循 Tab 开关，并完整保留最长允许译词。
let longTerm = UserTerm(code: "demo", text: "演示", gloss: String(repeating: "长", count: 240))
assert(TermPreview.text(for: longTerm, tabTranslation: true).hasSuffix(longTerm.gloss))
assert(TermPreview.text(for: longTerm, tabTranslation: false).contains("快捷键已关闭"))
assert(!TermPreview.text(for: longTerm, tabTranslation: false).contains(longTerm.gloss))
assert(TermPreview.text(for: UserTerm(code: "a", text: "甲", gloss: ""), tabTranslation: true).contains("本地候选"))
// 另一个保存者持锁时立即拒绝，不覆盖原文件，也不阻塞 UI。
let lockFD = open(directory.path, O_RDONLY)
assert(lockFD >= 0 && flock(lockFD, LOCK_EX | LOCK_NB) == 0)
let beforeLock = try Data(contentsOf: file)
do { try initial.save(to: directory); fatalError("held lock ignored") } catch {}
let afterLock = try Data(contentsOf: file)
assert(beforeLock == afterLock)
flock(lockFD, LOCK_UN)
close(lockFD)
print("alpha2 tests passed: Tab-disabled preview, full-length preview, empty translation fallback, held-lock preservation")

var selectionDraft = SettingsDraft(initial)
assert(selectionDraft.visibleRow(for: "bb", matching: "second") == 0)
assert(selectionDraft.visibleRow(for: "aa", matching: "second") == nil)
assert(selectionDraft.visibleRow(for: "bb", matching: "") == 1)
try selectionDraft.put(initial.terms[1], at: 1)
assert(!selectionDraft.canUndo && !selectionDraft.isDirty)
try selectionDraft.put(UserTerm(code: "cc", text: "丙", gloss: "third"), at: 1)
assert(selectionDraft.visibleRow(for: "cc", matching: "") == 1)
assert(selectionDraft.visibleRow(for: "bb", matching: "") == nil)
selectionDraft.remove(at: 0)
assert(selectionDraft.visibleRow(for: "cc", matching: "") == 0)
print("selection tests passed: filtered row, hidden selection, renamed code, shifted index, no-op edit")

// 仅供集成测试：向调用方新建的临时目录输出固定虚构配置。
if CommandLine.arguments.count == 3 && CommandLine.arguments[1] == "--integration-output" {
    let output = URL(fileURLWithPath: CommandLine.arguments[2], isDirectory: true)
    var fixture = SettingsModel()
    fixture.terms = [UserTerm(code: "ceshiya", text: "虚构验收词", gloss: "fictional acceptance term · complete")]
    try fixture.save(to: output, expected: nil, checkConflict: true)
}
