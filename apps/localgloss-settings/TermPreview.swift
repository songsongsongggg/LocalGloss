//! 草稿词条的只读说明；遵循当前 Tab 设置，不触发输入或文件访问。
import Foundation

struct TermPreview {
    static func text(for term: UserTerm, tabTranslation: Bool) -> String {
        let translation: String
        if !tabTranslation {
            translation = "译词快捷键已关闭；以目标应用的 Tab 行为为准"
        } else if term.gloss.isEmpty {
            translation = "未指定；是否有译词以本地候选为准"
        } else {
            translation = term.gloss
        }
        return "只读预览（中文模式下选中此词条；不向输入窗口提交）：\nSpace：\(term.text)\nTab：\(translation)"
    }
}
