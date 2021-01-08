# port_sniff

Tiny port sniffer written in Rust. It's inspired by this code here: [YouTube:Tensor Programming](https://www.youtube.com/watch?v=-Jp7sabBCp4) / [GitHub: Tensor-Programming](https://github.com/tensor-programming/Rust_Port_Sniffer), but utilizes [Clap](https://github.com/clap-rs/clap) for argument parsing and [threadpools](https://github.com/rust-threadpool/rust-threadpool).

## Usage
```bash
    port_sniff [OPTIONS] <ip_addr>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --threads <threads>    number of threads to spawn [default: 4]

ARGS:
    <ip_addr>    a valid IPv4 or IPv6 address
```
