Benchee.run(
  %{
    "memory (stream)" => fn input -> FileHash.memory(input) end,
    "shell" => fn input -> FileHash.shell(input) end
  },
  inputs: [
    {"1K", "data/1"},
    {"10K", "data/10"},
    {"100K", "data/100"},
    {"1000K", "data/1000"},
    {"10000K", "data/10000"}
  ]
)
