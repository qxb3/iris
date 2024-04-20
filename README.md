<h1 align="center">iris</h1>
<p align="center">
    <img src="https://img.shields.io/badge/contribution-welcome-green" />
    <img src="https://img.shields.io/badge/built_with-love-red" />
</p>
<p align="center">A command like in-memory database in rust</p>

<br>

## Features

- ⚡ **Command-like TCP Protocol**: Utilizes a custom, command-oriented protocol over TCP for structured communication between clients and the in-memory database server.
- 💨 **Efficient In-Memory Storage**: Crafted in Rust, "iris" is your go-to for a fast and reliable in-memory database.
- 👤 **User-Friendly Commands**: Easily communicate with the database server using simple commands like SET, GET, DELETE, and more. It's designed to offer an intuitive experience for developers of any skill level.

<br>

## Workspaces

| Workspace                                                                     | Description                                               |
| ----------------------------------------------------------------------------- | --------------------------------------------------------- |
| [iris](https://github.com/qxb3/iris/tree/main/crates/iris)                    | The iris cli that contains the server and the repl client |
| [iris_client](https://github.com/qxb3/iris/tree/main/crates/iris_client)      | The rust client crate to interact with iris server        |

<br>

## TCP Protocol

<img src="https://raw.githubusercontent.com/qxb3/iris/main/repo/tcp_protocol.png" />

Communication in "iris" follows a structured message format, allowing clients to send commands to the server for processing. The message format consists of the following components:

1. **Command Type**: A three-byte identifier specifying the type of command being sent.
2. **ID**: Variable-length identifier for the command, used for tracking and processing purposes.
3. **Data**: Variable-length payload containing additional information required for the command.

```
SET foo bar
│   │   └─ Data
│   └───── ID
└───────── COMMAND
```

Upon receiving a command, the server parses the message according to the defined format and processes the request accordingly. The server responds with a message in the following format:

1. **Status**: A two or three-byte identifier indicating the status of the command execution. It can be either "OK" for successful execution or "ERR" for error conditions.
2. **Response**: A variable-length payload containing additional information or the result of the command execution. This could include data retrieved from the database or an error message.

```
ok foo
│  └─ Response
└──── Status
```

<br>

## Contribution

Contributions to iris are welcome! If you have ideas for improvements, new features, or bug fixes, feel free to open an issue or submit a pull request on [iris](https://github.com/qxb3/iris)
