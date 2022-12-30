# 2456: Most Popular Video Creator

https://leetcode.com/problems/most-popular-video-creator/description/

### IO

Input: `Vec<String>` creators, `Vec<String>` ids, `Vec<i32>` views

Output: `Vec<Vec<String>>` [[most popular creator, most poplar video]]

If multiple creators have a tied max total view count, return both an any order.

For each most popular creator, if multiple videos have a tied max view count, return the one with the minimum id
in [lexicographic order](https://en.wikipedia.org/wiki/Lexicographic_order).

### Thought process

Convert the data into a hashmap like `<creator, HashMap<id, views>>`. Then we can find the max number of total views.
Since there could be multiple creators with the same total, we need to just filter out the lower ones. Then we just do
pretty much the same thing except for individual videos (for each creator), finally taking the video with the minimum id
in lexicographic order.
