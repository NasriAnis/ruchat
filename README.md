### Presentation
This is RuChat a rust based web server that uses websockets for real time communications.

Endpoints:
1. `/`: Entry point
2. `/api/login`: POST: login handling
3. `/api/register`: POST: Registring handling
4. `/js/*`: GET: Serving js files
5. `/css/*`: GET: Serving css

### Usage
To use the project just clone the repository and build it via cargo in release mode :
```
$ git clone git@github.com:NasriAnis/ruchat.git
$ cd ruchat
$ cargo build --release
$ cargo run --release
```
