
defmodule OutTrayTest do
  use ExUnit.Case
  doctest OutTray

  require OutTray
  alias OutTray.Logger

  def testit(y) do
    OutTray.pure do
      x = y
      Logger.info(x)
      Logger.info(:c)
      x
    end
  end

  OutTray.defpure testit2(v) do
    Logger.info(:a)
    testit(v)
    Logger.info(:d)
  end

  test "the truth" do
    assert %OutTray{value: :ok, messages: messages} = testit2(:b)
    messages = Enum.map(messages, fn ({OutTray.Logger, m}) -> m end)
    assert [:a, :b, :c, :d] == messages
  end
end
