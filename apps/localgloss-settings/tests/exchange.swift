//! 固定虚构词条覆盖全批校验、冲突合并、撤销和新文件导出。
import Foundation
import Darwin

func testExchange() throws {
    guard let resolved = realpath(NSTemporaryDirectory(), nil) else { fatalError("temporary directory unavailable") }
    defer { free(resolved) }
    let directory = URL(fileURLWithPath: String(cString: resolved), isDirectory: true)
        .appendingPathComponent("localgloss-exchange-" + UUID().uuidString)
    try FileManager.default.createDirectory(at: directory, withIntermediateDirectories: false)
    let old = UserTerm(code: "aa", text: "甲", gloss: "first")
    let changed = UserTerm(code: "aa", text: "新甲", gloss: "updated")
    let added = UserTerm(code: "bb", text: "乙", gloss: "")
    var model = SettingsModel(); model.terms = [old]; model.font_size = 22
    let exchange = TermsExchange(terms: [changed, added])
    var draft = SettingsDraft(model)
    try draft.merge(exchange, replacing: [])
    assert(draft.model.terms == [old, added] && draft.model.font_size == 22)
    draft.undo(); assert(draft.model == model)
    try draft.merge(exchange, replacing: ["aa"])
    assert(draft.model.terms == [changed, added])
    draft.undo(); assert(!draft.isDirty)
    try draft.merge(TermsExchange(terms: [old]), replacing: [])
    assert(!draft.isDirty && !draft.canUndo)
    let snapshot = draft.model
    for terms in [[old, old], [UserTerm(code: "A", text: "坏", gloss: "")]] {
        do { try draft.merge(TermsExchange(terms: terms), replacing: []); fatalError("invalid batch accepted") } catch {}
        assert(draft.model == snapshot && !draft.canUndo)
    }
    let json = try exchange.encoded()
    let decoded = try TermsExchange.decode(json)
    assert(decoded.terms == exchange.terms)
    let object = try JSONSerialization.jsonObject(with: json) as! [String: Any]
    assert(Set(object.keys) == Set(["format", "version", "terms"]))
    var invalid = object
    invalid["terms"] = [["code": "a", "text": "词", "gloss": "", "unknown": true]]
    do { _ = try TermsExchange.decode(JSONSerialization.data(withJSONObject: invalid)); fatalError("unknown term field accepted") } catch {}
    let utf16 = String(data: json, encoding: .utf8)!.data(using: .utf16)!
    do { _ = try TermsExchange.decode(utf16); fatalError("non-UTF8 accepted") } catch {}
    var full = SettingsModel()
    full.terms = (0..<500).map { i in
        UserTerm(code: String(UnicodeScalar(97 + i / 26)!) + String(UnicodeScalar(97 + i % 26)!), text: "虚构", gloss: "")
    }
    var fullDraft = SettingsDraft(full)
    do { try fullDraft.merge(TermsExchange(terms: [UserTerm(code: "zzzz", text: "超限", gloss: "")]), replacing: []); fatalError("501 terms accepted") } catch {}
    assert(fullDraft.model == full && !fullDraft.canUndo)
    let tooLarge = full.terms.map { UserTerm(code: $0.code, text: String(repeating: "词", count: 64), gloss: String(repeating: "译", count: 240)) }
    do { _ = try TermsExchange(terms: tooLarge).encoded(); fatalError("oversize encoding accepted") } catch {}
    for bad in [Data(repeating: 32, count: 262145), Data("{\"format\":\"localgloss-terms\",\"version\":2,\"terms\":[]}".utf8), Data("{\"format\":\"localgloss-terms\",\"version\":1,\"terms\":[],\"extra\":true}".utf8)] {
        do { _ = try TermsExchange.decode(bad); fatalError("invalid document accepted") } catch {}
    }
    let file = directory.appendingPathComponent("terms.json")
    try exchange.exportNew(to: file)
    let reread = try TermsExchange.read(from: file)
    assert(reread.terms == exchange.terms)
    let permissions = try FileManager.default.attributesOfItem(atPath: file.path)[.posixPermissions] as! NSNumber
    assert(permissions.intValue == 0o600)
    let before = try Data(contentsOf: file)
    do { try TermsExchange(terms: []).exportNew(to: file); fatalError("existing target overwritten") } catch {}
    let after = try Data(contentsOf: file)
    assert(after == before)
    let link = directory.appendingPathComponent("link.json")
    try FileManager.default.createSymbolicLink(at: link, withDestinationURL: file)
    do { _ = try TermsExchange.read(from: link); fatalError("symlink read") } catch {}
    do { try exchange.exportNew(to: link); fatalError("symlink export") } catch {}
    let parentLink = directory.appendingPathComponent("linked-directory")
    try FileManager.default.createSymbolicLink(at: parentLink, withDestinationURL: directory)
    do { _ = try TermsExchange.read(from: parentLink.appendingPathComponent("terms.json")); fatalError("parent symlink read") } catch {}
    do { try exchange.exportNew(to: parentLink.appendingPathComponent("new.json")); fatalError("parent symlink export") } catch {}
    do { _ = try TermsExchange.read(from: directory); fatalError("directory imported") } catch {}
    let pipe = directory.appendingPathComponent("pipe")
    assert(mkfifo(pipe.path, 0o600) == 0)
    do { _ = try TermsExchange.read(from: pipe); fatalError("FIFO imported") } catch {}
    do { try exchange.exportNew(to: directory.appendingPathComponent("absent/file.json")); fatalError("missing directory accepted") } catch {}
    let remaining = try FileManager.default.contentsOfDirectory(atPath: directory.path)
    assert(remaining.allSatisfy { !$0.hasPrefix(".localgloss-terms-") })
    print("terms exchange tests passed: validation, merge, undo, round trip, 0600, no overwrite, symlinks, failure cleanup")
}
