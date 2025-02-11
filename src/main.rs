use clap::Parser;

mod interface_data;
mod formatting;

/// Application that prints your network interfaces with associated information, such as ipv4 address, status etc
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// List mac addresses for the interfaces
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


fn main() {
    let args = Args::parse();
    let interfaces = interface_data::get_interface_data();
    let output = formatting::get_formatted_output(args, interfaces);

    for line in output {
        println!("{line}");
    }
}
