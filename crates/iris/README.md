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

iris@0.1.0 $ SET foo hello world
OK > "foo"

iris@0.1.0 $ GET foo
OK > "hello world"

iris@0.1.0 $ █
```

### commands

| Command                | Return |
| ---------------------- | ------ |
| `SET <id> <data>`      | ID     |
| `GET <id>`             | Data   |
| `DEL <expr>`           | Data[] |
| `LST <expr>`           | Data[] |
| `CNT <expr>`           | Number |

## Contribution

Contributions to iris are welcome! If you have ideas for improvements, new features, or bug fixes, feel free to open an issue or submit a pull request on [iris](https://github.com/qxb3/iris)
