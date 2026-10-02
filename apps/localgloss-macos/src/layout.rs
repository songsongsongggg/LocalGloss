//! 候选窗宽度约束；从宽屏移到窄屏时允许缩窄，不越过可用显示宽度。
pub fn panel_width(desired: f64, previous: f64, available: f64) -> f64 {
    let limit = if available.is_finite() && available > 0.0 {
        available.min(560.0)
    } else {
        560.0
    };
    desired.max(420.0).max(previous).min(limit)
}

pub fn column_width(measured: f64, panel: f64) -> f64 {
    // 中文栏至多占 40%，为英文栏保留空间。
    (measured.clamp(48.0, 150.0) + 52.0).min(panel * 0.4)
}

#[cfg(test)]
mod tests {
    use super::{column_width, panel_width};

    #[test]
    fn moving_to_narrow_screen_overrides_retained_width() {
        assert_eq!(panel_width(900.0, 0.0, 1440.0), 560.0);
        assert_eq!(panel_width(420.0, 560.0, 320.0), 320.0);
        assert_eq!(panel_width(420.0, 500.0, 1440.0), 500.0);
        assert!(column_width(500.0, 320.0) <= 128.0);
        assert_eq!(panel_width(420.0, 0.0, f64::NAN), 420.0);
    }
}
