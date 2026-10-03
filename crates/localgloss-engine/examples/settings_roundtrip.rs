//! 读取 Swift 测试生成的固定虚构配置，验证跨语言保存和译词上屏契约。
use localgloss_engine::{Key, OfflineEngine, Settings};

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    assert_eq!(
        args.len(),
        3,
        "expected fixture file and dictionary directory"
    );
    let bytes = std::fs::read(&args[1]).expect("read generated fixture");
    let mut settings = Settings::parse(&bytes).expect("parse Swift settings");
    assert_eq!(settings.terms.len(), 1);
    assert_eq!(settings.terms[0].code, "ceshiya");
    let resources = std::path::Path::new(&args[2]);
    let mut engine = OfflineEngine::from_paths(
        &resources.join("dict.tsv"),
        &resources.join("glossary-en.tsv"),
    )
    .expect("load fixture dictionaries");
    engine
        .apply_settings(settings.clone())
        .expect("apply settings");
    for letter in "ceshiya".chars() {
        engine.handle(Key::Letter(letter));
    }
    assert_eq!(engine.frame().rows[0].text, "虚构验收词");
    assert_eq!(
        engine.handle(Key::Space).commit.as_deref(),
        Some("虚构验收词")
    );
    for letter in "ceshiya".chars() {
        engine.handle(Key::Letter(letter));
    }
    assert_eq!(
        engine
            .handle(Key::HighlightedTranslation { second: false })
            .commit
            .as_deref(),
        Some("fictional acceptance term · complete")
    );
    settings.tab_translation = false;
    engine
        .apply_settings(settings)
        .expect("disable Tab translation");
    // 实际 Tab 键由 macOS controller 根据此值映射，本例仅验证配置传递。
    assert!(!engine.settings().tab_translation);
    println!(
        "Swift save -> Rust parse -> Chinese/translation commit and option propagation passed"
    );
}
