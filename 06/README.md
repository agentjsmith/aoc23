# Day 6 

Day 6 was a math problem, so I solved it and used 
Wolfram Alpha to chug the numbers.  Given:

* D the distance to beat
* T the total time of the race

Find:

* t how long you hold the button

Such that the total distance travelled, d is > D.

```
v = t   (velocity is equal to how long button held)
d = v * (T-t) 

therefore

d = t (T-t)
  = -t^2 + Tt

and solutions for each race are in the form

-t^2 + Tt > D 

which can be solved with the Quadratic formula, or good old
Wolfram Alpha.  Then the number of ways to win is the number of integers between the solutions to the equation.
```

## Part 1 

https://www.wolframalpha.com/input?i=239+%3C+54*t+-+t*t 

https://www.wolframalpha.com/input?i=1142+%3C+70*t+-+t*t 

https://www.wolframalpha.com/input?i=1295+%3C+82*t+-+t*t 

https://www.wolframalpha.com/input?i=1253+%3C+75*t+-+t*t 

## Part 2 

https://www.wolframalpha.com/input?i=count+of+integer+solutions+%28239114212951253+%3C+54708275*t+-+t*t%29
