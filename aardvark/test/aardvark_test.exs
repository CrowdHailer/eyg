defmodule AardvarkTest do
  use ExUnit.Case
  doctest Aardvark

  test "type of id is the same" do
    program = {:succ, {:succ, 3}}
    assert {:ok, {:integer, 5, 5}} = Aardvark.infer_type(program)

    program = {:succ, {:succ, {:var, :a}}}
    assert {:ok, {:integer, 3, 6}} = Aardvark.infer_type(program)
  end

  test "" do
    # simple succ tru false
  end
end
