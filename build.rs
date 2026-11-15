// SPDX-License-Identifier: GPL-3.0-only
//! 构建脚本：嵌入 Windows manifest 与图标资源。
//! release 构建声明 requireAdministrator，dev 构建不提权（见 ADR 0003）。

fn main() {
    let profile = std::env::var("PROFILE").unwrap_or_default();
    let rc = if profile == "release" { "app.rc" } else { "app.dev.rc" };
    println!("cargo:rerun-if-changed=app.rc");
    println!("cargo:rerun-if-changed=app.dev.rc");
    println!("cargo:rerun-if-changed=app.manifest");
    println!("cargo:rerun-if-changed=app.dev.manifest");
    // 图标资源也要 watch：否则只换 app.ico 时 cargo 不会重跑 build.rs，图标不生效
    println!("cargo:rerun-if-changed=assets/icons/app.ico");
    let root = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let version = std::env::var("CARGO_PKG_VERSION").unwrap();
    let numeric = ["MAJOR", "MINOR", "PATCH"]
        .map(|part| std::env::var(format!("CARGO_PKG_VERSION_{part}")).unwrap())
        .join(",");
    let resource = format!(
        r#"#include "{}"
1 VERSIONINFO
FILEVERSION {numeric},0
PRODUCTVERSION {numeric},0
FILEFLAGSMASK 0x3fL
FILEFLAGS 0x0L
FILEOS 0x40004L
FILETYPE 0x1L
FILESUBTYPE 0x0L
BEGIN
  BLOCK "StringFileInfo"
  BEGIN
    BLOCK "040904b0"
    BEGIN
      VALUE "FileDescription", "ScreenShot\0"
      VALUE "FileVersion", "{version}\0"
      VALUE "InternalName", "ScreenShot\0"
      VALUE "LegalCopyright", "GPL-3.0-only; based on AirSnip\0"
      VALUE "OriginalFilename", "ScreenShot.exe\0"
      VALUE "ProductName", "ScreenShot\0"
      VALUE "ProductVersion", "{version}\0"
    END
  END
  BLOCK "VarFileInfo"
  BEGIN
    VALUE "Translation", 0x0409, 1200
  END
END
"#,
        root.join(rc).to_string_lossy().replace('\\', "/")
    );
    let generated = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("screen-shot.rc");
    std::fs::write(&generated, resource).expect("write ScreenShot resource");
    embed_resource::compile(generated, embed_resource::NONE)
        .manifest_required()
        .expect("embed ScreenShot manifest, icon and version");
}
