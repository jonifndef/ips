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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatting_ipv6() {
        let args = crate::Args {
            mac: false,
            ipv6: true,
            gateway: false,
            connections: false,
            nocolor: false
        };

        let interfaces = vec![
            interface_data::InterfaceData {
                interface_name: String::from("ollebolle"),
                ip_addr: String::from("192.168.1.2/24"),
                mac_addr: String::from(""),
                status: String::from("UP"),
                ipv6_addrs: vec![
                    String::from("fdaa:bbcc:ddee:0:9347:deb9:2fa3:82a3/64"),
                    String::from("fe80::5fa:c189:2ae:94a2/64"),
                    String::from("fa20:bbdd:c189:2ae:94:a21f:b71d/64"),
                ],
                gateway: String::from(""),
                connections: vec![]
            },
            interface_data::InterfaceData {
                interface_name: String::from("ollebolle_the_second"),
                ip_addr: String::from("192.168.1.3/24"),
                mac_addr: String::from(""),
                status: String::from("UP"),
                ipv6_addrs: vec![
                    String::from("fe80::5fa:c189:2ae:94a2/64"),
                    String::from("fdaa:bbcc:ddee:0:9347:deb9:2fa3:82a3:bbff:aaee/64"),
                    String::from("fa20:bbdd:c189:2ae:94:a21f:b71d/64"),
                ],
                gateway: String::from(""),
                connections: vec![]
            },
            interface_data::InterfaceData {
                interface_name: String::from("ollebolle_yo"),
                ip_addr: String::from("192.168.1.4/24"),
                mac_addr: String::from(""),
                status: String::from("UP"),
                ipv6_addrs: vec![
                    String::from("fe80::5fa::::94a2/64"),
                    String::from("fa20:bbdd:c189:2ae:94:a21f/64"),
                    String::from("fdaa:bbcc:ddee:0:9347:deb9:2fa3:82a3:ccff/64"),
                ],
                gateway: String::from(""),
                connections: vec![]
            }
        ];

        let output = get_formatted_output(args, interfaces);
        //for line in output.iter() {
        //    println!("output len {}", line.len());
        //    println!("{line}");
        //}
        assert!(output[0].len() == 124);
    }

    #[test]
    fn test_formatting_ipv6_connections() {
        let args = crate::Args {
            mac: false,
            ipv6: true,
            gateway: false,
            connections: true,
            nocolor: false
        };

        let interfaces = vec![
            interface_data::InterfaceData {
                interface_name: String::from("ollebolle"),
                ip_addr: String::from("192.168.1.2/24"),
                mac_addr: String::from(""),
                status: String::from("UP"),
                ipv6_addrs: vec![
                    String::from("fdaa:bbcc:ddee:0:9347:deb9:2fa3:82a3/64"),
                    String::from("fe80::5fa:c189:2ae:94a2/64"),
                    String::from("fa20:bbdd:c189:2ae:94:a21f:b71d/64"),
                ],
                gateway: String::from(""),
                connections: vec![
                    String::from("hejconbacon"),
                    String::from("olle_connection_yo"),
                    String::from("mmmm"),
                ]
            },
            interface_data::InterfaceData {
                interface_name: String::from("ollebolle_the_second"),
                ip_addr: String::from("192.168.1.3/24"),
                mac_addr: String::from(""),
                status: String::from("UP"),
                ipv6_addrs: vec![],
                gateway: String::from(""),
                connections: vec![
                    String::from("olle_connection_yo"),
                    String::from("hejconbacon"),
                    String::from("mmmm"),
                ]
            },
            interface_data::InterfaceData {
                interface_name: String::from("ollebolle_yo"),
                ip_addr: String::from("192.168.1.4/24"),
                mac_addr: String::from(""),
                status: String::from("UP"),
                ipv6_addrs: vec![
                    String::from("fe80::5fa::::94a2/64"),
                ],
                gateway: String::from(""),
                connections: vec![
                    String::from("mmmm"),
                    String::from("hejconbacon"),
                    String::from("olle_connection_yo"),
                ]
            }
        ];

        let output = get_formatted_output(args, interfaces);
        for line in output.iter() {
        //    println!("output len {}", line.len());
        //    println!("{line}");
        }
        assert!(true);
    }

    #[test]
    fn test_formatting_mac_ipv6_gw_connection() {
        let args = crate::Args {
            mac: true,
            ipv6: true,
            gateway: true,
            connections: true,
            nocolor: false
        };

        let interfaces = vec![
            interface_data::InterfaceData {
                interface_name: String::from("ollebolle"),
                ip_addr: String::from("192.168.1.2/24"),
                mac_addr: String::from("02:42:2f:ff:49:ae"),
                status: String::from("UP"),
                ipv6_addrs: vec![
                    String::from("fdaa:bbcc:ddee:0:9347:deb9:2fa3:82a3/64"),
                    String::from("fe80::5fa:c189:2ae:94a2/64"),
                ],
                gateway: String::from(""),
                connections: vec![
                    String::from("hejconbacon"),
                ]
            },
            interface_data::InterfaceData {
                interface_name: String::from("ollebolle_the_second"),
                ip_addr: String::from("192.168.1.3/24"),
                mac_addr: String::from("03:42:2f:ff:49:ae"),
                status: String::from("UP"),
                ipv6_addrs: vec![],
                gateway: String::from(""),
                connections: vec![
                    String::from("olle_connection_yo"),
                ]
            },
            interface_data::InterfaceData {
                interface_name: String::from("ollebolle_yo"),
                ip_addr: String::from("192.168.1.4/24"),
                mac_addr: String::from("04:42:2f:ff:49:ae"),
                status: String::from("UP"),
                ipv6_addrs: vec![],
                gateway: String::from(""),
                connections: vec![
                    String::from("mmmm"),
                ]
            }
        ];

        let output = get_formatted_output(args, interfaces);
        for line in output.iter() {
        //    println!("output len {}", line.len());
            println!("{line}");
        }
        assert!(true);
    }
}

