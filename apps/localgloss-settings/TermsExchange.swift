//! 仅交换主动词条；严格解析、全批合并与受限本地文件操作。
import Foundation
import Darwin

struct TermsExchange: Codable {
    var format = "localgloss-terms"
    var version = 1
    var terms: [UserTerm]
    static let maxBytes = 262144

    func encoded() throws -> Data {
        guard format == "localgloss-terms", version == 1 else {
            throw ValidationError.invalid("词条交换格式或版本不受支持。")
        }
        var codes = Set<String>()
        for (index, term) in terms.enumerated() {
            guard term.fieldErrors.allSatisfy({ $0.isEmpty }) else {
                throw ValidationError.invalid("第 \(index + 1) 项字段无效，整批未导入。")
            }
            guard codes.insert(term.code).inserted else {
                throw ValidationError.invalid("第 \(index + 1) 项输入码重复，整批未导入。")
            }
        }
        var model = SettingsModel(); model.terms = terms
        try model.validate()
        let encoder = JSONEncoder(); encoder.outputFormatting = [.prettyPrinted, .sortedKeys]
        let data = try encoder.encode(self)
        guard data.count <= Self.maxBytes else { throw ValidationError.invalid("词条文件超过 256 KiB。") }
        return data
    }

    static func decode(_ data: Data) throws -> Self {
        guard data.count <= maxBytes else { throw ValidationError.invalid("词条文件超过 256 KiB。") }
        guard String(data: data, encoding: .utf8) != nil else { throw ValidationError.invalid("词条文件必须使用 UTF-8 编码。") }
        guard let object = try JSONSerialization.jsonObject(with: data) as? [String: Any],
              Set(object.keys) == Set(["format", "version", "terms"]),
              let rows = object["terms"] as? [[String: Any]],
              rows.allSatisfy({ Set($0.keys) == Set(["code", "text", "gloss"]) }) else {
            throw ValidationError.invalid("词条文件字段不受支持，未导入。")
        }
        let result = try JSONDecoder().decode(Self.self, from: data)
        _ = try result.encoded()
        return result
    }

    func merged(with model: SettingsModel, replacing codes: Set<String>) throws -> SettingsModel {
        _ = try encoded()
        var next = model
        for term in terms {
            if let index = next.terms.firstIndex(where: { $0.code == term.code }) {
                if codes.contains(term.code) { next.terms[index] = term }
            } else { next.terms.append(term) }
        }
        try next.validate()
        let encoder = JSONEncoder(); encoder.outputFormatting = [.prettyPrinted, .sortedKeys]
        guard try encoder.encode(next).count <= Self.maxBytes else {
            throw ValidationError.invalid("合并后的设置超过 256 KiB，草稿保持不变。")
        }
        return next
    }

    // 逐段以目录描述符打开，拒绝任一父级符号链接；不扫描目录。
    private static func parent(of url: URL) throws -> (Int32, String) {
        guard url.isFileURL else { throw ValidationError.invalid("只支持本地普通文件。") }
        let parts = url.pathComponents.dropFirst()
        guard let name = parts.last, !name.isEmpty,
              !parts.contains(".."), !parts.contains(".") else { throw ValidationError.invalid("请选择不含相对跳转的文件路径。") }
        var fd = open("/", O_RDONLY | O_DIRECTORY | O_NOFOLLOW)
        guard fd >= 0 else { throw ValidationError.invalid("无法打开文件目录。") }
        for part in parts.dropLast() {
            let next = openat(fd, part, O_RDONLY | O_DIRECTORY | O_NOFOLLOW)
            close(fd)
            guard next >= 0 else { throw ValidationError.invalid("文件目录不可用或包含符号链接。") }
            fd = next
        }
        return (fd, name)
    }

    static func read(from url: URL) throws -> Self {
        let (directory, name) = try parent(of: url); defer { close(directory) }
        let fd = openat(directory, name, O_RDONLY | O_NOFOLLOW | O_NONBLOCK)
        guard fd >= 0 else { throw ValidationError.invalid("无法读取词条文件，未导入。") }
        defer { close(fd) }
        var info = stat()
        guard fstat(fd, &info) == 0, (info.st_mode & S_IFMT) == S_IFREG,
              info.st_size >= 0, info.st_size <= Int64(maxBytes) else {
            throw ValidationError.invalid("只支持不超过 256 KiB 的普通文件。")
        }
        var data = Data(); var buffer = [UInt8](repeating: 0, count: 8192)
        while true {
            let count = Darwin.read(fd, &buffer, buffer.count)
            if count < 0 && errno == EINTR { continue }
            guard count >= 0 else { throw ValidationError.invalid("读取失败，未导入。") }
            if count == 0 { break }
            guard data.count + count <= maxBytes else { throw ValidationError.invalid("读取期间文件超限，未导入。") }
            data.append(contentsOf: buffer.prefix(count))
        }
        return try decode(data)
    }

    // 先写同目录 0600 临时文件，再用 linkat 原子创建目标；已有目标始终保留。
    func exportNew(to url: URL) throws {
        let data = try encoded()
        let (directory, name) = try Self.parent(of: url); defer { close(directory) }
        let temporary = ".localgloss-terms-" + UUID().uuidString
        let fd = openat(directory, temporary, O_WRONLY | O_CREAT | O_EXCL | O_NOFOLLOW, 0o600)
        guard fd >= 0 else { throw ValidationError.invalid("无法创建导出文件。") }
        defer { close(fd); unlinkat(directory, temporary, 0) }
        guard fchmod(fd, 0o600) == 0 else { throw ValidationError.invalid("无法设置导出权限，未导出。") }
        try data.withUnsafeBytes { bytes in
            var offset = 0
            while offset < bytes.count {
                let count = Darwin.write(fd, bytes.baseAddress!.advanced(by: offset), bytes.count - offset)
                if count < 0 && errno == EINTR { continue }
                guard count > 0 else { throw ValidationError.invalid("写入失败，未导出。") }
                offset += count
            }
        }
        guard fsync(fd) == 0 else { throw ValidationError.invalid("无法完成导出写入。") }
        guard linkat(directory, temporary, directory, name, 0) == 0 else {
            throw ValidationError.invalid("目标已存在或无法创建。请选择新文件名，原文件保持不变。")
        }
    }
}
