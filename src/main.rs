use clap::Parser;
use nix::ifaddrs;

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

fn main() {
    let args = Args::parse();
    get_interfaces(args);
}

fn get_interfaces(args: Args) {
    if args.mac {
        println!("Mac argument is set!");
    }

    let if_addrs = match ifaddrs::getifaddrs() {
        Ok(addrs) => addrs,
        Err(e) => {
            println!("Could not get network interfaces: {e}");
            return;
        }
    };

    for if_addr in if_addrs {
        //println!("{:?}, {:?}, {:?}, {:?}", if_addr.interface_name, if_addr.address, if_addr.netmask, if_addr.flags);
        println!("{}", if_addr.interface_name);
    }
}
