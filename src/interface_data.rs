use pnet::datalink;
use pnet::ipnetwork::IpNetwork;
use duct::cmd;

use crate::colors;

#[derive(Default, Debug)]
pub struct InterfaceData {
    pub interface_name: String,
    pub ip_addr: String,
    pub status: String,
    pub mac_addr: String,
    pub ipv6_addrs: Vec<String>,
    pub gateway: String,
    pub connections: Vec<String>
}

pub struct FieldWidths {
    pub interface_name: usize,
    pub ip_addr: usize,
    pub status: usize,
    pub mac: usize,
    pub ipv6: usize,
    pub gateway: usize,
    pub connections: usize
}

#[derive(Debug)]
pub enum IfcField {
    NAME,
    IP,
    STATUS,
    MAC,
    IPV6,
    GW,
    CONN
}

impl InterfaceData {
    pub fn get(&self, field: &IfcField, linenum: usize) -> &str {
        let val = match field {
            IfcField::NAME   => if linenum == 0 { self.interface_name.as_str() } else { "" },
            IfcField::IP     => if linenum == 0 { self.ip_addr.as_str() } else { "" },
            IfcField::STATUS => if linenum == 0 { self.status.as_str() } else { "" },
            IfcField::MAC    => if linenum == 0 { self.mac_addr.as_str() } else { "" },
            IfcField::IPV6   => if let Some(addr) = self.ipv6_addrs.get(linenum) { addr.as_str() } else { "" },
            IfcField::GW     => if linenum == 0 { self.gateway.as_str() } else { "" },
            IfcField::CONN   => if let Some(connection) = self.connections.get(linenum) { connection.as_str() } else { "" },
        };

        val
    }
}

impl FieldWidths {
    pub fn get(&self, field: &IfcField) -> usize {
        let val = match field {
            IfcField::NAME   => self.interface_name,
            IfcField::IP     => self.ip_addr,
            IfcField::STATUS => self.status,
            IfcField::MAC    => self.mac,
            IfcField::IPV6   => self.ipv6,
            IfcField::GW     => self.gateway,
            IfcField::CONN   => self.connections,
        };

        val
    }
}

pub fn get_interface_data() -> Vec::<InterfaceData> {
    let mut interface_data = Vec::<InterfaceData>::new();

    let interfaces = datalink::interfaces();
    for interface in interfaces
    {
        if interface.is_loopback() {
            continue;
        }

        let mut data = InterfaceData::default();

        data.status = String::from(if interface.is_up() { "UP" } else { "DOWN" });
        data.interface_name = interface.name.clone();

        for ip in interface.ips.iter() {
            match ip {
                IpNetwork::V4(ip_addr) => {
                    data.ip_addr = ip_addr.to_string();
                }
                IpNetwork::V6(ip_addr) => {
                    data.ipv6_addrs.push(ip_addr.to_string())
                }
            }
        }

        if let Some(mac_addr) = interface.mac {
            data.mac_addr = mac_addr.to_string();
        }

        get_gateway(&interface, &mut data);
        get_connections(&interface, &mut data);

        interface_data.push(data);
    }

    interface_data
}

pub fn get_field_widths(interfaces: &[InterfaceData], args: &crate::Args) -> FieldWidths {
    let mut widths = FieldWidths {
        interface_name: 0,
        ip_addr: 15,
        status: 0,
        mac: 18,
        ipv6: 0,
        gateway: 12,
        connections: 0
    };

    for interface in interfaces {
        if interface.interface_name.len() > widths.interface_name {
            widths.interface_name = interface.interface_name.len() + 1;
        }

        if interface.status.len() > widths.status {
            widths.status = interface.status.len() + 1;
        }

        for ipv6 in &interface.ipv6_addrs {
            if ipv6.len() > widths.ipv6 {
                widths.ipv6 = ipv6.len() + 1;
            }
        }

        for conn in &interface.connections {
            if conn.len() > widths.connections {
                widths.connections = conn.len() + 1;
            }
        }
    }

    if !args.nocolor {
        widths.interface_name += colors::ColorTokens::TOKENS_LEN;
        widths.ip_addr += colors::ColorTokens::TOKENS_LEN;
        widths.status += colors::ColorTokens::TOKENS_LEN;
        widths.mac += colors::ColorTokens::TOKENS_LEN;
        widths.ipv6 += colors::ColorTokens::TOKENS_LEN;
        widths.gateway += colors::ColorTokens::TOKENS_LEN;
        widths.connections += colors::ColorTokens::TOKENS_LEN;
    }

    return widths
}

// TODO: Use crate that allows usage of a single line-formatted command
fn get_gateway(interface: &datalink::NetworkInterface, data: &mut InterfaceData) {
    // route -n | grep 'UG[ \t]' | grep 'wlp2s0' | awk '{print $2}'
    if let Ok(output) = cmd!("route", "-n")
        .pipe(cmd!("grep", "UG[ \t]"))
        .pipe(cmd!("grep", &interface.name))
        .pipe(cmd!("awk", "{print $2}"))
        .read() {
        data.gateway = output.trim().to_string();
    }
}

fn get_connections(interface: &datalink::NetworkInterface, data: &mut InterfaceData) {
    // nmcli -t con show | grep "wlp2s0" | awk -F: '{print $1}'
    if let Ok(output) = cmd!("nmcli", "-t", "con", "show")
        .pipe(cmd!("grep", &interface.name))
        .pipe(cmd!("awk", "-F:", "{print $1}"))
        .read() {
        let trimmed_out_str = output.trim_end();
        data.connections.push(trimmed_out_str.to_string());
    }
}
