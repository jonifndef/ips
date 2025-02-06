use pnet::datalink;
use pnet::ipnetwork::IpNetwork;
use std::process::{Command, Stdio};

#[derive(Default, Debug)]
pub struct InterfaceData {
    pub interface_name: String,
    pub ip_addr: String,
    pub mac_addr: String,
    pub status: String,
    pub ipv6_addrs: Vec<String>,
    pub gateway: String,
    pub connections: Vec<String>
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

// TODO: Use crate that allows usage of a single line-formatted command
fn get_gateway(interface: &datalink::NetworkInterface, data: &mut InterfaceData) {
    // route -n | grep 'UG[ \t]' | grep 'wlp2s0' | awk '{print $2}'
    let route_child = match Command::new("route")
        .arg("-n")
        .stdout(Stdio::piped())
        .spawn() {
        Ok(route_child) => route_child,
        Err(_) => { return; }
    };

    let route_out = match route_child.stdout {
        Some(route_out) => route_out,
        None => { return; }
    };

    let gw_grep_child = match Command::new("grep")
        .arg("UG[ \\t]")
        .stdin(Stdio::from(route_out))
        .stdout(Stdio::piped())
        .spawn() {
        Ok(gw_grep_child) => gw_grep_child,
        Err(_) => { return; }
    };

    let gw_grep_out = match gw_grep_child.stdout {
        Some(gw_grep_out) => gw_grep_out,
        None => { return; }
    };

    let ifc_grep_child = match Command::new("grep")
        .arg(&interface.name)
        .stdin(Stdio::from(gw_grep_out))
        .stdout(Stdio::piped())
        .spawn() {
        Ok(ifc_grep_child) => ifc_grep_child,
        Err(_) => { return; }
    };

    let ifc_grep_out = match ifc_grep_child.stdout {
        Some(ifc_grep_out) => ifc_grep_out,
        None => { return; }
    };

    let awk_child = match Command::new("awk")
        .arg("{print $2}")
        .stdin(Stdio::from(ifc_grep_out))
        .stdout(Stdio::piped())
        .spawn() {
        Ok(awk_child) => awk_child,
        Err(_) => { return; }
    };


    let output = awk_child.wait_with_output().expect("Failed to wait on sed");

    if let Ok(out_str) = String::from_utf8(output.stdout) {
        let trimmed_out_str = out_str.trim_end();
        data.gateway = trimmed_out_str.to_string();
    }
}

fn get_connections(interface: &datalink::NetworkInterface, data: &mut InterfaceData) {
    // nmcli -t con show | grep "wlp2s0" | awk -F: '{print $1}'
    // TODO: What if there are multiple connections using this interface?
    let nmcli_child = match Command::new("nmcli")
        .args(["-t", "con", "show"])
        .stdout(Stdio::piped())
        .spawn() {
        Ok(nmcli_child) => nmcli_child,
        Err(_) => { return; }
    };

    let nmcli_out = match nmcli_child.stdout {
        Some(nmcli_out) => nmcli_out,
        None => { println!("None nmcli_out"); return; }
    };

    let grep_child = match Command::new("grep")
        .arg(&interface.name)
        .stdin(Stdio::from(nmcli_out))
        .stdout(Stdio::piped())
        .spawn() {
        Ok(grep_child) => grep_child,
        Err(_) => { return; }
    };

    let grep_out = match grep_child.stdout {
        Some(grep_out) => grep_out,
        None => { println!("None grep_out"); return; }
    };

    let awk_child = match Command::new("awk")
        .args(["-F:", "{print $1}"])
        .stdin(Stdio::from(grep_out))
        .stdout(Stdio::piped())
        .spawn() {
        Ok(awk_child) => awk_child,
        Err(_) => { return; }
    };

    let output = awk_child.wait_with_output().expect("Failed to wait on sed");

    if let Ok(out_str) = String::from_utf8(output.stdout) {
        let trimmed_out_str = out_str.trim_end();
        data.connections.push(trimmed_out_str.to_string());
    }
}

