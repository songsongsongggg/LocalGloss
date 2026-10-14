//! 用随包公开词表验证高亮译词提交，不读取个人配置或输入历史。
use localgloss_engine::{Key, OfflineEngine};

fn main() {
    let resources = std::path::PathBuf::from(
        std::env::args_os()
            .nth(1)
            .expect("resource directory required"),
    );
    let mut engine = OfflineEngine::from_paths(
        &resources.join("dict.tsv"),
        &resources.join("glossary-en.tsv"),
    )
    .expect("bundled dictionaries must load");
    for _ in 0..3 {
        for letter in "kafei".chars() {
            engine.handle(Key::Letter(letter));
        }
        assert_eq!(engine.frame().rows[0].text, "咖啡");
        assert_eq!(engine.frame().rows[0].primary_gloss, "(loanword) coffee");
        let result = engine.handle(Key::HighlightedTranslation { second: false });
        assert!(result.handled);
        assert_eq!(result.commit.as_deref(), Some("(loanword) coffee"));
        assert!(result.frame.preedit.is_empty());
    }
    for letter in "wode".chars() {
        engine.handle(Key::Letter(letter));
    }
    let missing = engine.handle(Key::HighlightedTranslation { second: false });
    assert!(missing.handled);
    assert!(missing.commit.is_none());
    assert_eq!(missing.frame.preedit, "wode");
    assert_eq!(engine.handle(Key::Space).commit.as_deref(), Some("我的"));
    assert!(
        !engine
            .handle(Key::HighlightedTranslation { second: false })
            .handled
    );
    for letter in "kafei".chars() {
        engine.handle(Key::Letter(letter));
    }
    engine.set_blocked(true);
    let blocked = engine.handle(Key::HighlightedTranslation { second: false });
    assert!(!blocked.handled);
    assert!(blocked.commit.is_none());
    assert!(blocked.frame.preedit.is_empty());
    println!(
        "bundled highlighted translation passed: coffee x3, missing gloss, idle Tab, secure input"
    );
}
