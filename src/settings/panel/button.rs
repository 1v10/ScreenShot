// SPDX-License-Identifier: GPL-3.0-only
//! 快捷键页恢复默认按钮的绘制与命中几何。

use tiny_skia::{Pixmap, Transform};

use crate::editor::Point;
use crate::overlay::geometry::SelRect;
use crate::overlay::shape::rounded_rect;
use crate::render::text as dwrite;

use super::Panel;

/// 按钮几何（纯计算，便于测试）：高度与快捷键行的键帽一致（30px 基准），宽度按文字量算，
/// 右对齐、不超出控件区。
fn btn_geom(control: SelRect, text_w: f32, s: f32) -> SelRect {
    let pad = 16.0 * s;
    let bh = (30.0 * s) as i32;
    let bw = ((text_w + pad * 2.0) as i32).clamp((90.0 * s) as i32, control.w);
    SelRect {
        x: control.x + control.w - bw,
        y: control.y + (control.h - bh) / 2,
        w: bw,
        h: bh,
    }
}

/// 按钮行的按钮矩形：绘制与鼠标命中共用这一份几何，保证「看得见的边界 == 点得中的边界」。
/// 命中测试必须用它——曾经整行可点，按钮外一点就触发（比按钮自身宽得多）。
pub(super) fn button_rect(panel: &Panel, row: &super::controls::Row) -> SelRect {
    button_geom(panel, row).0
}

/// 按钮矩形 + 文案宽度（绘制需要宽度来居中文字，不重复测量）。
fn button_geom(panel: &Panel, row: &super::controls::Row) -> (SelRect, f32) {
    let s = panel.scale;
    let text = "恢复默认";
    let (tw, _) = dwrite::measure(text, &dwrite::FontSpec::ui(15.0 * s));
    (btn_geom(row.control, tw, s), tw)
}

/// 按钮风格行：真正的按钮外观（圆角底 + 边框 + 居中文字）。
/// 「恢复默认」使用固定文案。
/// `hovered`：鼠标悬停在按钮上时给强调底色，明确「可点」（悬停在行内空白处不算）。
pub(super) fn draw_button(pixmap: &mut Pixmap, panel: &Panel, row: &super::controls::Row, hovered: bool) {
    let p = panel.palette;
    let s = panel.scale;
    let text = "恢复默认";
    let (btn, tw) = button_geom(panel, row);
    let size = 15.0 * s;
    let spec = dwrite::FontSpec::ui(size);

    let (fill, border, fg) = if hovered {
        (p.accent_soft, p.accent, p.accent)
    } else {
        ([p.bg[0], p.bg[1], p.bg[2], 255], p.border, p.text)
    };
    rounded_rect(pixmap, btn, (6.0 * s) as f32, fill, Some(border));

    let x = btn.x as f32 + (btn.w as f32 - tw) / 2.0;
    let y = btn.y as f32 + (btn.h as f32 - size * 1.2) / 2.0;
    dwrite::draw(pixmap, Point::new(x, y), text, &spec, fg, Transform::identity());
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 按钮矩形不占满行（否则会出现「按钮外一点就触发」）：右对齐、比行矮、比控件区窄。
    #[test]
    fn button_rect_stays_inside_control() {
        // 标准行的控件区：260 宽 × 46 高，起点在行右侧
        let c = SelRect {
            x: 400,
            y: 0,
            w: 260,
            h: 46,
        };
        let r = btn_geom(c, 60.0, 1.0);
        assert_eq!(r.x + r.w, c.x + c.w, "按钮应与控件区右对齐");
        assert!(r.x > c.x, "按钮左缘不应贴到控件区左缘：{r:?}");
        assert!(r.h < c.h, "按钮应比行矮：{r:?}");
        // 文案极长时钳到控件区宽度（仍不越出控件区、更不占满整行）
        let wide = btn_geom(c, 999.0, 1.0);
        assert_eq!(wide.w, c.w);
        assert_eq!(wide.x + wide.w, c.x + c.w);
        // 高 DPI 下同样成立（控件区几何也跟着缩放）
        let hi_c = SelRect {
            x: 800,
            y: 0,
            w: 520,
            h: 92,
        };
        let hi = btn_geom(hi_c, 120.0, 2.0);
        assert_eq!(hi.x + hi.w, hi_c.x + hi_c.w);
        assert!(hi.x > hi_c.x && hi.h < hi_c.h);
    }

    /// 恢复默认文案应能装进控件区。
    #[test]
    fn button_text_fits_control_width() {
        let (w, _) = dwrite::measure("恢复默认", &dwrite::FontSpec::ui(15.0));
        assert!(w <= 260.0 - 32.0);
    }
}
