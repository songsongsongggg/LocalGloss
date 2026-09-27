//! 用户主动填写的词条；不从输入法会话获取。
import Foundation

struct UserTerm: Codable {
    var code: String
    var text: String
    var gloss: String
}
