// SPDX-License-Identifier: GPL-3.0-only
//! 界面字体（SYS-3 / ADR 0041）：复用 ScreenTrans 的 Segoe UI → Microsoft YaHei → 系统回退。

use std::sync::{Arc, OnceLock};

use windows::Win32::Graphics::DirectWrite::{
    DWRITE_UNICODE_RANGE, IDWriteFactory2, IDWriteFontFallback, IDWriteTextFormat, IDWriteTextFormat1,
};
use windows::core::{Interface, PCWSTR, w};

use super::text::{FontSpec, factory};

const UI_FAMILY: &str = "Segoe UI";

impl FontSpec {
    /// 系统界面字体；标注对象继续使用自己的字体规格。
    pub fn ui(size: f32) -> Self {
        Self {
            family: Arc::from(UI_FAMILY),
            size,
            bold: false,
            italic: false,
        }
    }
}

/// 为缺失字形优先选微软雅黑；未覆盖的字符继续走系统回退。
fn fallback() -> Option<&'static IDWriteFontFallback> {
    static FALLBACK: OnceLock<Option<IDWriteFontFallback>> = OnceLock::new();
    FALLBACK
        .get_or_init(|| unsafe {
            let factory: IDWriteFactory2 = factory()?.cast().ok()?;
            let builder = factory.CreateFontFallbackBuilder().ok()?;
            builder
                .AddMapping(
                    &[DWRITE_UNICODE_RANGE {
                        first: 0,
                        last: 0x10FFFF,
                    }],
                    &[w!("Microsoft YaHei").as_ptr()],
                    None,
                    PCWSTR::null(),
                    PCWSTR::null(),
                    1.0,
                )
                .ok()?;
            builder.AddMappings(&factory.GetSystemFontFallback().ok()?).ok()?;
            builder.CreateFontFallback().ok()
        })
        .as_ref()
}

/// 格式同时用于测量与绘制，避免显示字体与布局计算不一致。
pub(super) fn apply(format: &IDWriteTextFormat, spec: &FontSpec) {
    if spec.family.as_ref() == UI_FAMILY
        && let Some(fallback) = fallback()
        && let Ok(format) = format.cast::<IDWriteTextFormat1>()
    {
        let _ = unsafe { format.SetFontFallback(fallback) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::text::measure;

    #[test]
    fn ui_chinese_follows_yahei_without_changing_annotation_font() {
        assert!(fallback().is_some(), "系统应能创建界面字体回退");
        let ui = FontSpec::ui(16.0);
        let yahei = FontSpec::from_parts(&Arc::from("Microsoft YaHei"), 16.0, false, false);
        let text = "通用工具栏快捷键保存外观";
        let (ui_width, _) = measure(text, &ui);
        let (yahei_width, _) = measure(text, &yahei);
        assert!(
            (ui_width - yahei_width).abs() < 0.1,
            "中文应使用微软雅黑：{ui_width} vs {yahei_width}"
        );
        assert_eq!(FontSpec::new(16.0).family.as_ref(), crate::editor::DEFAULT_FONT_FAMILY);
    }
}
