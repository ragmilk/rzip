use std::io;
use winreg::RegKey;
use winreg::enums::*;

pub fn install_context_menu() -> io::Result<()> {
    let exe_path = std::env::current_exe()?.to_string_lossy().to_string();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    create_context_menu(&hkcu, r"Software\Classes\*\shell\rzip", "rzip で圧縮", &exe_path, "-e")?;
    create_context_menu(&hkcu, r"Software\Classes\Directory\shell\rzip", "rzip で圧縮", &exe_path, "-e")?;
    create_context_menu(&hkcu, r"Software\Classes\.zip\shell\rzip", "rzip で解凍", &exe_path, "-d")?;
    create_context_menu(&hkcu, r"Software\Classes\CompressedFolder\shell\rzip", "rzip で解凍", &exe_path, "-d")?;
    Ok(())
}

pub fn uninstall_context_menu() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    hkcu.delete_subkey_all(r"Software\Classes\*\shell\rzip")?;
    hkcu.delete_subkey_all(r"Software\Classes\Directory\shell\rzip")?;
    hkcu.delete_subkey_all(r"Software\Classes\.zip\shell\rzip")?;
    hkcu.delete_subkey_all(r"Software\Classes\CompressedFolder\shell\rzip")?;

    Ok(())
}

fn create_context_menu(hkcu: &RegKey, key_path: &str, display_name: &str, exe_path: &str, flag: &str) -> io::Result<()> {
    let shell_key = hkcu.create_subkey(key_path).unwrap().0;
    shell_key.set_value("", &display_name).unwrap();
    shell_key.set_value("Icon", &format!("{},0", exe_path)).unwrap();

    let command_key = shell_key.create_subkey("command").unwrap().0;
    command_key.set_value("", &format!("\"{}\" {} \"%1\"", exe_path, flag)).unwrap();

    Ok(())
}
