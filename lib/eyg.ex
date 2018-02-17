defmodule Eyg do
  def check(ast) do
    Macro.postwalk(ast, %{}, &process/2)
  end

  defp process(string, acc) when is_binary(string) do
    term = {:other, string}
    {term, acc}
  end
  defp process(integer, acc) when is_integer(integer) do
    term = {:integer, integer}
    {term, acc}
  end
  defp process({:=, env, [{left_type, left_term}, {right_type, right_term}]}, acc) do
    type = compare(left_type, right_type)
    acc = case left_term do
      {var, _env, _module} ->
        Map.put(acc, var, type)
      _ ->
        acc
    end
    term = {type, {:=, env, [left_term, right_term]}}
    {term, acc}
  end
  defp process({var, env, module}, acc) when is_atom(module) do
    type = Map.get(acc, var, :any)
    acc = Map.put(acc, var, type)
    {{type, {var, env, module}}, acc}
  end
  defp process({:__block__, env, lines}, acc) do
    {type, lines} = Enum.reduce(lines, {:any, []}, fn
      ({type, term}, {_previous, lines}) -> {type, lines ++ [term]} end)
    {{type, {:__block__, env, lines}}, acc}
  end

  # reduce
  defp compare(:other, :integer) do
    raise "Tried for compare integer with other"
  end
  defp compare(:integer, :integer) do
    :integer
  end
  defp compare(:any, :integer) do
    :integer
  end
end
