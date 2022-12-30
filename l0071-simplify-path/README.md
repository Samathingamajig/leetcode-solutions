# 71: Simplify Path

https://leetcode.com/problems/simplify-path/description/

### IO

Input: A Unix-style absolute path (`String`) to a file or directory

Output: A Unix-style canonical path (`String`) to the same file or directory

### Thought process

Initialize a stack for the canonical path. Split the absolute path on the `/` (forward slash) characters, if it's `""`
or `"."` ignore it, if it's `".."` pop off the stack (it's fine if the stack is already empty), if it's anything else
push that to the stack. Then output a string starting with `"/"` then join the stack on `"/"`.
