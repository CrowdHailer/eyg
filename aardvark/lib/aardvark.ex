defmodule Aardvark do
  @moduledoc """
  Types of AST node
  Literal - 3, "foo", true, false
  Variable - a, b
  Function call/application {}
  Function definition {lambda, var, exp} <- don't even need name

  # Multi arity
  {lambda, [a, b], exp} = {lambda, a, {lambda, b, exp}}
  {x, [a, b]} = {{x, a}, b}

  fn big(x > 4) -> true
  fn big(x < 4) -> false
  This should raise an error

  fn main(binary, machine) ->
    start(parse(binary), machine)
  fn start(options, machine) ->
    # This should all be returned as list of messages to send to the application master
    start_child(
      machine.application_master,
      [:ssl, :init, :ssl.options([])]

    start_child(
      machine.application_master,
      [__MODULE__, :init, [options, machine]],
      name: __MODULE__, ...etc)

  # I think here is where most things return a started supervisor
  fn init() ->

  take the name immutable on hex. I think with the number of people who
  look for immutable js this might be a good idea. have OTP, container and docker in the project description

  """
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

  def check(n) when is_integer(n) do
    # Core.Integer record
    {:ok, {:integer, n, n}}
  end

  def check({function, arg}) do
    case check(arg) do
      {:ok, arg_type} ->
        StdLib.deduce_type(function, arg_type)
    end
  end
  def check({function, arg1, arg2}) do
    case check(arg1) do
      {:ok, arg_type1} ->
        case check(arg2) do
          {:ok, arg_type2} ->
            StdLib.deduce_type(function, arg_type1, arg_type2)
        end
    end
  end
end
