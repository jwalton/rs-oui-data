//! OUI database, generated from IEEE CSV files.
//!
//! Use the `lookup` function to lookup a MAC address in the MA-L, MA-M, MA-S, CID,
//! and IAB registries.
//!
//! ```rs
//! let record = oui::lookup("00:00:00:00:00:00").unwrap();
//! assert_eq!(record.organization(), "XEROX CORPORATION");
//! ```
//!

/// The registry for an OUI record.
#[derive(Debug, Copy, Clone)]
pub enum Registry {
    /// MA-L 24-bit OUI (Organizationally Unique Identifier) registry.
    MAL,
    /// MA-M 28-bit registry.
    MAM,
    /// MA-S 36-bit registry.
    MAS,
    /// CID 24-bit registry. Entries in the CID registry are used for cases where
    /// unique MAC addresses are not required.
    CID,
    /// IAB (Individual Address Blocks) registry.
    IAB,
}

/// A record from the OUI database.
#[derive(Debug, Clone)]
pub struct OuiData {
    registry: Registry,
    oui: &'static str,
    organization: &'static str,
}

impl OuiData {
    /// The registry for this record.
    pub fn registry(&self) -> Registry {
        self.registry
    }

    /// The MAC address prefix for this record. This is an upper-case string
    /// like "000000", representing MAC addresses that are prefixed with "00:00:00".
    pub fn oui(&self) -> &'static str {
        self.oui
    }

    /// The name of the vendor associated with this record.
    pub fn organization(&self) -> &'static str {
        self.organization
    }
}

/// Retrieve the OUI record for a given MAC address.
pub fn lookup(mac: &str) -> Option<&'static OuiData> {
    if !mac.contains(':') && mac.chars().all(|c| matches!(c, '0'..='9' | 'A'..='F')) {
        return lookup_prefix(mac);
    }

    let mac = mac.to_uppercase().replace(':', "");
    lookup_prefix(&mac)
}

fn lookup_prefix(mac: &str) -> Option<&'static OuiData> {
    let mut result: Option<&'static OuiData> = None;
    if mac.len() >= 9 {
        result = OUI_ENTRIES.get(&mac[..9]);
    }
    if mac.len() >= 7 {
        result = result.or_else(|| OUI_ENTRIES.get(&mac[..7]));
    }
    if mac.len() >= 6 {
        result = result.or_else(|| OUI_ENTRIES.get(&mac[..6]));
    }

    result
}

include!(concat!(env!("OUT_DIR"), "/oui.rs"));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_lookup_entry_from_oui() {
        let record = lookup("00:00:00:00:00:00").unwrap();
        assert_eq!(record.organization(), "XEROX CORPORATION");
    }

    #[test]
    fn should_ignore_case() {
        let record = lookup("50:a6:d8:00:00:00").unwrap();
        assert_eq!(record.organization(), "Apple, Inc.");
    }

    #[test]
    fn should_ignore_colons() {
        let record = lookup("50A6D8000000").unwrap();
        assert_eq!(record.organization(), "Apple, Inc.");
    }

    #[test]
    fn should_accept_prefix() {
        let record = lookup("50A6D8").unwrap();
        assert_eq!(record.organization(), "Apple, Inc.");
    }

    #[test]
    fn should_lookup_entry_from_oui28() {
        let record = lookup("B8:4C:87:40:00:00").unwrap();
        assert_eq!(record.organization(), "Blum Novotest GmbH");
    }

    #[test]
    fn should_lookup_entry_from_oui36() {
        let record = lookup("8C:1F:64:AF:A0:00").unwrap();
        assert_eq!(record.organization(), "DATA ELECTRONIC DEVICES, INC");
    }

    #[test]
    fn should_lookup_entry_from_cid() {
        let record = lookup("EA:27:01:00:00:00").unwrap();
        assert_eq!(record.organization(), "ACCE Technology Corp.");
    }

    #[test]
    fn should_lookup_entry_from_iab() {
        let record = lookup("40:D8:55:0D:70:00").unwrap();
        assert_eq!(record.organization(), "Avant Technologies");
    }
}
