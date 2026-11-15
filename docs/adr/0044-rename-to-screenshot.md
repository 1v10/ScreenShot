# 44. 软件与发布包改名为 ScreenShot

- Status: Accepted
- Context: 用户澄清先前将本截图项目与字体参考项目 ScreenTrans 混淆，要求软件和打包统一改名 ScreenShot。
- Decision: 展示名、二进制、Windows 产品名与文件描述、Actions artifact、Release 标题和发布附件均使用 ScreenShot；技术标识使用 screenshot / SCREENSHOT。版本保留 1.0.0，继续仅推送 v* tag 自动构建发布，普通提交/推送不触发。
- Compatibility: 新配置位于 `%APPDATA%\screenshot\config.toml`；新文件不存在时依次兼容读取旧 `%APPDATA%\screentrans\config.toml`、`%APPDATA%\airsnip\config.toml`。不覆盖旧文件，不移动已保存截图。旧版自启需用户在旧版关闭后在新版重新开启，防止并行启动争抢热键。字体参考项目仍是真实的 ScreenTrans，不修改另一个项目目录。
- Validation: 本地测试、构建、检查 EXE 产品元数据、打包和发布配置。远程 Git 操作、tag 创建和推送仍由用户执行。
