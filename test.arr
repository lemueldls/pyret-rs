~-1/12

1 / PI

6/9

0.1 + 0.2

"one" + "two"

((((1 + (4 - 2)) / (3 * 5)) > 0) and not(1 < 2)) or (3 > 4)

rec x = 1

_plus(   (  block: x - 5 end )  ,   (2  )  )

check "quick maths":
  2 + 2 is 4
  4 - 1 is 3
end

check "roughnum approx":
  b = ~999999999999999990000

  b is-roughly ~1000000000000000000000

  b is-roughly b + 10000
  b is-roughly b - 10000
end
