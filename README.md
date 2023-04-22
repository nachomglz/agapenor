
# Agapenor

This binary crate sends a screenshot of all screens, along with the public IP and computer hostname, to an external server when executed. The purpose of this program is for testing and learning purposes only, and not for any illegal or unethical activities.


## Prerequisites

* Rust programming language installed on your system.


## Installation

1. Clone the repository: git clone https://github.com/nachomglz/agapenor

2. Change into the directory: cd agapenor

3. Build the binary crate: cargo build --release


## Usage
1. Navigate to the directory where the binary crate is located: cd target/release
2. Execute the binary crate: ./agapenor
3. The program will take a screenshot of all screens, retrieve the public IP and computer hostname, and send them to the external server.
4. The program will terminate once the data has been sent.

> Note: The external server URL can be configured in the code before building the binary crate.


### Disclaimer

This program is intended for educational and testing purposes only. The author and contributors of this program are not responsible for any illegal or unethical activities conducted using this program. Use at your own risk.
