## Prerequisites

- A modern version of rust
- Mutool

## Usage

Run the server on http://localhost:9999:

```bash
$ cargo run
```

The server will accept requests on `/blank` with a given `density` and `height` params. For example:

```bash
$ curl -s "localhost:9999/blank?density=300&height=9999" | jq
```
