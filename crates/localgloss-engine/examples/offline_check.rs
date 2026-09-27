//! 仅使用虚构测试拼音，检查完整随包词表在禁止联网和写文件的环境中可用。
use localgloss_engine::{Key, OfflineEngine};

fn main() {
    let resources = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets");
    let started = std::time::Instant::now();
    let mut engine = OfflineEngine::from_paths(
        &resources.join("lexicon/dict.tsv"),
        &resources.join("glossary/glossary-en.tsv"),
    )
    .expect("bundled dictionaries must load");
    let loaded = started.elapsed();
    for letter in "kaifa".chars() {
        engine.handle(Key::Letter(letter));
    }
    let frame = engine.frame();
    let digit = frame
        .rows
        .iter()
        .position(|row| row.text == "开发")
        .expect("fixture candidate")
        + 1;
    let result = engine.handle(Key::Translation {
        digit,
        second: false,
    });
    assert!(
        result
            .commit
            .as_deref()
            .is_some_and(|text| !text.is_empty())
    );
    assert!(result.frame.preedit.is_empty());
    for letter in "kafei".chars() {
        engine.handle(Key::Letter(letter));
    }
    engine.set_blocked(true);
    assert!(!engine.handle(Key::Letter('a')).handled);
    assert!(engine.frame().preedit.is_empty());
    engine.set_blocked(false);
    assert!(engine.finish_raw().is_none());
    println!(
        "bundled offline flow passed; load={}ms; flow={}ms",
        loaded.as_millis(),
        started.elapsed().as_millis()
    );
}
