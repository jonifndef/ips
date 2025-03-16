use super::*;

#[test]
fn test_mac() {
    let args = crate::Args {
        mac: true,
        ipv6: false,
        gateway: false,
        connections: false,
        nocolor: false
    };

    let interfaces = vec![
        interface_data::InterfaceData {
            interface_name: String::from("ollebolle"),
            ip_addr: String::from("192.168.1.2/24"),
            mac_addr: String::from("AA:BB:CC:DD:EE:FF"),
            status: String::from("UP"),
            ipv6_addrs: vec![
                String::from("fdaa:bbcc:ddee:0:9347:deb9:2fa3:82a3/64"),
                String::from("fe80::5fa:c189:2ae:94a2/64"),
            ],
            gateway: String::from(""),
            connections: vec![
                String::from("my_connection_yao")
            ]
        },
        interface_data::InterfaceData {
            interface_name: String::from("ollebolle_the_second"),
            ip_addr: String::from("192.168.1.3/24"),
            mac_addr: String::from("00:11:22:33:44:55"),
            status: String::from("UP"),
            ipv6_addrs: vec![],
            gateway: String::from(""),
            connections: vec![
                String::from("another_connection_yao")
            ]
        },
        interface_data::InterfaceData {
            interface_name: String::from("ollebolle_yo"),
            ip_addr: String::from("192.168.1.4/24"),
            mac_addr: String::from("C0:FF:EE:BA:BE:00"),
            status: String::from("UP"),
            ipv6_addrs: vec![],
            gateway: String::from(""),
            connections: vec![
                String::from("another_connection_yao")
            ]
        }
    ];

    let output = get_formatted_output(args, interfaces);

    assert!(output.len() == 3);
}

#[test]
fn test_mac_and_connections() {
    let args = crate::Args {
        mac: true,
        ipv6: false,
        gateway: false,
        connections: true,
        nocolor: false
    };

    let interfaces = vec![
        interface_data::InterfaceData {
            interface_name: String::from("ollebolle"),
            ip_addr: String::from("192.168.1.2/24"),
            mac_addr: String::from("AA:BB:CC:DD:EE:FF"),
            status: String::from("UP"),
            ipv6_addrs: vec![
                String::from("fdaa:bbcc:ddee:0:9347:deb9:2fa3:82a3/64"),
                String::from("fe80::5fa:c189:2ae:94a2/64"),
            ],
            gateway: String::from(""),
            connections: vec![
                String::from("my_connection_yao")
            ]
        },
        interface_data::InterfaceData {
            interface_name: String::from("ollebolle_the_second"),
            ip_addr: String::from("192.168.1.3/24"),
            mac_addr: String::from("00:11:22:33:44:55"),
            status: String::from("UP"),
            ipv6_addrs: vec![],
            gateway: String::from(""),
            connections: vec![
                String::from("another_connection_yao")
            ]
        },
        interface_data::InterfaceData {
            interface_name: String::from("ollebolle_yo"),
            ip_addr: String::from("192.168.1.4/24"),
            mac_addr: String::from("C0:FF:EE:BA:BE:00"),
            status: String::from("UP"),
            ipv6_addrs: vec![],
            gateway: String::from(""),
            connections: vec![
                String::from("another_connection_yao")
            ]
        }
    ];

    let output = get_formatted_output(args, interfaces);

    assert!(output.len() == 3);
}

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
