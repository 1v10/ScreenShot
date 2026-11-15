# ScreenTrans 构建与发布

## 约定

软件展示名 `ScreenTrans`，Cargo 包名 `screentrans`，可执行文件 `ScreenTrans.exe`。当前版本 `1.0.0`，唯一版本源是 `Cargo.toml`，Windows 文件属性中的产品版本由构建脚本自动生成。

仓库内不会保存个人 token 或绑定上游仓库。远程 Git 操作、创建 tag 和推送由维护者自行执行；本次只完成本地文件配置与验证。

## 本地构建

Windows x64，Rust 1.98.0、MSVC C++ Build Tools 和 Windows SDK。支持 PowerShell 5.1 / 7。

```powershell
./scripts/build.ps1                  # 格式检查、全部桌面测试、release 构建、打包
./scripts/build.ps1 -Headless        # 无交互桌面环境：跳过两个真实桌面抓屏测试
./scripts/package.ps1 -SkipBuild     # 仅重新打包已有 release
./scripts/check-release.ps1 -Tag v1.0.0  # 本地版本/说明检查，不创建 tag、不联网
```

`cargo test` 中 ignored 项需相应的 HDR/多屏等实机环境，不由 CI 自动开启。托管 runner 的无桌面模式额外跳过 `capture::tests::capture_screen_works` 和 `capture::tests::bitblt_rop_timing`，其他测试照常执行。CI 的静态检查与字体布局测试不能替代真实桌面交互验证。

标准产物：

- `dist/ScreenTrans-v1.0.0-win-x64.zip`：含 ScreenTrans.exe、README、LICENSE、THIRD-PARTY.md。
- `dist/ScreenTrans-v1.0.0-win-x64.exe`：独立运行文件。
- `dist/ScreenTrans-v1.0.0-win-x64.sha256`：上述两份文件的校验和。

打包会检查 5 MB 体积上限、产品名/版本、图标和 PE 运行库依赖。MSVC CRT 必须静态链接。EXE 未做代码签名，GitHub Release 和 SHA-256 文件也不等于 Windows 代码签名。

## Actions 构建

`.github/workflows/build.yml` 在分支 push、pull request 和手动触发时执行 Windows x64 构建，并上传保留 14 天的 artifact。此流程没有 Release 写入权限。

两条工作流都先调用 `dependencies.yml`，根据 `deny.toml` 执行 cargo-deny 的许可证、安全公告、依赖和来源检查，检查失败会阻止打包/发布。当前机器没有 cargo-deny，已配置云端检查但未在本地执行该扫描。

`.github/workflows/release.yml` 仅响应 `v*` tag 的推送，先核对 tag 与 Cargo 版本完全一致、对应发布说明非空，再测试/打包并发布 GitHub Release。使用当前仓库自动提供的 `GITHUB_TOKEN`，仅发布 job 声明 `contents: write`；无需配置个人 PAT。

tag 如 `v1.0.1-beta.1` 必须对应 `Cargo.toml` 版本 `1.0.1-beta.1`，发布为 prerelease 且不会成为 latest。稳定版本发布为 latest。创建 tag 的代码必须已经包含这些工作流。手动运行 Build Windows 只生成 artifact，不发布 Release。

## 由维护者执行的发布步骤

1. 修改 `Cargo.toml` 版本，通过 Cargo 更新锁文件；新增 `docs/releases/v{版本}.md`。本仓库当前已准备 `docs/releases/v1.0.0.md`。
2. 在本机执行 `scripts/build.ps1`，体验托盘、截图、标注、保存和设置，并执行 `scripts/check-release.ps1 -Tag v{版本}`。
3. 检查并提交本地改动，再自行推送目标分支；等 Build Windows 成功。
4. 自行创建并推送版本 tag。以下仅为命令示例，自动化助手没有执行：

```powershell
git tag -a v1.0.0 -m "ScreenTrans v1.0.0"
git push origin v1.0.0
```

5. 查看 Release workflow；成功后仓库 Releases 出现 ZIP、EXE、SHA-256 三份附件。若仓库/组织禁用了 Actions 或限制写权限，需由维护者在 GitHub 设置中允许；配置本身不会修改这些远程设置。

发布说明以 `docs/releases/v{版本}.md` 为准。`.github/release.yml` 额外定义 GitHub 手动生成说明时的分类规则，自动发布流程不依赖 PR 标签。

如果目标 tag 或 Release 已存在，先核查其内容，不应覆盖公开版本。当前流程遇到已有 Release 会报错，由维护者自行决定后续处理，不自动移动 tag、不删除 Release。

## 改名兼容

新配置写入 `%APPDATA%\screentrans\config.toml`；缺少新文件时读取旧 AirSnip 配置。新旧程序的单实例标识不同，不应同时运行。旧 AirSnip 自启任务不会被本次构建命令改动，请先在旧版关闭自启，再从新程序启用。旧截图文件不移动。

历史 `docs/release-notes/` 与早期 ADR 保留上游记录，当前发布说明使用 `docs/releases/`。

工作流接口参考：[GitHub Release 文档](https://docs.github.com/en/repositories/releasing-projects-on-github/automatically-generated-release-notes)、[GitHub CLI release create](https://cli.github.com/manual/gh_release_create)、[cargo-deny Action](https://github.com/EmbarkStudios/cargo-deny-action)。