pub fn get_formatted_output(args: crate::Args, interfaces: Vec<interface_data::InterfaceData>) -> Vec<String> {
    let widths = get_output_widths(&interfaces);
    let mut lines: Vec<String> = vec![];

    for interface in interfaces {
        let i = lines.len();
        let line = format!("{:<name_width$} {:<ip_width$} {:<status_width$}",
            colorize_string_if_enabled(&interface.interface_name, args.nocolor, ColorTokens::GREEN),
            colorize_string_if_enabled(&interface.ip_addr, args.nocolor, ColorTokens::YELLOW),
            colorize_string_if_enabled(&interface.status, args.nocolor, ColorTokens::BRIGHT_GREEN),
            name_width = widths.interface_name + if args.nocolor { 0 } else { ColorTokens::TOKENS_LEN },
            ip_width = widths.ip_addr + if args.nocolor { 0 } else { ColorTokens::TOKENS_LEN },
            status_width = widths.status + if args.nocolor { 0 } else { ColorTokens::TOKENS_LEN });
        lines.push(line);

        if args.mac {
            let mac = format!(" {:<mac_width$}",
                colorize_string_if_enabled(&interface.mac_addr, args.nocolor, ColorTokens::RED),
                mac_width = widths.mac + if args.nocolor { 0 } else { ColorTokens::TOKENS_LEN });
            if let Some(line) = lines.get_mut(i) {
                line.push_str(&mac);
            }
        }

        if args.ipv6 {
            let mut ipv6_idx = i;
            for addr in interface.ipv6_addrs.iter() {
                let ipv6 = format!(" {:<ipv6_width$}",
                    colorize_string_if_enabled(addr, args.nocolor, ColorTokens::BLUE),
                    ipv6_width = widths.ipv6 + if args.nocolor { 0 } else { ColorTokens::TOKENS_LEN });
                if let Some(line) = lines.get_mut(ipv6_idx) {
                    // If there are columns that are not printed for a interface here, e.g. mac, we
                    // need to prepend whitespace to fit the columns, like we do below with the
                    // 'let prefix = format!("{prefix_width$}", "",...' statement
                    line.push_str(&ipv6);
                    ipv6_idx += 1;
                } else {
                    let mut ipv6 = format!(" {:<ipv6_width$}",
                        colorize_string_if_enabled(addr, args.nocolor, ColorTokens::BLUE),
                        ipv6_width = widths.ipv6 + if args.nocolor { 0 } else { ColorTokens::TOKENS_LEN });
                    let width = widths.interface_name + 1 + widths.ip_addr + 1 + widths.status + if args.mac { widths.mac + 1 } else { 0 };
                    let prefix = format!("{:<prefix_width$}", "", prefix_width = width);
                    ipv6.insert_str(0, &prefix);
                    lines.push(ipv6);
                    ipv6_idx += 1;
                }
            }
        }

        if args.connections {
            let mut conn_idx = i;
            for addr in interface.connections.iter() {
                let connection = format!(" {:<connection_width$}",
                    colorize_string_if_enabled(addr, args.nocolor, ColorTokens::BLUE),
                    connection_width = widths.connections + if args.nocolor { 0 } else { ColorTokens::TOKENS_LEN });
                if let Some(line) = lines.get_mut(conn_idx) {
                    line.push_str(&connection);
                    conn_idx += 1;
                } else {
                    let mut connection = format!(" {:<connection_width$}",
                        colorize_string_if_enabled(addr, args.nocolor, ColorTokens::BLUE),
                        connection_width = widths.connections + if args.nocolor { 0 } else { ColorTokens::TOKENS_LEN });
                    let width = widths.interface_name + 1 + widths.ip_addr + 1 + widths.status + if args.mac { widths.mac + 1 } else { 0 } + if args.ipv6 { widths.ipv6 + 1 } else { 0 };
                    let prefix = format!("{:<prefix_width$}", "", prefix_width = width);
                    connection.insert_str(0, &prefix);
                    lines.push(connection);
                    conn_idx += 1;
                }
            }
        }
    }

    return lines;
}

fn colorize_string_if_enabled(input: &String, nocolor: bool, color: &str) -> String {
    if nocolor {
        return input.clone();
    } else {
        return format!(
            "{}{}{}",
            color, input, ColorTokens::ENDING
        );
    }
}

fn get_output_widths(interfaces: &[interface_data::InterfaceData]) -> OutputWidths {
    let mut widths = OutputWidths {
        interface_name: 0,
        ip_addr: 14,
        status: 0,
        mac: 17,
        ipv6: 0,
        gateway: 11,
        connections: 0
    };

    for interface in interfaces {
        if interface.interface_name.len() > widths.interface_name {
            widths.interface_name = interface.interface_name.len();
        }

        if interface.status.len() > widths.status {
            widths.status = interface.status.len();
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

