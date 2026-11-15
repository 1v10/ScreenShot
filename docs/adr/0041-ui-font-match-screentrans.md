# 41. 界面字体与 ScreenTrans 保持一致

- Status: Accepted
- Context: 用户希望 AirSnip 使用本地 ScreenTrans 的界面字体。参考 `D:\work\ScreenTrans\ScreenTrans\ui\main.css`：`Segoe UI`, `Microsoft YaHei`, system-ui, sans-serif。
- Decision: 设置面板、托盘菜单、工具栏文字、快捷键、提示和长截图状态条使用 Segoe UI；缺字时优先回退 Microsoft YaHei，再使用系统字体回退。文字测量与绘制共用相同的 DirectWrite 字体规格和回退规则。截图标注的字体选择和默认 Microsoft YaHei UI 保持独立。
- Consequences: 不增加字体文件和运行时依赖。保留原字号与布局缩放规则，并运行现有布局/文本检查防止文字溢出。字体族与参考应用一致，但原生灰度栅格化与 WebView 的抗锯齿实现不同，不保证像素完全相同。
