#[cfg(target_os = "windows")]
use crate::errors::HWIDError;
#[cfg(target_os = "windows")]
use serde::Deserialize;
#[cfg(target_os = "windows")]
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
#[cfg(target_os = "windows")]
use wmi::WMIConnection;

#[cfg(target_os = "windows")]
pub fn get_hwid() -> Result<String, HWIDError> {
    use winreg::enums::{KEY_READ, KEY_WOW64_64KEY};

    let rkey = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Cryptography",
        KEY_READ | KEY_WOW64_64KEY,
    )?;

    let id: String = rkey.get_value("MachineGuid")?;
    Ok(id)
}

#[cfg(target_os = "windows")]
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DiskGeneric {
    serial_number: Option<String>,
}

#[cfg(target_os = "windows")]
#[derive(Deserialize)]
struct MACGeneric {
    #[serde(rename = "MACAddress")]
    mac_address: Option<String>,
}

#[cfg(target_os = "windows")]
pub(crate) fn get_disk_id() -> Result<String, HWIDError> {
    let con = WMIConnection::new()?;

    let disks: Vec<DiskGeneric> = con.raw_query("SELECT SerialNumber FROM Win32_PhysicalMedia")?;

    let serial = disks
        .into_iter()
        .find_map(|d| d.serial_number)
        .ok_or(HWIDError::new(
            "UuidError",
            "Could not retrieve disk serial number",
        ))?;

    Ok(serial)
}

#[cfg(target_os = "windows")]
pub(crate) fn get_mac_address() -> Result<String, HWIDError> {
    let con = WMIConnection::new()?;

    let adapters: Vec<MACGeneric> =
        con.raw_query("SELECT MACAddress FROM Win32_NetworkAdapter WHERE MACAddress IS NOT NULL")?;

    let mac = adapters
        .into_iter()
        .find_map(|a| a.mac_address)
        .ok_or(HWIDError::new(
            "MACAddress",
            "Could not retrieve Mac Address",
        ))?;

    Ok(mac)
}
