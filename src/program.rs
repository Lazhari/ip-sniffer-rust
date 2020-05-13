use std::net::IpAddr;
use std::str::FromStr;

pub struct Arguments {
  pub flag: String,
  pub ip_addr: IpAddr,
  pub threads: u16,
}

impl Arguments {
  pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
    if args.len() < 2 {
      return Err("not enough arguments");
    } else if args.len() > 4 {
      return Err("too many arguments");
    }
    let f = args[1].clone();
    if let Ok(ip_addr) = IpAddr::from_str(&f) {
      return Ok(Arguments {
        flag: String::from(""),
        ip_addr,
        threads: 4,
      });
    } else {
      let flag = args[1].clone();
      if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
        println!(
          "Usage: -j to select the number threads of threads
              \r\n -h or -help to show the help message"
        );
        Err("help")
      } else if flag.contains("-h") || flag.contains("-help") {
        Err("too many arguments")
      } else if flag.contains("-j") {
        let ip_addr = match IpAddr::from_str(&args[3]) {
          Ok(s) => s,
          Err(_) => return Err("Not a valid IP Address; must be IPv4 or IPv6"),
        };
        let threads: u16 = match args[2].trim().parse() {
          Ok(s) => s,
          Err(_) => return Err("Failed to parse thread number"),
        };
        return Ok(Arguments {
          threads,
          flag,
          ip_addr,
        });
      } else {
        return Err("Invalid syntax");
      }
    }
  }
}
