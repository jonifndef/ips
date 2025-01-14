use clap::Parser;
use sysinfo::Networks;

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

    if args.mac {
        println!("Mac argument is set!");
    }

    let networks = Networks::new_with_refreshed_list();
    for (interface_name, network) in &networks {
        //println!("[{interface_name}]: {network:?}");
        println!("{interface_name}");
        //network.ip_networks()
    }
}
