defmodule AardvarkTest do
  use ExUnit.Case
  doctest Aardvark

  test "type of id is the same" do
    program = {:succ, {:succ, 3}}
    assert {:ok, {:integer, 5, 5}} = Aardvark.check(program)

    program = {:succ, {:succ, {:var, :a}}}
    assert {:ok, {:integer, 3, 6}} = Aardvark.check(program)
  end

  test "" do
    # simple succ tru false
    # auto curried
    {{:add, 3}, 2}
    {{:point, 3}, 2}

    # check x is not in scope
    {:lambda, :x, {:lambda, :y, {{:add, :x}, :y}}}
    # multi headed is a list of terms


    # This needs an add_2 function, that function is built from successors and applied to the first function
    {
      {:lambda, :add_2, {:add_2, 1}},
      {:lambda, :x, {:succ,{:succ, :x}}}
    }
  end
end
