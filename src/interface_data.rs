use std::collections::HashMap;

use duct::cmd;
use pnet::datalink;
use pnet::ipnetwork::IpNetwork;

use crate::colors;

#[derive(Default, Debug)]
pub struct InterfaceData {
    pub interface_name: String,
    pub ip_addr: String,
    pub status: String,
    pub mac_addr: String,
    pub ipv6_addrs: Vec<String>,
    pub gateway: String,
    pub connections: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum IfcField {
    Name,
    Ip,
    Status,
    Mac,
    Ipv6,
    Gw,
    Conn,
}

//impl Hash for IfcField {
//
//}

impl InterfaceData {
    pub fn get(&self, field: &IfcField, linenum: usize) -> &str {
        let val = match field {
            IfcField::Name => {
                if linenum == 0 {
                    self.interface_name.as_str()
                } else {
                    ""
                }
            }
            IfcField::Ip => {
                if linenum == 0 {
                    self.ip_addr.as_str()
                } else {
                    ""
                }
            }
            IfcField::Status => {
                if linenum == 0 {
                    self.status.as_str()
                } else {
                    ""
                }
            }
            IfcField::Mac => {
                if linenum == 0 {
                    self.mac_addr.as_str()
                } else {
                    ""
                }
            }
            IfcField::Ipv6 => {
                if let Some(addr) = self.ipv6_addrs.get(linenum) {
                    addr.as_str()
                } else {
                    ""
                }
            }
            IfcField::Gw => {
                if linenum == 0 {
                    self.gateway.as_str()
                } else {
                    ""
                }
            }
            IfcField::Conn => {
                if let Some(connection) = self.connections.get(linenum) {
                    connection.as_str()
                } else {
                    ""
                }
            }
        };

        val
    }
}

pub fn get_interface_data() -> Vec<InterfaceData> {
    let mut interface_data = Vec::<InterfaceData>::new();

    let interfaces = datalink::interfaces();
    for interface in interfaces {
        if interface.is_loopback() {
            continue;
        }

        let mut data = InterfaceData {
            interface_name: interface.name.clone(),
            status: String::from(if interface.is_up() { "UP" } else { "DOWN" }),
            ..Default::default()
        };

        for ip in interface.ips.iter() {
            match ip {
                IpNetwork::V4(ip_addr) => {
                    data.ip_addr = ip_addr.to_string();
                }
                IpNetwork::V6(ip_addr) => data.ipv6_addrs.push(ip_addr.to_string()),
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

pub fn get_field_widths(
    interfaces: &[InterfaceData],
    args: &crate::Args,
) -> HashMap<IfcField, usize> {
    let mut widths = HashMap::new();

    widths.insert(IfcField::Name, 0);
    widths.insert(IfcField::Ip, 0);
    widths.insert(IfcField::Status, 0);
    widths.insert(IfcField::Mac, 0);
    widths.insert(IfcField::Ipv6, 0);
    widths.insert(IfcField::Gw, 0);
    widths.insert(IfcField::Conn, 0);

    for interface in interfaces {
        if let Some(width) = widths.get_mut(&IfcField::Name) {
            if interface.interface_name.len() > *width {
                *width = interface.interface_name.len() + 1;
            }
        }

        if let Some(width) = widths.get_mut(&IfcField::Ip) {
            if interface.ip_addr.len() > *width {
                *width = interface.ip_addr.len() + 1;
            }
        }

        if let Some(width) = widths.get_mut(&IfcField::Status) {
            if interface.status.len() > *width {
                *width = interface.status.len() + 1;
            }
        }

        if let Some(width) = widths.get_mut(&IfcField::Mac) {
            if interface.mac_addr.len() > *width {
                *width = interface.mac_addr.len() + 1;
            }
        }

        for ipv6 in &interface.ipv6_addrs {
            if let Some(width) = widths.get_mut(&IfcField::Ipv6) {
                if ipv6.len() > *width {
                    *width = ipv6.len() + 1;
                }
            }
        }

        if let Some(width) = widths.get_mut(&IfcField::Gw) {
            if interface.gateway.len() > *width {
                *width = interface.gateway.len() + 1;
            }
        }

        for conn in &interface.connections {
            if let Some(width) = widths.get_mut(&IfcField::Conn) {
                if conn.len() > *width {
                    *width = conn.len() + 1;
                }
            }
        }
    }

    if !args.nocolor {
        if let Some(width) = widths.get_mut(&IfcField::Name) {
            *width += colors::ColorTokens::TOKENS_LEN;
        }
        if let Some(width) = widths.get_mut(&IfcField::Ip) {
            *width += colors::ColorTokens::TOKENS_LEN;
        }
        if let Some(width) = widths.get_mut(&IfcField::Status) {
            *width += colors::ColorTokens::TOKENS_LEN;
        }
        if let Some(width) = widths.get_mut(&IfcField::Mac) {
            *width += colors::ColorTokens::TOKENS_LEN;
        }
        if let Some(width) = widths.get_mut(&IfcField::Ipv6) {
            *width += colors::ColorTokens::TOKENS_LEN;
        }
        if let Some(width) = widths.get_mut(&IfcField::Gw) {
            *width += colors::ColorTokens::TOKENS_LEN;
        }
        if let Some(width) = widths.get_mut(&IfcField::Conn) {
            *width += colors::ColorTokens::TOKENS_LEN;
        }
    }

    widths
}

fn get_gateway(interface: &datalink::NetworkInterface, data: &mut InterfaceData) {
    // route -n | grep 'UG[ \t]' | grep 'wlp2s0' | awk '{print $2}'
    if let Ok(output) = cmd!("route", "-n")
        .pipe(cmd!("grep", "UG[ \t]"))
        .pipe(cmd!("grep", &interface.name))
        .pipe(cmd!("awk", "{print $2}"))
        .read()
    {
        data.gateway = output.trim().to_string();
    }
}

fn get_connections(interface: &datalink::NetworkInterface, data: &mut InterfaceData) {
    // nmcli -t con show | grep "wlp2s0" | awk -F: '{print $1}'
    if let Ok(output) = cmd!("nmcli", "-t", "con", "show")
        .pipe(cmd!("grep", &interface.name))
        .pipe(cmd!("awk", "-F:", "{print $1}"))
        .read()
    {
        let trimmed_out_str = output.trim_end();
        data.connections.push(trimmed_out_str.to_string());
    }
}
