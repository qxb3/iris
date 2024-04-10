<h1 align="center">iris</h1>

<p align="center">An in-memory database in rust</p>

```bash
Usage: iris <COMMAND>

Commands:
  server  Start the in-memory server
  client  Enter client repl mode
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

<br>

## Usage

### server

```bash
$ iris server

  ▀             ▀
▄▄▄     ▄ ▄▄  ▄▄▄     ▄▄▄
  █     █▀  ▀   █    █   ▀
  █     █       █     ▀▀▀▄
▄▄█▄▄   █     ▄▄█▄▄  ▀▄▄▄▀

Server has started.
• version:  0.1.0
• host:     http://127.0.0.1:3000
• port:     3000
```

<br>

### client

```bash
$ iris client

  ▀             ▀
▄▄▄     ▄ ▄▄  ▄▄▄     ▄▄▄
  █     █▀  ▀   █    █   ▀
  █     █       █     ▀▀▀▄
▄▄█▄▄   █     ▄▄█▄▄  ▀▄▄▄▀

Client is connected.
• version:  0.1.0
• host:     http://127.0.0.1:3000
• port:     3000

iris@0.1.0 $ SET 0 hello world
ok

iris@0.1.0 $ GET 0
ok "hello world"

iris@0.1.0 $ █
```

<br>

## Todo

- [x] A working server
- [x] A working client repl
- [x] Better client and server code (idk, i think its much better than before)
- [ ] `list` command
- [ ] Make the thing an installable cli
- [ ] Types maybe?
- [ ] Rust client crate
- [ ] Javascript client
