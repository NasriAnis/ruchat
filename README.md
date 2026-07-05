### Presentation
This is RuChat a rust based web server that uses websockets for real time communications.

Endpoints:
1. `/`: Entry point
2. `/chat`: Chat page

### Usage

Note that it is possible to find hardcoded URLs (ip:port) in the code chnage them to the machine you are running the server from. for now in :
- `public/chatv2.html` at line 110.

To use the project just clone the repository and build it via cargo in release mode :
```
$ git clone git@github.com:NasriAnis/ruchat.git
$ cd ruchat
$ cargo build --release
$ cargo run --release
```