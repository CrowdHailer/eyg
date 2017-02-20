defmodule Mack do
  defmacro hi(a, b) do
    IO.inspect(a)
    quote do: unquote(b)
  end
end
defmodule EygTest do
  use ExUnit.Case
  doctest Eyg

  require Mack

  ast = quote do: 5 + 5
  IO.inspect(ast)
  Macro.prewalk(ast, fn (t) -> IO.inspect(t) end)
  modi = Macro.postwalk(ast, fn
    # (5) -> IO.inspect(10)
    (t) -> IO.inspect(t)
  end)
  |> IO.inspect
  Module.eval_quoted(__MODULE__, modi)
  |> IO.inspect
  import Mack

  test "the truth" do
    ast = quote do: Mack.hi(1, Mack.hi(2, 3))
    Macro.expand_once(ast, __ENV__)
    |> IO.inspect
  end
end
