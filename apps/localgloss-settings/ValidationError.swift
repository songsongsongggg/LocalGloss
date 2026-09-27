//! 设置校验错误只描述规则，不回显用户词条。
import Foundation

enum ValidationError: LocalizedError {
    case invalid(String)
    var errorDescription: String? { if case .invalid(let message) = self { return message }; return nil }
}
