use librclone::rpc;
use serde_json::json;

fn main() -> anyhow::Result<()> {
    librclone::initialize();

    println!("{}", noop()?);
    println!("{}", fsinfo(".")?);
    println!("{}", list(".", "src")?);
    println!("{}", size("", "src")?);
    println!("{}", stat(".", "src")?);

    librclone::finalize();

    Ok(())
}

/// "command=noop fs=. -o echo=yes -o blue -a path1 -a path2"
fn noop() -> anyhow::Result<serde_json::Value> {
    let out = rpc( "backend/command", json!({"command": "noop", "fs": ".", "arg": [ "path1", "path2" ],   "opt": { "blue": "", "echo": "yes" }}).to_string() ).map_err(|e| anyhow::anyhow!(e))?;
    let json: serde_json::Value = serde_json::from_str(&out)?;
    Ok(json)
}

fn fsinfo(fs: &str) -> anyhow::Result<serde_json::Value> {
    rpc("operations/fsinfo", json!({"fs": fs}).to_string())
        .map_err(|e| anyhow::anyhow!(e))
        .and_then(|out| Ok(serde_json::from_str(&out)?))
}

fn list(fs: &str, dir: &str) -> anyhow::Result<serde_json::Value> {
    let out = rpc(
        "operations/list",
        json!({"fs": fs, "remote": dir, "opt": { "recurse": false }}).to_string(),
    )
    .map_err(|e| anyhow::anyhow!(e))?;
    let json: serde_json::Value = serde_json::from_str(&out)?;
    Ok(json)
}

fn stat(fs: &str, dir: &str) -> anyhow::Result<serde_json::Value> {
    let out = rpc(
        "operations/stat",
        json!({"fs": fs, "remote": dir, "opt": { "recurse": false }}).to_string(),
    )
    .map_err(|e| anyhow::anyhow!(e))?;
    let json: serde_json::Value = serde_json::from_str(&out)?;
    Ok(json)
}

fn size(fs: &str, dir: &str) -> anyhow::Result<serde_json::Value> {
    let out = rpc(
        "operations/size",
        json!({"fs": format!("{}{}", fs, dir), "opt": { "recurse": false }}).to_string(),
    )
    .map_err(|e| anyhow::anyhow!(e))?;
    let json: serde_json::Value = serde_json::from_str(&out)?;
    Ok(json)
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
