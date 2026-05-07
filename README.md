# SyncVault

A lightweight TCP server written in Rust for secure file synchronization over a custom text-based protocol.

## Features

- Custom text-based protocol over raw TCP
- API key authentication
- Configurable via YAML
- Auto-generates default config on first run
- Multi-threaded connection handling

## Requirements

- Rust 1.75+ (edition 2021)
- Cargo

## Installation

```bash
git clone https://github.com/THELAPISUA/SyncVault
cd SyncVault
cargo build --release
```

## Running

```bash
cargo run --release
```

On first launch, SyncVault will automatically create the following files in the working directory:

| File | Description |
|---|---|
| `config.yaml` | Server configuration |
| `keys.txt` | API keys (one per line) |
| `data.txt` | Data file served to clients |

## Configuration

Edit `config.yaml` to customize the server:

```yaml
name: SYNCVAULT
ip: 0.0.0.0
port: 9090
version: "1.0"
keys: keys.txt
data: data.txt
```

| Field | Description |
|---|---|
| `name` | Service name shown to clients on login |
| `ip` | IP address to bind (use `0.0.0.0` for all interfaces) |
| `port` | TCP port to listen on |
| `version` | Version string returned by `CHECK` command |
| `keys` | Path to the API keys file |
| `data` | Path to the data file served via `GETUPDATE` |

## API Keys

Add one API key per line to `keys.txt`:

```
mysecretkey123
anotherapikey456
```

## Protocol

Communication happens over a plain TCP connection. Commands are sent as UTF-8 text.

### Commands

| Command | Auth required | Description |
|---|---|---|
| `LOGIN <apikey>` | No | Authenticate with an API key |
| `CHECK` | Yes | Returns the server version |
| `GETUPDATE` | Yes | Returns the contents of the data file |

### Example session

```
→ LOGIN mysecretkey123
← Successful login to SYNCVAULT

→ CHECK
← 1.0

→ GETUPDATE
← <contents of data.txt>
```

You can test with `netcat`:

```bash
nc 127.0.0.1 9090
LOGIN mysecretkey123
```

## Project Structure

```
src/
├── main.rs       # Entry point, binds TCP listener
├── server.rs     # Connection handling, threading
├── protocol.rs   # Command parsing and dispatch
├── config.rs     # Config loading and defaults
├── state.rs      # Per-connection user state
└── utils.rs      # File reading helpers
```

## Known Limitations

- No TLS — all traffic including API keys is transmitted in plain text
- Simple space-delimited protocol with no escaping
- Flat file storage only

## License

MIT
