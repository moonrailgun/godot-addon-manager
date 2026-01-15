use std::fs;
use std::path::Path;

/// Plugin information from plugin.cfg
#[derive(Debug, Default)]
pub struct PluginInfo {
    pub name: Option<String>,
    pub version: Option<String>,
}

/// Parse a Godot plugin.cfg file (INI format)
pub fn parse_plugin_cfg(path: &Path) -> PluginInfo {
    let mut info = PluginInfo::default();

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return info,
    };

    let mut in_plugin_section = false;

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with('[') && line.ends_with(']') {
            in_plugin_section = line == "[plugin]";
            continue;
        }

        if in_plugin_section {
            if let Some((key, value)) = parse_ini_line(line) {
                match key {
                    "name" => info.name = Some(value),
                    "version" => info.version = Some(value),
                    _ => {}
                }
            }
        }
    }

    info
}

/// Parse a single INI line into key-value pair
fn parse_ini_line(line: &str) -> Option<(&str, String)> {
    let mut parts = line.splitn(2, '=');
    let key = parts.next()?.trim();
    let value = parts.next()?.trim();

    // Remove surrounding quotes if present
    let value = value
        .trim_start_matches('"')
        .trim_end_matches('"')
        .to_string();

    Some((key, value))
}
