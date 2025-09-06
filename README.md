## Rusty Dominoes

This is a part of the journey of getting me comfortable with rust in order to be able to work on [Thunfisch](https://github.com/Kaynoux/thunfisch).
<br>
This project revisits an assignment from 8th grade, which was pretty much my first experience with programming (which I completely bonked [twice] so it's about time to do it right lol)
<br>

## Problem
> Given $n$ randomly chosen dominoes, find the longest chain of k $k\leq n$ dominoes that can be constructed.

## Plan to solve
1. Have a backtracking algorithm attempt to build a chain of length $n$
2. Constantly keep track of the longest chain $c_{max}$ constructed so far
4. If the backtracking algorithm fails (i.e. a chain of length $n$ is not possible), use $c_{max}$
