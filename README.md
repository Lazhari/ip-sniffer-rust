# Port Sniffer With Rust

A multi-threading rust program for sniffing an IP address port

## usage

### Help

```
cargo run -- -h
```

> output

```
Usage: -j to select the number threads of threads

 -h or -help to show the help message
```

### Scanning an IP Address

```
cargo run -- -j 10000 192.168.0.1
```

> output

```
............
88 is open
445 is open
3283 is open
3306 is open
3333 is open
5432 is open
5900 is open
6379 is open
8899 is open
27275 is open
49266 is open
50106 is open
```
