//! 平台无关的输入操作，不接收剪贴板、正文或应用上下文。
#[derive(Clone, Copy)]
pub enum Key {
    Letter(char),
    Digit(usize),
    Translation { digit: usize, second: bool },
    HighlightedTranslation { second: bool },
    Literal(char),
    ToggleEnglish,
    ToggleDetails,
    Backspace,
    Space,
    Enter,
    Escape,
    Previous,
    Next,
    PagePrevious,
    PageNext,
    Left,
    Right,
    PassThrough,
}
