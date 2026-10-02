//! 使用虚构候选检查原生排版并导出预览；不注册输入法或接收键盘输入。
#[path = "../src/layout.rs"]
mod layout;
#[path = "../src/view.rs"]
mod view;

use localgloss_engine::{Frame, Row};
use objc2::MainThreadMarker;
use objc2_app_kit::NSApplication;
use objc2_foundation::NSSize;

fn main() {
    let mtm = MainThreadMarker::new().expect("main thread");
    NSApplication::sharedApplication(mtm);
    let view = view::CandidateView::new(mtm);
    view.configure(localgloss_engine::Settings::default());
    let frame = Frame {
        preedit: "kaifa".into(),
        cursor: 5,
        rows: vec![
            Row {
                text: "开发".into(),
                gloss: "develop · exploit".into(),
                primary_gloss: "develop".into(),
            },
            Row {
                text: "开放".into(),
                gloss: "open · open up".into(),
                primary_gloss: "open".into(),
            },
            Row {
                text: "开发者".into(),
                gloss: "developer".into(),
                primary_gloss: "developer".into(),
            },
            Row {
                text: "测试词".into(),
                gloss: String::new(),
                primary_gloss: String::new(),
            },
        ],
        highlighted: 0,
        page: 0,
        pages: 2,
        ..Frame::default()
    };
    let normal = view.set_frame(frame.clone(), 1440.0);
    let output = std::env::args().nth(1).expect("preview output directory");
    let output = std::path::Path::new(&output);
    std::fs::create_dir_all(output).expect("preview directory");
    std::fs::write(
        output.join("normal.pdf"),
        view.dataWithPDFInsideRect(view.bounds()).to_vec(),
    )
    .expect("preview");
    let mut long = frame.clone();
    long.rows[0] = Row { text: "较长中文候选显示检查".into(), gloss: "a deliberately long translation used only to verify that the two columns remain separate and show an ellipsis instead of drawing outside the panel".into(), primary_gloss: "a deliberately long first translation that must remain complete when committed and only be shortened visually".into() };
    let expanded = view.set_frame(long.clone(), 1440.0);
    assert!(expanded.width >= normal.width && expanded.width <= 560.0);
    std::fs::write(
        output.join("long.pdf"),
        view.dataWithPDFInsideRect(view.bounds()).to_vec(),
    )
    .expect("preview");
    assert_eq!(view.set_frame(frame.clone(), 1440.0).width, expanded.width);
    let narrow = view.set_frame(long, 320.0);
    assert_eq!(narrow.width, 320.0);
    std::fs::write(
        output.join("narrow.pdf"),
        view.dataWithPDFInsideRect(view.bounds()).to_vec(),
    )
    .expect("preview");
    view.clear();
    assert_eq!(view.frame().size, NSSize::ZERO);
    assert_eq!(view.set_frame(frame.clone(), 1440.0), normal);
    view.configure(localgloss_engine::Settings {
        font_size: 22,
        ..Default::default()
    });
    let detail_frame = Frame {
        detail: Some(
            "开发\ndevelop: build or improve something. 完整释义按页展示，不截断实际提交的词条。"
                .repeat(3),
        ),
        ..frame
    };
    view.set_frame(detail_frame, 1440.0);
    std::fs::write(
        output.join("details.pdf"),
        view.dataWithPDFInsideRect(view.bounds()).to_vec(),
    )
    .expect("preview");
    println!("candidate layout checks passed; fictional previews exported");
}
