use clap::Parser;
//use nix::ifaddrs;
use pnet::datalink::{self, NetworkInterface};
use pnet::ipnetwork::IpNetwork;

// Use this (nix crate), or MAAAYBE pnet crate?

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

struct OutputFields {
    interface_name: String,
    ip_addr: String,
    mac_addr: String,
    status: String,
    ipv6_addr: String,
    gateway: String,
    connections: String
}

fn main() {
    let args = Args::parse();
    get_interfaces(args);
}

fn get_interfaces(args: Args) {
    if args.mac {
        println!("Mac argument is set!");
    }

    let interfaces = pnet::datalink::interfaces();
    for interface in interfaces {
        if interface.is_loopback() {
            continue;
        }
        if interface.is_up() {
            println!("its up");
        }
        println!("interface name is: {}", interface.name);
        for ip in interface.ips {
            match ip {
                IpNetwork::V4(ip_addr) => { println!("Ipv4 addr: {}", ip_addr) }
                IpNetwork::V6(ip_addr) => { println!("Ipv6 addr: {}", ip_addr) }
            }
        }

        println!("{:?}", interface.mac);
    }

    //let if_addrs = match ifaddrs::getifaddrs() {
    //    Ok(addrs) => addrs,
    //    Err(e) => {
    //        println!("Could not get network interfaces: {e}");
    //        return;
    //    }
    //};

    //for if_addr in if_addrs {
    //    //println!("{:?}, {:?}, {:?}, {:?}", if_addr.interface_name, if_addr.address, if_addr.netmask, if_addr.flags);
    //    println!("{}", if_addr.interface_name);
    //}
}
