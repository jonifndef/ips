use clap::Parser;
use sysinfo::Networks;
use std::net::IpAddr;

/// Application that prints your network interfaces with associated information, such as ipv4 address, status etc
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    //List mac addresses for the interfaces
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
    connections: bool
}

fn main() {
    let args = Args::parse();
    get_interfaces(args);
}

fn get_interfaces(args: Args) {
    if args.mac {
        println!("Mac argument is set!");
    }

    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        println!("Interface: {interface_name}");
        let ip_networks = data.ip_networks();
        for ip_network in ip_networks {
            let addr = &ip_network.addr;
            match addr {
                IpAddr::V4(ipv4) => {
                    println!("IPv4: {}", ipv4);
                }
                IpAddr::V6(ipv6) => {
                    println!("IPv6: {}", ipv6);
                }
            }
        }
        let mac_addr = data.mac_address();
        println!("mac addr: {mac_addr}");
    }
}
