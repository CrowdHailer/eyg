defmodule OutTray do
  defstruct [value: nil, messages: []]

  def wrap(statement, tray \\ [])
  def wrap(monad = %OutTray{messages: messages}, tray) do
    %{monad | messages: tray ++ messages}
  end
  def wrap(value, tray) do
    %OutTray{value: value, messages: tray}
  end

  defmacro pure(do: b = {:__block__, env, lines}) do
    tray = quote do: tray
    updated = Enum.reduce(lines, [quote do unquote(tray) = [] end], fn
      ({:=, env, [lhs, rhs]}, lines) ->
        lhs = quote do: %OutTray{value: unquote(lhs), messages: unquote(tray)}
        rhs = quote do: unquote(__MODULE__).wrap(unquote(rhs), unquote(tray))
        lines ++ [{:=, env, [lhs, rhs]}]
      (statement, lines) ->
        lhs = quote do: %OutTray{value: _, messages: unquote(tray)}
        rhs = quote do: unquote(__MODULE__).wrap(unquote(statement), unquote(tray))
        lines ++ [{:=, [], [lhs, rhs]}]
    end)
    {:__block__, env, updated}
  end

  defmacro defpure(head, do: action) do
    quote do
      def unquote(head) do
        unquote(__MODULE__).pure do
          unquote(action)
        end
      end
    end
  end

  defmodule Logger do
    import Kernel, except: [send: 2]
    def info(message, recipient \\ Logger) do
      send(recipient, message)
    end
    def send(recipient, message) do
      %OutTray{value: :ok, messages: [{recipient, message}]}
    end
  end

end
