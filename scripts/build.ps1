# 本地与 GitHub Actions 共用的格式检查、测试、release 构建与打包入口。
param([switch]$Headless)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    cargo fmt --check
    if ($LASTEXITCODE -ne 0) { throw 'cargo fmt --check failed' }

    # 托管 runner 无可交互桌面，跳过两个真实屏幕测试；本地默认执行全部测试。
    if ($Headless) {
        cargo test --locked -- --skip capture::tests::capture_screen_works --skip capture::tests::bitblt_rop_timing
    } else {
        cargo test --locked
    }
    if ($LASTEXITCODE -ne 0) { throw 'cargo test failed' }

    & (Join-Path $PSScriptRoot 'package.ps1')
} finally {
    Pop-Location
}
