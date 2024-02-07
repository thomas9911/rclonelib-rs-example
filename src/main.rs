use librclone::rpc;
use serde_json::json;

#[derive(Debug)]
pub enum Operations {
    FsInfo,
    List,
    Stat,
    Size,
}

impl std::fmt::Display for Operations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let op = match self {
            Operations::FsInfo => "operations/fsinfo",
            Operations::List => "operations/list",
            Operations::Stat => "operations/stat",
            Operations::Size => "operations/size",
        };

        write!(f, "{}", op)
    }
}

impl Into<String> for Operations {
    fn into(self) -> String {
        self.to_string()
    }
}

fn main() -> anyhow::Result<()> {
    librclone::initialize();

    println!("noop: {}", serde_json::to_string_pretty(&noop()?)?);
    println!("fsinfo: {}", serde_json::to_string_pretty(&fsinfo(".")?)?);
    println!("list: {}", serde_json::to_string_pretty(&list(".", "src")?)?);
    println!("size: {}", serde_json::to_string_pretty(&size("", "src")?)?);
    println!("stat: {}", serde_json::to_string_pretty(&stat(".", "src")?)?);

    librclone::finalize();

    Ok(())
}

/// "command=noop fs=. -o echo=yes -o blue -a path1 -a path2"
fn noop() -> anyhow::Result<serde_json::Value> {
    rpc(
        "backend/command",
        json!({"command": "noop", "fs": ".", "arg": [ "path1", "path2" ], "opt": { "blue": "", "echo": "yes" }})
            .to_string(),
    )
    .map_err(|e| anyhow::anyhow!(e))
    .and_then(|out| Ok(serde_json::from_str(&out)?))
}

fn fsinfo(fs: &str) -> anyhow::Result<serde_json::Value> {
    rpc(Operations::FsInfo, json!({"fs": fs}).to_string())
        .map_err(|e| anyhow::anyhow!(e))
        .and_then(|out| Ok(serde_json::from_str(&out)?))
}

fn list(fs: &str, dir: &str) -> anyhow::Result<serde_json::Value> {
    rpc(
        Operations::List,
        json!({"fs": fs, "remote": dir, "opt": { "recurse": false }}).to_string(),
    )
    .map_err(|e| anyhow::anyhow!(e))
    .and_then(|out| Ok(serde_json::from_str(&out)?))
}

fn stat(fs: &str, dir: &str) -> anyhow::Result<serde_json::Value> {
    rpc(
        Operations::Stat,
        json!({"fs": fs, "remote": dir, "opt": { "recurse": false }}).to_string(),
    )
    .map_err(|e| anyhow::anyhow!(e))
    .and_then(|out| Ok(serde_json::from_str(&out)?))
}

fn size(fs: &str, dir: &str) -> anyhow::Result<serde_json::Value> {
    rpc(
        Operations::Size,
        json!({"fs": format!("{}{}", fs, dir), "opt": { "recurse": false }}).to_string(),
    )
    .map_err(|e| anyhow::anyhow!(e))
    .and_then(|out| Ok(serde_json::from_str(&out)?))
}

// // not supported
// fn command() -> anyhow::Result<serde_json::Value> {
//     let out = rpc(
//         "core/command",
//         json!({"command": "ls", "arg": [ "test1:" ],   "opt": { "max-depth": "1" }}).to_string(),
//     )
//     .map_err(|e| anyhow::anyhow!(e))?;
//     let json: serde_json::Value = serde_json::from_str(&out)?;
//     Ok(json)
// }
