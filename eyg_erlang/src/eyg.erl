-module(eyg).

-ifdef(TEST).
-include_lib("eunit/include/eunit.hrl").
-endif.

%% API exports
-export([check/1]).


%%====================================================================
%% API functions
%%====================================================================

check(AST) ->
  {ok, infer_type(AST)}.

infer_type(X) when is_integer(X) ->
  {integer, {X, X}}.

%%====================================================================
%% Internal functions
%%====================================================================


simple_integer_test() ->
  ?assertMatch({ok, {integer, {3, 3}}}, check(3)).

simple_addition_test() ->
  ?assertMatch({ok, {integer, {5, 5}}}, check({add, {2, 3}})).
