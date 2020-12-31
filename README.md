# WireCrab :crab:

> Note: I've created this project to learn Rust and, other than for educational purposes, it's completely useless. If you need a network capturing/monitoring program use [tcpdump](https://www.tcpdump.org/) or [wireshark](https://www.wireshark.org/).

A terminal utility for capturing the network packets and display them on screen.

### Install from source

Prerequisites:
* `rustc 1.48.0`
* `cargo 1.48.0`
* `libpcap-dev` (you can install it with `sudo apt install libpcap-dev`)

To install from source follow these steps:
1. clone the repository
2. run `cargo build` inside the repo directory
3. to allow the build binary to capture network you need to run:

   `sudo setcap cap_net_raw,cap_net_admin=eip target/debug/wirecrab`

4. the binary to run this program should be in `wirecrab/target/debug`.

### Running the program

To run the program just run the binary `./wirecrab`, although there are a couple of required arguments required when running the binary.

##### Modes (required)

There are currently 4 different modes:
* `0` - print each packet individually on the screen
* `1` - print each packet sequentially on the screen
* `2` - print each packet individually and include the hex data on the screen
* `3` - print each packet sequentially and include the hex data on the screen

The argument flag is `-m`.

##### Filename (required)

Pass in the name of the file to which WireCrab will save the captured packets in pcap format. You can then load that file into tcpdump or wireshark to do a deeper analysis.

The argument flag is `-f`.

##### Count

You can optionally pass in the number of packets to be captured.

The argument flag is `-c`. The default value is 100.

#### Example

An example for running the program:

`./wirecrab -m 1 -f output -c 5`

The output would be something like:
```
λ > ./target/debug/wirecrab -m 1 -f output -c 10
Using device wlp0s20f3
Num:       | Time:                | Source:                                  | Destination:                             | Protocol:  | Length:
=================================================================================================================================================
1          | 1609421360.325228    | 192.168.0.13                             | 192.168.0.11                             | TCP        | 176
2          | 1609421360.325347    | 192.168.0.11                             | 192.168.0.13                             | TCP        | 66
3          | 1609421360.523991    | 192.168.0.11                             | 199.232.57.140                           | TCP        | 66
4          | 1609421360.567946    | 199.232.57.140                           | 192.168.0.11                             | TCP        | 66
5          | 1609421360.715769    | 192.168.0.11                             | 151.101.37.140                           | TCP        | 66
6          | 1609421360.760434    | 2a05:4f44:201:7100:c405:846a:f7c9:f8fe   | 2a00:1450:400d:808::200a                 |            | 277
7          | 1609421360.764844    | 151.101.37.140                           | 192.168.0.11                             | TCP        | 66
8          | 1609421360.823793    | 2a00:1450:400d:808::200a                 | 2a05:4f44:201:7100:c405:846a:f7c9:f8fe   |            | 87
9          | 1609421360.971670    | 192.168.0.11                             | 151.101.37.140                           | TCP        | 66
10         | 1609421360.971690    | 192.168.0.11                             | 151.101.113.140                          | TCP        | 66
```

### To do

- [ ] read pcap file
- [ ] tests
- [ ] comments

### Licence

Licenced under MIT license ([LICENSE](LICENCE) or https://opensource.org/licenses/MIT).

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed as above, without any additional terms or conditions.
