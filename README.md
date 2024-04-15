<h1 align="center">iris</h1>

<p align="center">A command like in-memory database in rust</p>

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
- [x] `list` command
- [x] Make a better and sensible command parsing
- [x] Unrestrict id to just a string
- [x] Server response in different formats (json)
- [x] Implement piping operator
- [x] Start writing the rust client crate so i can see i can do the below
- [x] A more sensible non-idiotic server response (still idiotic but i have a vision now? idk)
- [ ] Make the thing an installable cli
- [ ] Javascript client
- [ ] Types maybe?
