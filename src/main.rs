use clap::Parser;
use pnet::datalink;
use pnet::ipnetwork::IpNetwork;
use std::process::{Command, Stdio};

/// Application that prints your network interfaces with associated information, such as ipv4 address, status etc
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// List mac addresses for the interfaces
    #[arg(short, long)]
    mac: bool,

    /// List ipv6 addresses for the interfaces
    #[arg(short, long)]
    ipv6: bool,

    /// List gateways for the interfaces
    #[arg(short, long)]
    gateway: bool,

    /// List NetworkManager connections that use the respective interfaces
    #[arg(short, long)]
    connections: bool,

    /// Disable colored output
    #[arg(short, long)]
    nocolor: bool
}

#[derive(Default)]
struct InterfaceData {
    interface_name: String,
    ip_addr: String,
    mac_addr: String,
    status: String,
    ipv6_addrs: Vec<String>,
    gateway: String,
    connections: Vec<String>
}

fn main() {
    let args = Args::parse();
    let interfaces = get_interface_data();

    print_interfaces(args, &interfaces);
}

fn get_interface_data() -> Vec::<InterfaceData> {
    let mut interface_data = Vec::<InterfaceData>::new();

    let interfaces = datalink::interfaces();
    for interface in interfaces
    {
        if interface.is_loopback() {
            continue;
        }

        let mut data = InterfaceData::default();

        if interface.is_up() {
            data.status = String::from("UP");
        }
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

fn print_interfaces(args: Args, interfaces: &[InterfaceData]) {
    let mut name_len = 0;
    let mut ipv6_len = 0;
    let mut conn_len = 0;
    for interface in interfaces {
        if interface.interface_name.len() > name_len {
            name_len = interface.interface_name.len();
        }

        // for ipv6 in interfae.ipv6 blabla...
        // if ipv6.len() > ipv6_len {
        //     ipv6_len = ipv6.len(); etc etc etc
        // }

        // for conn in interfae.connections blabla...
        // if conn.len() > conn_len {
        //     conn_len = conn.len(); etc etc etc
        // }
    }

    let name_alignment = name_len;

    for interface in interfaces {
        let mut line = format!("{:<name_width$} {:<ip_width$} {:<status_width$}", interface.interface_name, interface.ip_addr, interface.status, name_width = name_alignment, ip_width = 14, status_width = 3);
        if args.mac {
            let mac = format!("{:<mac_width$}", interface.mac_addr, mac_width = 10);
            line.push_str(&mac);
        }

        println!("{line}");
    }
}
