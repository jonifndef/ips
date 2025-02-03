use crate::interface_data;

struct OutputWidths {
    interface_name: usize,
    ip_addr: usize,
    status: usize,
    mac: usize,
    ipv6: usize,
    gateway: usize,
    connections: usize
}

struct ColorTokens {

}

impl ColorTokens {
    const ENDING: &str = "\x1b[0m";
    const GREEN: &str = "\x1b[32m";
    const BRIGHT_GREEN: &str = "\x1b[92m";
    const YELLOW: &str = "\x1b[33m";
    const RED: &str = "\x1b[31m";
    const BLUE: &str = "\x1b[34m";
    const TOKENS_LEN: usize = 9;
}

pub fn print_interfaces(args: crate::Args, interfaces: &mut Vec<interface_data::InterfaceData>) {
    let widths = get_output_widths(&interfaces);

    for interface in interfaces {
        if !args.nocolor {
            colorize_data_strings(interface);
        }

        let mut line = format!("{:<name_width$} {:<ip_width$} {:<status_width$}",
            interface.interface_name,
            interface.ip_addr,
            interface.status,
            name_width = widths.interface_name + if args.nocolor { 0 } else { ColorTokens::TOKENS_LEN },
            ip_width = widths.ip_addr + if args.nocolor { 0 } else { ColorTokens::TOKENS_LEN },
            status_width = widths.status + if args.nocolor { 0 } else { ColorTokens::TOKENS_LEN });

        if args.mac {
            let mac = format!("{:<mac_width$}",
                interface.mac_addr,
                mac_width = widths.mac + if args.nocolor { 0 } else { ColorTokens::TOKENS_LEN });
            line.push_str(&mac);
        }

        if args.gateway {
            let gateway = format!("{:<gateway_width$}",
                interface.gateway,
                gateway_width = widths.gateway + if args.nocolor { 0 } else { ColorTokens::TOKENS_LEN });
            line.push_str(&gateway);
        }

        println!("{line}");
    }
}

fn colorize_data_strings(data: &mut interface_data::InterfaceData) {
    data.interface_name = format!(
        "{}{}{}",
        ColorTokens::GREEN, data.interface_name, ColorTokens::ENDING
    );
    data.ip_addr = format!(
        "{}{}{}",
        ColorTokens::YELLOW, data.ip_addr, ColorTokens::ENDING
    );
    data.status = format!(
        "{}{}{}",
        ColorTokens::BRIGHT_GREEN, data.status, ColorTokens::ENDING
    );
    data.mac_addr = format!(
        "{}{}{}",
        ColorTokens::RED, data.mac_addr, ColorTokens::ENDING
    );
    data.gateway = format!(
        "{}{}{}",
        ColorTokens::BLUE, data.gateway, ColorTokens::ENDING
    );
}

fn get_output_widths(interfaces: &[interface_data::InterfaceData]) -> OutputWidths {
    let mut widths = OutputWidths {
        interface_name: 0,
        ip_addr: 14,
        status: 3,
        mac: 18,
        ipv6: 0,
        gateway: 14,
        connections: 0
    };

    for interface in interfaces {
        if interface.interface_name.len() > widths.interface_name {
            widths.interface_name = interface.interface_name.len();
        }

        for ipv6 in &interface.ipv6_addrs {
            if ipv6.len() > widths.ipv6 {
                widths.ipv6 = ipv6.len();
            }
        }

        for conn in &interface.connections {
            if conn.len() > widths.connections {
                widths.connections = conn.len();
            }
        }
    }

    return widths
}

