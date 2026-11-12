//! Windows registry read helpers for Workflowy.

pub const APP_KEY: &str = "Software\\Workflowy\\Workflowy";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Workflowy";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Workflowy") },
        RegValue { name: "Version",     value: String::from("v3.2.0") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Workflowy") },
        RegValue { name: "Publisher",       value: String::from("Workflowy") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Workflowy\\uninstall.exe\"") },
    ]
}
