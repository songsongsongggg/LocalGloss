//! 设置与唯一的落盘入口；只在用户点击保存时写入。
import Foundation

struct SettingsModel: Codable {
    var version = 1
    var font_size = 16
    var page_size = 9
    var tab_translation = true
    var symbol_paging = true
    var chinese_punctuation = false
    var terms: [UserTerm] = []

    static var directory: URL {
        FileManager.default.homeDirectoryForCurrentUser.appendingPathComponent("Library/Application Support/LocalGloss", isDirectory: true)
    }
    static var file: URL { directory.appendingPathComponent("settings.json") }

    func validate() throws {
        guard version == 1, (14...22).contains(font_size), (3...9).contains(page_size), terms.count <= 500 else {
            throw ValidationError.invalid("设置版本或数值不正确；最多 500 个词条。")
        }
        var used = Set<String>()
        for term in terms {
            guard !term.code.isEmpty, term.code.utf8.count <= 32,
                  term.code.utf8.allSatisfy({ (97...122).contains($0) }),
                  !term.text.contains("|"),
                  !term.text.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty,
                  term.text.unicodeScalars.count <= 64, term.gloss.unicodeScalars.count <= 240,
                  !term.text.unicodeScalars.contains(where: CharacterSet.controlCharacters.contains),
                  !term.gloss.unicodeScalars.contains(where: CharacterSet.controlCharacters.contains),
                  used.insert(term.code).inserted else {
                throw ValidationError.invalid("输入码须为 1–32 个小写字母且不能重复；词条最多 64 字、译词最多 240 字，均为单行。")
            }
        }
    }

    static func load(from file: URL = Self.file) throws -> SettingsModel {
        guard FileManager.default.fileExists(atPath: file.path) else { return SettingsModel() }
        let size = try FileManager.default.attributesOfItem(atPath: file.path)[.size] as? NSNumber
        guard let size, size.intValue <= 262144 else { throw ValidationError.invalid("设置文件过大，未加载。") }
        let data = try Data(contentsOf: file)
        guard let object = try JSONSerialization.jsonObject(with: data) as? [String: Any],
              Set(object.keys) == Set(["version", "font_size", "page_size", "tab_translation", "symbol_paging", "chinese_punctuation", "terms"]),
              let terms = object["terms"] as? [[String: Any]],
              terms.allSatisfy({ Set($0.keys).isSubset(of: ["code", "text", "gloss"]) }) else {
            throw ValidationError.invalid("设置格式不受支持，未加载。")
        }
        let result = try JSONDecoder().decode(Self.self, from: data)
        try result.validate()
        return result
    }

    func save(to directory: URL = Self.directory) throws {
        let file = directory.appendingPathComponent("settings.json")
        try validate()
        let manager = FileManager.default
        for url in [directory, file] {
            if let attributes = try? manager.attributesOfItem(atPath: url.path), attributes[.type] as? FileAttributeType == .typeSymbolicLink {
                throw ValidationError.invalid("设置路径是符号链接，未写入。")
            }
        }
        try manager.createDirectory(at: directory, withIntermediateDirectories: true, attributes: [.posixPermissions: 0o700])
        let encoder = JSONEncoder()
        encoder.outputFormatting = [.prettyPrinted, .sortedKeys]
        let data = try encoder.encode(self)
        guard data.count <= 262144 else { throw ValidationError.invalid("设置文件过大，未保存。") }
        try data.write(to: file, options: .atomic)
        try manager.setAttributes([.posixPermissions: 0o600], ofItemAtPath: file.path)
    }
}
