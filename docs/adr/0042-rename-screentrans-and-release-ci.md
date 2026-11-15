# 42. 改名 ScreenTrans 并配置本地打包与 GitHub 发布流程

- Status: Accepted
- Context: 用户要求软件和打包改名 ScreenTrans，配置 Actions 与 Release；所有远程 Git 操作、tag 创建和推送均由用户执行。
- Decision: 展示名称、Windows 产品元数据和运行文件使用 ScreenTrans；crate、窗口类、互斥量、配置目录、自启任务与调试环境变量使用 screentrans / SCREENTRANS。构建产物为 `ScreenTrans-v{版本}-win-x64.zip`、同名独立 EXE 和 SHA-256 文件。新配置缺失时兼容读取 `%APPDATA%\airsnip\config.toml`，后续保存到 `%APPDATA%\screentrans`；旧自启任务由用户停用，避免两个程序争抢热键。
- CI: Windows x64/MSVC，固定 Rust 1.98.0，格式检查、测试、release 构建及打包共用本地脚本。无交互桌面的 CI 只跳过两个真实桌面抓屏测试，其他测试照常执行。普通推送/PR/手动构建上传 Actions artifacts。
- Release: 仅用户推送 `v*` tag 时触发；严格匹配 Cargo.toml 版本、要求存在对应发布说明。通过后发布 ZIP、EXE 和 SHA-256 文件。含预发布标识的版本标记 prerelease，稳定版本标记 latest。不恢复应用内自动更新。工作流使用当前仓库的 GITHUB_TOKEN，不绑定上游仓库或个人 token。
- Validation: 本地执行全部桌面测试、构建与打包；校验工作流 YAML 和发布参数。不会调用远程 Git 或 GitHub 发布命令来验证。云端工作流由用户推送后首次运行验证。
