# FileHash

## Run the benchmark

```bash
$ mix deps.get
$ mix run bench.exs
```

## Generate test files

Generate a 1K file named `1` with the following command:

```bash
$ head -c 1K </dev/urandom >data/1
```

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `file_hash` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:file_hash, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/file_hash](https://hexdocs.pm/file_hash).
