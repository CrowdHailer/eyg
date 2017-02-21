defmodule MyStateMachine do
  use Actor

  init do
    {args, parent} ->
      # tell for writing to the reader monad
      {{:idle, args}, %{parent, ack({:ok, self()})}}
  end

  state {:idle, count} do
    Actor.ask(from, message) ->
      msgs = Actor.reply(from, count)
      msgs = Logger.debug(message, :"$Logger")
      {{:idle, count + 1}, msgs}
      # nil as same
       Idle.ask(message, count)
    _ ->
      {:nil, %{}}
  end

  state :active, state = {channel_name, refs} do
    subscribe(subscriber, channel_name) ->
      ref <- monitor(subscriber)
      Active.subscribe({ref, channel_name}, state)
      {{:active, {channel_name, refs}}, %{subscriber, {:monitor, self()}}}
    down(ref, _reason) ->
      Active.unsubscribe(ref, state)
    unsubscribe(pid) ->
      Active.unsubscribe(ref, state)
  end

  # Message monad can assume it value is not a writer then wrap the value as a writer
  # can even rewrite equal as bind

  state do
    a = 5
    Logger.write(5)
    Logger.write(10)
    a
  end
  state do
    a <- 5
    _ <- Logger.write(5)
    _ <- Logger.write(10)
    a
  end
  state do
    {a, messages} = wrap(5)
    {_, messages} = wrap(Logger.write(a))
    {_. messages} = wrap(Logger.write(2 * a))
  end
  state do
    wrap(5)
    |> Messenger.flat_map(fn
      (a) ->
        wrap(Logger.write(a))
        |> Messenger.flat_map(fn
          (_) ->
            wrap(Logger.write(a))
          end
        )
      end
    )
  end

  wrap({state, %{}}) -> same
  wrap(other) -> {other, %{}}

  def flat_map(monad1, func) do
    monad2 = func.(monad1.value)
    {monad2.value, Map.merge(monad1.msgs, monad2.msgs)}
  end

  # Do send should be extensible so can just do send on HTTP

  # Monitor as a module

  Monitor.down -> pure msg
  Monitor.setup -> target + message

  # Looking like gen_fsm but helped with macros

  state awaiting(ref, request) do
    Actor.reply(ref, message) ->
      body = render(message, request)
      response = Response.ok(body)
      {{:exit, :normal}
  end

  # Two step have actor define behaviour using method name and state argments then have a second set of modues implement.
end

# Colorize states and mix with :observer.start
defmodule Door do
  use Actor
  @color :blue
  state Open
  @color :red
  state Close
end

# test extract from processes and use very same funcs that return state
defmodule ActorTest do
  MyStateMachine.start_link(conf)
end
