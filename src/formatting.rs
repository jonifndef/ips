use crate::colors;
use crate::interface_data;

pub fn get_formatted_output(
    args: crate::Args,
    mut interfaces: Vec<interface_data::InterfaceData>,
) -> Vec<String> {
    let widths = interface_data::get_field_widths(&interfaces, &args);

    if !args.nocolor {
        interfaces = get_colorized_interfaces_data(interfaces);
    }

    let chosen_cols = get_chosen_cols(&args);
    let mut lines: Vec<String> = vec![];

    for interface in interfaces {
        let num_lines_for_interface = get_num_lines(&interface, &args);
        let mut lines_for_interface: Vec<String> = vec![];

        for line_num in 0..num_lines_for_interface {
            let mut line = String::default();
            for col in &chosen_cols {
                let data = interface.get(col, line_num);
                let col_width = widths.get(col).unwrap();
                let whitespace = col_width - data.len();
                line.push_str(data);
                line.push_str(&format!(
                    "{:>width$}",
                    "",
                    width = if data.is_empty() && !args.nocolor {
                        whitespace - colors::ColorTokens::TOKENS_LEN
                    } else {
                        whitespace
                    }
                ));
            }

            lines_for_interface.push(line);
        }

        lines.append(&mut lines_for_interface);
    }

    lines
}

fn get_colorized_interfaces_data(
    interfaces: Vec<interface_data::InterfaceData>,
) -> Vec<interface_data::InterfaceData> {
    interfaces
        .into_iter()
        .map(|interface| interface_data::InterfaceData {
            interface_name: format!(
                "{}{}{}",
                colors::ColorTokens::GREEN,
                interface.interface_name,
                colors::ColorTokens::ENDING
            ),
            ip_addr: format!(
                "{}{}{}",
                colors::ColorTokens::YELLOW,
                interface.ip_addr,
                colors::ColorTokens::ENDING
            ),
            status: format!(
                "{}{}{}",
                colors::ColorTokens::RED,
                interface.status,
                colors::ColorTokens::ENDING
            ),
            mac_addr: format!(
                "{}{}{}",
                colors::ColorTokens::BRIGHT_GREEN,
                interface.mac_addr,
                colors::ColorTokens::ENDING
            ),
            ipv6_addrs: interface
                .ipv6_addrs
                .into_iter()
                .map(|ipv6_addr| {
                    format!(
                        "{}{}{}",
                        colors::ColorTokens::CYAN,
                        ipv6_addr,
                        colors::ColorTokens::ENDING
                    )
                })
                .collect(),
            gateway: format!(
                "{}{}{}",
                colors::ColorTokens::MAGENTA,
                interface.gateway,
                colors::ColorTokens::ENDING
            ),
            connections: interface
                .connections
                .into_iter()
                .map(|connection| {
                    format!(
                        "{}{}{}",
                        colors::ColorTokens::BLUE,
                        connection,
                        colors::ColorTokens::ENDING
                    )
                })
                .collect(),
        })
        .collect()
}

fn get_chosen_cols(args: &crate::Args) -> Vec<interface_data::IfcField> {
    let mut cols = vec![
        interface_data::IfcField::Name,
        interface_data::IfcField::Ip,
        interface_data::IfcField::Status,
    ];

    if args.mac {
        cols.push(interface_data::IfcField::Mac);
    }

    if args.ipv6 {
        cols.push(interface_data::IfcField::Ipv6);
    }

    if args.gateway {
        cols.push(interface_data::IfcField::Gw);
    }

    if args.connections {
        cols.push(interface_data::IfcField::Conn);
    }

    cols
}

fn get_num_lines(interface_data: &interface_data::InterfaceData, args: &crate::Args) -> usize {
    let mut num_lines = 1;
    if args.ipv6 && interface_data.ipv6_addrs.len() > num_lines {
        num_lines = interface_data.ipv6_addrs.len();
    }

    if args.connections && interface_data.connections.len() > num_lines {
        num_lines = interface_data.connections.len();
    }

    num_lines
}

#[cfg(test)]
mod tests;
