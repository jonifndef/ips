# ips - A small program used to list network interfaces and their information
Available interface information:
- interface name
- ipv4 address
- status (up/down)
- mac address
- ipv6 addresses
- gateway
- NetworkManager connections using this interface

The fields *"gateway"* and *"ipv6 addresses"* run child processes and are dependent on the cli tools **route** and **nmcli** respectively

In the default state, only *interface name*, *ipv4 address* and *status* is shown for each interface. But the output can easily be controlled with the different flags. Run *--help* to learn more

## Test
A number of unit tests are available in the **src/formatting.rs** file. When adding new tests, it can be useful to see stdout output in a synchronous way. To do this, run
```
cargo test -- --nocapture --test-threads=1
```
Or in order to run a specific test:
```
cargo test test_mac -- --nocapture --test-threads=1
```
Where "*test_mac*" is the name of the test
