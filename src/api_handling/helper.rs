/// Modifies the supplied `input` to generate proper snake case.
pub fn rustify_string(input: &str) -> String {
    input
        .replace("NewX_AVM-DE_", "newXAvmDe")
        .replace("NewX_AVM_DE_", "newXAvmDe")
        .replace("X_AVM-DE_", "XAvmDe")
        .replace("X_", "x")
        .replace('_', "")
        .replace("NATRSIP", "NatRsip")
        .replace("NAT", "Nat")
        .replace("RSIP", "Rsip")
        .replace("FCS", "Fcs")
        .replace("ATM", "Atm")
        .replace("DAV", "Dav")
        .replace("PPP", "Ppp")
        .replace("WAN", "Wan")
        .replace("MAC", "Mac")
        .replace("AIN", "Ain")
        .replace("DDNS", "Ddns")
        .replace("DNS", "Dns")
        .replace("IPTVo", "IptvO")
        .replace("IPTV", "Iptv")
        .replace("US", "Us")
        .replace("VoIP", "Voip")
        .replace("AVM", "Avm")
        .replace("URL", "Url")
        .replace("ATUC", "Atuc")
        .replace("CHECK", "Check")
        .replace("DSL", "Dsl")
        .replace("DS", "Ds")
        .replace("SNRG", "Snrg_")
        .replace("SNRMT", "Snrmt_")
        .replace("SNR", "Snr_")
        .replace("LATN", "Latn_")
        .replace("HEC", "Hec")
        .replace("TAM", "Tam")
        .replace("OKZ", "Okz")
        .replace("LKZ", "Lkz")
        .replace("OKZ", "Okz")
        .replace("STUN", "Stun")
        .replace("UPnP", "Upnp")
        .replace("FTP", "Ftp")
        .replace("SSL", "Ssl")
        .replace("SMB", "Smb")
        .replace("CGI", "Cgi")
        .replace("NTP", "Ntp")
        .replace("TR069", "Tr069")
        .replace("BSSID", "Bssid")
        .replace("SSID", "Ssid")
        .replace("SID", "Sid")
        .replace("UUID", "Uuid")
        .replace("OUI", "Oui")
        .replace("ATUR", "Atur")
        .replace("FEC", "Fec")
        .replace("CRC", "Crc")
        .replace("PSK", "Psk")
        .replace("WEP", "Wep")
        .replace("WPA", "Wpa")
        .replace("WLAN", "Wlan")
        .replace("LAN", "Lan")
        .replace("AP", "Ap")
        .replace("WPS", "Wps")
        .replace("RX", "Rx")
        .replace("WOL", "Wol")
        .replace("DHCP", "Dhcp")
        .replace("ID", "Id")
        .replace("IP", "Ip")
        .chars()
        .enumerate()
        .map(|x| {
            if x.1.is_uppercase() {
                if x.0 == 0 {
                    x.1.to_lowercase().to_string()
                } else {
                    format!("_{}", x.1.to_lowercase())
                }
            } else {
                x.1.to_string()
            }
        })
        .collect()
}