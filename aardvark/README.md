# Aardvark

## Implementing a type checker on my own AST

see up-next in crowdhailer/me

## AST

http://blog.mgechev.com/2017/08/05/typed-lambda-calculus-create-type-checker-transpiler-compiler-javascript/
http://osa1.net/posts/2013-06-13-type-checking-with-prolog.html
https://www.xtuc.fr/notes/simple-type-checker.html
```
(function, x, (succ, x))


export id(integer(a, b)) -> integer(a, b)
function id(a) -> a

export double(integer(a, b)) -> integer(a, b)
function double(a) -> a

id(3) === 3

{id {}}

check({add, 2, ok})


function sum(a, b) {
  :math.add(a, b)
}

(
  (assign, sum),
  (function,
    a,
    (function,
      b,
      (
        (:math.add, a),
        b
      )
```
