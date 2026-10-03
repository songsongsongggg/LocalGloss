//! 译词快捷键、末页与翻页边界回归测试。
use super::{engine, type_pinyin};
use crate::{Key, OfflineEngine};

#[test]
fn highlighted_translation_follows_selection_and_preserves_focus_when_idle() {
    let mut engine = engine();
    assert!(
        !engine
            .handle(Key::HighlightedTranslation { second: false })
            .handled
    );
    type_pinyin(&mut engine, "kaifa");
    assert_eq!(
        engine
            .handle(Key::HighlightedTranslation { second: true })
            .commit
            .as_deref(),
        Some("exploit")
    );
    type_pinyin(&mut engine, "kaifa");
    let position = engine
        .frame()
        .rows
        .iter()
        .position(|row| row.text == "开")
        .unwrap();
    for _ in 0..position {
        engine.handle(Key::Next);
    }
    assert_eq!(
        engine
            .handle(Key::HighlightedTranslation { second: false })
            .commit
            .as_deref(),
        Some("open")
    );
    assert!(engine.engine.history().is_empty());
    assert_eq!(engine.frame().preedit, "fa");
    engine.handle(Key::Escape);
    type_pinyin(&mut engine, "kafei");
    let result = engine.handle(Key::HighlightedTranslation { second: false });
    assert!(result.handled);
    assert!(result.commit.is_none());
    assert_eq!(result.frame.preedit, "kafei");
    engine.set_blocked(true);
    assert!(
        !engine
            .handle(Key::HighlightedTranslation { second: false })
            .handled
    );
}

#[test]
fn page_navigation_selects_first_row_even_on_a_short_last_page() {
    let dictionary = (0..20)
        .map(|index| format!("词{index}\tci\t{}\n", 1000 - index))
        .collect::<String>();
    let mut engine = OfflineEngine::from_tsv(&dictionary, "").unwrap();
    type_pinyin(&mut engine, "ci");
    assert_eq!(engine.frame().pages, 3);
    assert!(
        engine
            .frame()
            .rows
            .iter()
            .any(|row| row.raw && row.text == "ci")
    );
    engine.handle(Key::Next);
    let second = engine.handle(Key::PageNext).frame;
    assert_eq!((second.page, second.highlighted), (1, 0));
    let last = engine.handle(Key::PageNext).frame;
    assert_eq!((last.page, last.highlighted, last.rows.len()), (2, 0, 3));
    let bounded = engine.handle(Key::PageNext).frame;
    assert_eq!((bounded.page, bounded.highlighted), (2, 0));
    let previous = engine.handle(Key::PagePrevious).frame;
    assert_eq!((previous.page, previous.highlighted), (1, 0));
    engine.handle(Key::PagePrevious);
    let first = engine.handle(Key::PagePrevious).frame;
    assert_eq!((first.page, first.highlighted), (0, 0));
    assert_eq!(engine.handle(Key::Space).commit.as_deref(), Some("词0"));
}
