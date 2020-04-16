# SDS011 Data to Substrate Driver

## Build

```$xslt
git clone https://github.com/vourhey/sds011_rust_driver
cd sds011_rust_driver
cargo build --release
```

## Run
```$xslt
SDS011 Data to Substrate Driver 0.1.0
Vadim Manaenko <vadim.razorq@gmail.com>
The app publishes an IPFS hash of data to Robonomics Chain

USAGE:
    sds011_rust_driver [OPTIONS] -s <suri>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dump <dump_interval>    Interval between dumps to blockchain in minutes [default: 60]
    -p, --port <port>             Specify port a sensor is connected to [default: /dev/ttyUSB0]
        --remote <remote>         Substrate node WebSocket endpoint [default: ws://localhost:9944]
    -s <suri>                     Sender account seed URI
    -w, --work <work_period>      Work period in minutes [default: 5]
```