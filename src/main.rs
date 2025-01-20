use clap::Parser;
use pnet::datalink;
use pnet::ipnetwork::IpNetwork;

// to get gateway, use something like:
// route -n | grep 'UG[ \t]' | awk '{print $2}'
// or:
// route -n | grep 'UG[ \t]' | grep 'wlp2s0' | awk '{print $2}'
// to get connections that use this interface, run:
// nmcli -t con show | grep "wlp2s0", and parse the first field

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
    connections: bool,

    /// Disable colored output
    #[arg(short, long)]
    nocolor: bool
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

    let interfaces = datalink::interfaces();
    for interface in interfaces {
        if interface.is_loopback() {
            continue;
        }
        println!("interface name is: {}", interface.name);
        for ip in &interface.ips {
            match ip {
                IpNetwork::V4(ip_addr) => { println!("Ipv4 addr: {}", ip_addr) }
                IpNetwork::V6(ip_addr) => { println!("Ipv6 addr: {}", ip_addr) }
            }
        }

        println!("{:?}", interface.mac);

        if interface.is_up() {
            println!("its up");
        }
    }
}
