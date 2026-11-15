# 纯本地校验，不创建 tag，不访问 GitHub。输出供 release.yml 使用的元数据。
param([Parameter(Mandatory = $true)][string]$Tag)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$versionMatch = Select-String -LiteralPath (Join-Path $root 'Cargo.toml') -Pattern '^version = "([^"]+)"' | Select-Object -First 1
if (-not $versionMatch) { throw 'Missing Cargo package version' }
$version = $versionMatch.Matches[0].Groups[1].Value
if ($Tag -notmatch '^v[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$' -or $Tag -cne "v$version") {
    throw "Tag must exactly match Cargo.toml: expected v$version, received $Tag"
}
$notes = "docs/releases/v$version.md"
if (-not (Test-Path -LiteralPath (Join-Path $root $notes) -PathType Leaf)) {
    throw "Missing release notes: $notes"
}
if ([string]::IsNullOrWhiteSpace([IO.File]::ReadAllText((Join-Path $root $notes)))) {
    throw "Empty release notes: $notes"
}
$isPrerelease = $version.Contains('-').ToString().ToLowerInvariant()
$lines = @("version=$version", "prerelease=$isPrerelease", "notes=$notes", "asset_prefix=ScreenTrans-v$version-win-x64")
if ($env:GITHUB_OUTPUT) { $lines | Out-File -FilePath $env:GITHUB_OUTPUT -Encoding utf8 -Append }
$lines | Write-Output
