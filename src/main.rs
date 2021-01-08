use std::io::{self, Write};
use std::time::Duration;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::channel;

use threadpool::ThreadPool;
use clap::{Arg, App};

const MAX_PORT: u16 = 65535;

fn main() -> Result< () , String> {
    let matches = App::new("Port Sniffer")
        .version("0.1.0")
        .author("Ben Lorenz <bnlrnz@gmail.com>")
        .about("Port sniffer written in Rust. See https://github.com/bnlrnz/port_sniff")
        .arg(Arg::with_name("ip_addr")
                .takes_value(true)
                .required_unless("help")
                .help("a valid IPv4 or IPv6 address"))
        .arg(Arg::with_name("threads")
                .short("t")
                .long("threads")
                .takes_value(true)
                .default_value("4")
                .help("number of threads to spawn"))
        .get_matches();
    
    let ip_addr = match IpAddr::from_str(matches.value_of("ip_addr").unwrap()){
        Ok(ip_addr) => ip_addr,
        Err(_) => return Err("Not a valid IPv4 or IPv6 address".to_string()),
    };

    let num_threads = match matches.value_of("threads").unwrap().parse::<u16>(){
        Ok(threads) => threads,
        _ => 4, // this should not happen, cause clap defaults to 4
    };

    let pool = ThreadPool::new(num_threads as usize);

    let (tx, rx) = channel::<u16>();

    print!("Scanning ports");
    for port in 1..=MAX_PORT {
        let tx = tx.clone();
        pool.execute(move || {
            let sock_addr = SocketAddr::new(ip_addr, port);
            match TcpStream::connect_timeout(&sock_addr, Duration::from_secs(5)){
                Ok(_) => {
                    print!(".");
                    io::stdout().flush().unwrap();
                    tx.send(port).unwrap();
                },
                Err(_) => {}
            }
        });
    }

    let mut out = vec![];
    drop(tx);
    for port in rx {
        out.push(port);
    }

    println!("");
    out.sort();
    for port in out {
        println!("Port {} is open", port);
    }

    Ok(())
}
