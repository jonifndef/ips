use crate::interface_data;
use crate::colors;

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add test for this command:
    // cargo run -- -m -c
    // we get a blank line after the first line, even though we are not printing the ipv6
    // addrs, I assume this is the culprit since this should (?) be the only field that gives us
    // multiple lines in this command

    #[test]
    fn test_one_formatting_ipv6() {
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

        assert!(output[0].len() == 125);
        assert!(output[1].len() == 98);
        assert!(output[2].len() == 98);
        assert!(output[3].len() == 125);
        assert!(output[4].len() == 98);
        assert!(output[5].len() == 98);
        assert!(output[6].len() == 125);
        assert!(output[7].len() == 98);
        assert!(output[8].len() == 98);
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

        assert!(output[0].len() == 143);
        assert!(output[1].len() == 116);
        assert!(output[2].len() == 116);
        assert!(output[3].len() == 134);
        assert!(output[4].len() == 107);
        assert!(output[5].len() == 107);
        assert!(output[6].len() == 143);
        assert!(output[7].len() == 107);
        assert!(output[8].len() == 107);
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

        assert!(output[0].len() == 179);
        assert!(output[1].len() == 125);
        assert!(output[2].len() == 170);
        assert!(output[3].len() == 170);
    }
}

pub fn get_formatted_output(args: crate::Args, mut interfaces: Vec<interface_data::InterfaceData>) -> Vec<String> {
    let widths = interface_data::get_field_widths(&interfaces, &args);

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
                let data = interface.get(col, line_num);
                let whitespace = widths.get(col) - data.len();
                line.push_str(data);
                line.push_str(
                    &format!(
                        "{:>width$}",
                        "",
                        width = if data == "" && !args.nocolor {
                            whitespace - colors::ColorTokens::TOKENS_LEN
                        } else {
                            whitespace
                        }
                    )
                );
            }

            lines_for_interface.push(line);
        }

        lines.append(&mut lines_for_interface);
    }

    return lines;
}

fn get_colorized_interfaces_data(interfaces: Vec<interface_data::InterfaceData>) -> Vec<interface_data::InterfaceData> {
    interfaces.into_iter().map(
        |interface| {
            interface_data::InterfaceData {
                interface_name: format!("{}{}{}", colors::ColorTokens::GREEN, interface.interface_name, colors::ColorTokens::ENDING),
                ip_addr: format!("{}{}{}", colors::ColorTokens::YELLOW, interface.ip_addr, colors::ColorTokens::ENDING),
                status: format!("{}{}{}", colors::ColorTokens::RED, interface.status, colors::ColorTokens::ENDING),
                mac_addr: format!("{}{}{}", colors::ColorTokens::BRIGHT_GREEN, interface.mac_addr, colors::ColorTokens::ENDING),
                ipv6_addrs: interface.ipv6_addrs.into_iter().map(
                    |ipv6_addr| {
                        format!("{}{}{}", colors::ColorTokens::CYAN, ipv6_addr, colors::ColorTokens::ENDING)
                    }
                ).collect(),
                gateway: format!("{}{}{}", colors::ColorTokens::MAGENTA, interface.gateway, colors::ColorTokens::ENDING),
                connections: interface.connections.into_iter().map(
                    |connection| {
                        format!("{}{}{}", colors::ColorTokens::BLUE, connection, colors::ColorTokens::ENDING)
                    }
                ).collect()
            }
        }
    ).collect()
}

fn get_chosen_cols(args: &crate::Args) -> Vec<interface_data::IfcField> {
    let mut cols = vec![
        interface_data::IfcField::NAME,
        interface_data::IfcField::IP,
        interface_data::IfcField::STATUS
    ];

    if args.mac {
        cols.push(interface_data::IfcField::MAC);
    }

    if args.ipv6 {
        cols.push(interface_data::IfcField::IPV6);
    }

    if args.gateway {
        cols.push(interface_data::IfcField::GW);
    }

    if args.connections {
        cols.push(interface_data::IfcField::CONN);
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

