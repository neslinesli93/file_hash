defmodule FileHash do
  @moduledoc """
  Documentation for `FileHash`.
  """

  def memory(input) do
    input
    |> File.stream!([], 2_048)
    |> Enum.reduce(:crypto.hash_init(:sha), &:crypto.hash_update(&2, &1))
    |> :crypto.hash_final()
    |> Base.encode16()
  end

  def shell(input) do
    {result, 0} = System.cmd("sha1sum", ["-b", input])
    shell_hash(result)
  end

  defp shell_hash(<<hash::binary-size(40), _rest::binary>>), do: hash
end
