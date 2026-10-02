//! 用户主动填写的词条；不从输入法会话获取。
import Foundation

struct UserTerm: Codable, Equatable {
    var code: String
    var text: String
    var gloss: String
}

extension UserTerm {
    var fieldErrors: [String] {
        let codeError = code.isEmpty || code.utf8.count > 32 || !code.utf8.allSatisfy({ (97...122).contains($0) })
        let textError = text.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty || text.contains("|") || text.unicodeScalars.count > 64 || text.unicodeScalars.contains(where: CharacterSet.controlCharacters.contains)
        let glossError = gloss.unicodeScalars.count > 240 || gloss.unicodeScalars.contains(where: CharacterSet.controlCharacters.contains)
        return [codeError ? "须为 1–32 个小写 ASCII 字母。" : "",
                textError ? "须为 1–64 个 Unicode 标量，不含竖线或控制字符。" : "",
                glossError ? "最多 240 个 Unicode 标量，不能含控制字符。" : ""]
    }
}
