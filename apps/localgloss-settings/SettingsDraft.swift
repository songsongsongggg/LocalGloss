//! 内存草稿与过滤索引；不访问输入会话、不自动保存。
import Foundation

struct SettingsDraft {
    var model: SettingsModel
    private(set) var original: SettingsModel
    private var undoTerms: [[UserTerm]] = []

    init(_ model: SettingsModel = SettingsModel()) {
        self.model = model
        original = model
    }

    var isDirty: Bool { model != original }
    var canUndo: Bool { !undoTerms.isEmpty }

    func indices(matching query: String) -> [Int] {
        model.terms.indices.filter { index in
            let term = model.terms[index]
            return query.isEmpty || [term.code, term.text, term.gloss].contains {
                $0.localizedCaseInsensitiveContains(query)
            }
        }
    }

    mutating func put(_ term: UserTerm, at index: Int?) throws {
        var next = model
        if let index {
            guard next.terms.indices.contains(index) else { throw ValidationError.invalid("选中词条已变化，请重新选择。") }
            next.terms[index] = term
        } else { next.terms.append(term) }
        try next.validate()
        if undoTerms.count >= 100 { undoTerms.removeFirst() }
        undoTerms.append(model.terms)
        model = next
    }

    mutating func remove(at index: Int) {
        guard model.terms.indices.contains(index) else { return }
        if undoTerms.count >= 100 { undoTerms.removeFirst() }
        undoTerms.append(model.terms)
        model.terms.remove(at: index)
    }

    mutating func undo() {
        if let previous = undoTerms.popLast() { model.terms = previous }
    }
}
