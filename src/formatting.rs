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
    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const YELLOW: &str = "\x1b[33m";
    const BLUE: &str = "\x1b[34m";
    const MAGENTA: &str = "\x1b[35m";
    const CYAN: &str = "\x1b[36m";
    const BRIGHT_RED: &str = "\x1b[91m";
    const BRIGHT_GREEN: &str = "\x1b[92m";
    const BRIGHT_YELLOW: &str = "\x1b[93m";
    const BRIGHT_BLUE: &str = "\x1b[94m";
    const ENDING: &str = "\x1b[0m";
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

        assert!(output[0].len() == 124);
        assert!(output[1].len() == 97);
        assert!(output[2].len() == 97);
        assert!(output[3].len() == 124);
        assert!(output[4].len() == 97);
        assert!(output[5].len() == 97);
        assert!(output[6].len() == 124);
        assert!(output[7].len() == 97);
        assert!(output[8].len() == 97);
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

        assert!(output[0].len() == 142);
        assert!(output[1].len() == 115);
        assert!(output[2].len() == 115);
        assert!(output[3].len() == 133);
        assert!(output[4].len() == 106);
        assert!(output[5].len() == 106);
        assert!(output[6].len() == 142);
        assert!(output[7].len() == 106);
        assert!(output[8].len() == 106);
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

        assert!(output[0].len() == 169);
        assert!(output[1].len() == 105);
        assert!(output[2].len() == 160);
        assert!(output[3].len() == 160);
    }
}

pub fn get_formatted_output(args: crate::Args, mut interfaces: Vec<interface_data::InterfaceData>) -> Vec<String> {
    let widths = get_output_widths(&interfaces, &args);

    if !args.nocolor {
        interfaces = get_colorized_interfaces_data(interfaces);
    }

    let chosen_cols = get_chosen_cols(&args);
    let mut lines: Vec<String> = vec![];

    for interface in interfaces {
        let num_lines_for_interface = get_num_lines(&interface);
        let mut lines_for_interface: Vec<String> = vec![];

        for line_num in 0..num_lines_for_interface {
            let mut line = String::default();
            for col in &chosen_cols {
                let data = get_data_for_col(&interface, line_num, col);
                line.push_str(data);
            }

            lines_for_interface.push(line);
        }

        lines.append(&mut lines_for_interface);
    }

    return lines;
}

fn get_data_for_col<'a>(interface: &'a interface_data::InterfaceData, linenum: usize, col: &str) -> &'a str {
    let data: &str = match col {
        "interface_name" => if linenum == 0 { interface.interface_name.as_str() } else { "" },
        "ip_addr" => if linenum == 0 { interface.ip_addr.as_str() } else { "" },
        "status" => if linenum == 0 { interface.status.as_str() } else { "" },
        "mac_addr" => if linenum == 0 { interface.mac_addr.as_str() } else { "" },
        "ipv6_addrs" => if let Some(addr) = interface.ipv6_addrs.get(linenum) { addr.as_str() } else { "" },
        "gateway" => if linenum == 0 { interface.gateway.as_str() } else { "" },
        "connections" => if let Some(connection) = interface.connections.get(linenum) { connection.as_str() } else { "" },
        _ => ""
    };

    data
}

fn get_colorized_interfaces_data(interfaces: Vec<interface_data::InterfaceData>) -> Vec<interface_data::InterfaceData> {
    interfaces.into_iter().map(
        |interface| {
            interface_data::InterfaceData {
                interface_name: format!("{}{}{}", ColorTokens::GREEN, interface.interface_name, ColorTokens::ENDING),
                ip_addr: format!("{}{}{}", ColorTokens::YELLOW, interface.ip_addr, ColorTokens::ENDING),
                status: format!("{}{}{}", ColorTokens::RED, interface.status, ColorTokens::ENDING),
                mac_addr: format!("{}{}{}", ColorTokens::BRIGHT_GREEN, interface.mac_addr, ColorTokens::ENDING),
                ipv6_addrs: interface.ipv6_addrs.into_iter().map(
                    |ipv6_addr| {
                        format!("{}{}{}", ColorTokens::CYAN, ipv6_addr, ColorTokens::ENDING)
                    }
                ).collect(),
                gateway: format!("{}{}{}", ColorTokens::MAGENTA, interface.gateway, ColorTokens::ENDING),
                connections: interface.connections.into_iter().map(
                    |connection| {
                        format!("{}{}{}", ColorTokens::BLUE, connection, ColorTokens::ENDING)
                    }
                ).collect()
            }
        }
    ).collect()
}

fn get_chosen_cols(args: &crate::Args) -> Vec<&str> {
    let mut cols = vec![
        "interface_name",
        "ip_addr",
        "status"
    ];

    if args.mac {
        cols.push("mac_addr");
    }

    if args.ipv6 {
        cols.push("ipv6_addrs");
    }

    if args.gateway {
        cols.push("gateway");
    }

    if args.connections {
        cols.push("connections");
    }

    cols
}

fn get_num_lines(interface_data: &interface_data::InterfaceData) -> usize {
    let mut num_lines = 0;
    if interface_data.ipv6_addrs.len() > num_lines {
        num_lines = interface_data.ipv6_addrs.len();
    }
    if interface_data.connections.len() > num_lines {
        num_lines = interface_data.connections.len();
    }

    num_lines
}

fn get_output_widths(interfaces: &[interface_data::InterfaceData], args: &crate::Args) -> OutputWidths {
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

    if !args.nocolor {
        widths.interface_name += ColorTokens::TOKENS_LEN;
        widths.ip_addr += ColorTokens::TOKENS_LEN;
        widths.status += ColorTokens::TOKENS_LEN;
        widths.mac += ColorTokens::TOKENS_LEN;
        widths.ipv6 += ColorTokens::TOKENS_LEN;
        widths.gateway += ColorTokens::TOKENS_LEN;
        widths.connections += ColorTokens::TOKENS_LEN;
    }

    return widths
}

