defmodule EygTest do
  use ExUnit.Case

  def reduce(:any, :int) do
    :int
  end

  test "integer does not equal a string" do
    ast = quote do
      "3" = 3
    end
    {:error, _} = Eyg.check(ast)
  end
  test "integer can equal a integer" do
    ast = quote do
      3 = 3
    end
    {{:integer, ast}, _acc} = Eyg.check(ast)
    assert {3, []} = Code.eval_quoted(ast)
  end
  test "type of expression is string" do
    ast = quote do
      a = 5
      a
    end
    {{:integer, ast}, _acc} = Eyg.check(ast)
    assert {5, _} = Code.eval_quoted(ast)
  end
  test "what about specs" do
    quote do
      @spec a() :: int
    end
    |> IO.inspect()
  end

  @tag :skip
  test "inspecting some forms" do
    ast = quote do
      x = 5
      y = 7
      x + y
    end
    |> Macro.postwalk(%{}, fn
      (i, acc) when is_integer(i) ->
        {{:int, i}, acc}
      (key = {var, env, context}, acc) when is_atom(context) ->
        case Map.fetch(acc, key) do
          :error ->
            acc = Map.put(acc, key, :any)
            {{:any, {var, env, context}}, acc}
          {:ok, p} ->
            IO.inspect(var)
            IO.inspect(p)
            {{:any, {var, env, context}}, acc}
        end
      ({:=, env, [{l_type, l}, {r_type, r}]}, acc) ->
        type = reduce(l_type, r_type)
        {{type, {:=, env, [l, r]}}, acc}
      (other, acc) ->
        {other, acc} |> IO.inspect
    end)
    |> IO.inspect
  end
end
