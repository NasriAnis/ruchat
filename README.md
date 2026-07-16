# Presentation
This is RuChat a rust based web server that uses websockets for real time communications.

Endpoints:
1. `/`: Entry point
2. `/api/login`: POST: login handling
3. `/api/register`: POST: Registring handling
4. `wss://x.x.x.x:2121/`: Websocket endpoint
5. `/js/*`: GET: Serving js files
6. `/css/*`: GET: Serving css

# Usage
To use the project just clone the repository setup local TLS/SSL certs and build it via cargo in release mode :
### Clonning repo:
```
$ git clone git@github.com:NasriAnis/ruchat.git
$ cd ruchat
```
### TLS/SSL Setup for Local Development
To enable secure WebSocket (wss://) and HTTPS connections for local development, your browser requires valid TLS certificates. Using mkcert is the recommended approach, as it creates a trusted local Certificate Authority that browsers will automatically accept.

We have provided an automated setup script (setup_certs.sh) to handle this configuration effortlessly.

**What the Setup Script Does:** The setup_certs.sh script automates the tedious configuration steps by:
1. Checking Requirements: Ensuring mkcert is installed on your local system before running.
2. Root Authority Registration: Installing a unique local Certificate Authority (CA) straight into your system and browser root stores (including Firefox, Chrome, and Brave).
3. Certificate Generation: Automatically creating a secure certificate valid for localhost, 127.0.0.1, and ::1.
4. Project Structuring: Dropping them cleanly into a ./certs/ directory and renaming them to cert.pem and key.pem to match our Rust application configuration.
5. Security Enforcement: Checking for a .gitignore file and appending your private key (key.pem) to ensure it's never accidentally leaked to version control.

**How to Run It:**
```
$ chmod +x setup_certs.sh
$ ./setup_certs.sh
```
### Building the project:
```
$ cargo build --release
$ cargo run --release
```
### Clean up:
After running the script new certificates will be saved inside the mkcert directory to delete those just run this command:
```
$ rm -rf "$(mkcert -CAROOT)"
```