defmodule Aardvark do
  defmodule StdLib do
    def deduce_type(:integer, min, max) do
      {:ok, {:integer, min, max}}
    end
    def deduce_type(:succ, {:integer, min, max}) do
      {:ok, {:integer, 1 + min, 1 + max}}
    end
    def deduce_type(:succ, type) do
      {:error, {:invalid_type, {:succ, 1}, type}}
    end
  end

  def infer_type(n) when is_integer(n) do
    # Core.Integer record
    {:ok, {:integer, n, n}}
  end

  def infer_type({function, arg}) do
    case infer_type(arg) do
      {:ok, arg_type} ->
        StdLib.deduce_type(function, arg_type)
    end
  end
  def infer_type({function, arg1, arg2}) do
    case infer_type(arg1) do
      {:ok, arg_type1} ->
        case infer_type(arg2) do
          {:ok, arg_type2} ->
            StdLib.deduce_type(function, arg_type1, arg_type2)
        end
    end
  end
end
