# 1910: Remove All Occurrences of a Substring

https://leetcode.com/problems/remove-all-occurrences-of-a-substring/description/

### IO

Input: `String` s, `String` part

Output: `String` with all instances of `part` removed from `s`, from the left (but as if you restarted every time)

### Thought process

Originally I just had a loop removing all instances of `part` until `s` remained unchanged, but since you need to (at
least act like you) restart from index 0 every time you start to remove. For my naïve implementation I decided to just
do that.

I noticed that this only beat (or tied with) 44.83% of solutions with a runtime of 2ms for the test suite, so I wanted
to do something better. Instead of restarting from the beginning of the string every time, you actually only need to
start at where you removed the last instance of `part`, but backwards `part.len() - 1` since this most recent deletion
could never form a match before a match that includes the previous starting point. This method now runs the test suite
in 0ms (beating or tying with 100% since LeetCode is only precise up to the millisecond) and uses less memory (
2.4 MB -> 2.1 MB). Surprisingly, the ternary operator gave it the final push to 0ms, so that was either a fluke or
something's weird with the zero-cost abstractions.

### Example Edge Case

s = "aabababa", part = "aba"

#### Bad

```text
aabababa
a   b    # removes 2 occurrences

ab       # rejoin
ab       # removes 0 occurrences, now finished
```

#### Good

```text
aabababa
a   baba # successfully removed "aba"

ababa    # rejoin
   ba    # successfully remove "aba"

ba       # rejoin
ba       # failed to removed "aba", now finished
```
