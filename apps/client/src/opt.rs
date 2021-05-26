use std::str::FromStr;
use structopt::StructOpt;

use std::net::SocketAddr;

use consoles::controller::platform::Platform;

fn parse_socket(s: &str) -> Result<std::net::SocketAddr, std::net::AddrParseError> {
    if s.contains(':') {
        SocketAddr::from_str(s)
    } else {
        let sock = s.to_owned() + ":4242";
        SocketAddr::from_str(&sock)
    }
}

#[derive(StructOpt, Debug)]
pub struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// IP and port to connect to (if no port is specified, defaults to 4242)
    #[structopt(name = "IP", parse(try_from_str = parse_socket))]
    pub server: SocketAddr,

    /// Which console platform to emulate
    #[structopt(name = "PLATFORM")]
    pub platform: Platform,
}
