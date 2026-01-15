use crate::error::Result;

pub fn execute() -> Result<()> {
    // TODO: Implement version command
    let version = env!("CARGO_PKG_VERSION");
    println!("godot-addon-manager version: v{}", version);
    Ok(())
}
