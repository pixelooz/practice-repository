## DSA Revision Index

| #   | Problem                                                                             | Pattern                               | Time / Space            | The Question's Note                                                     |
| :-- | :---------------------------------------------------------------------------------- | :------------------------------------ | :---------------------- | :-----------------------------------------------------------------------|
| 844 | [Backspace String Compare](https://leetcode.com/problems/backspace-string-compare/) | Two Pointers (Reverse Scan)           | `O(N + M)` / `O(1)`     | [Link](#844-backspace-string-compare)                                   |
| 443 | [String Compression](https://leetcode.com/problems/string-compression/)             | Two Pointers (Read / Write)           | `O(N)` / `O(1)`\        | [Link](#443-string-compression)                                         |
| 75  | [Sort Colors](https://leetcode.com/problems/sort-colors/)                           | Three Pointers (3-Way Partition)      | `O(N)` / `O(1)`         | [Link](#75-sort-colors)                                                 |
| 16  | [3Sum Closest](https://leetcode.com/problems/3sum-closest/)                         | Sort + Two Pointers (k-Sum)           | `O(N^2)` / `O(1)`       | [Link](#16-3sum-closest)                                                |
|  *  | Concept                                                                             | Sliding Window \|\| Two Pointer       |  N/A                    | [Link](#sliding-window-vs-opposite-ends-vs-same-direction-two-pointers) |
| 881 | [Boats to Save People](https://leetcode.com/problems/boats-to-save-people/)         | Sort + Two Pointers (Greedy)          | `O(N log N)` / `O(1)`   | [Link](#881-boats-to-save-people)                                       |
| 532 | [K-diff Pairs in an Array](https://leetcode.com/problems/k-diff-pairs-in-an-array/) | HashMap / Sort + Same-Dir 2P          | `O(N)` / `O(N)`         | [Link](#532-k-diff-pairs-in-an-array)

---

### 844. Backspace String Compare 

**Pattern:** Two Pointers (Reverse Scan) | **Complexity:** `O(N + M)` time, `O(1)` space

**Trick:** Scan **right-to-left** with a `skips` counter (`#` increments `skips`, normal char decrements if `> 0`, else survives). Extract a `next_valid(&mut iter)` 
helper and compare both in a `loop`/`match`.

**Notes & Gotchas:**
  - Future `#` decides past chars -> reversing direction drops the `O(N)` stack to `O(1)`.
  - Use `.bytes().rev()` for a free cursor and zero `usize` underflow at index `0`.
  - Never compare raw lengths (`"ab#c"` == `"ac"`), and don't stop early—both iterators must hit `None`.

---

### 443. String Compression

**Pattern:** Two Pointers (Read / Write) | **Complexity:** `O(N)` time, `O(1)`\* space

**Trick:** Keep `read` & `write` pointers. Flush each run on its **last element** via lookahead (`i + 1 == len || chars[i] != chars[i + 1]`) — or consume one full 
group per outer iteration with a nested `while` — writing `ch` + `count` digits only when `count > 1`.

**Notes & Gotchas:**
  - Lookahead (or group-by-group nested `while`) avoids the "dangling final group" and removes `last` / `is_empty()` checks.
  - Only write digits when `count > 1`; writing `'1'` breaks `write <= read` and corrupts unread input.
  - Skip any `temp` buffer—write directly into `chars[write]` using `count.to_string().chars()`.
  - *Strictly `O(log N)` temporary space for `count.to_string()`, treated as `O(1)` in practice.*

---

### 75. Sort Colors

**Pattern:** Three Pointers (Dutch National Flag) | **Complexity:** `O(N)` time (1-pass), `O(1)` space

**Trick:** Maintain 4 half-open zones with `low = 0`, `mid = 0`, `high = len`: `[0..low)` = 0s, `[low..mid)` = 1s, `[mid..high)` = unknown, `[high..n)` = 2s. 
Loop `while mid < high`: on `0` swap with `low` & advance both; on `1` advance `mid`; on `2` decrement `high` then swap (keep `mid` still).

**Notes & Gotchas:**
  - Start `mid = 0` so `[low..mid)` starts empty and `nums[0]` isn't skipped; never add `if` guards to avoid harmless `O(1)` self-swaps or `2`-with-`2` swaps.
  - `mid` advances after swapping with `low` because `nums[low]` is guaranteed `1` (or `low == mid`), never unknown.
  - `mid` must **not** advance after swapping with `high`, because the swapped-in element came from the unknown zone and still needs inspection.
  - Initializing `high = nums.len()` (exclusive) and doing `high -= 1` before swapping guarantees zero `usize` underflow (even on `[]` or `[2]`).

---

### 16. 3Sum Closest

**Pattern:** Sort + Two Pointers (k-Sum Reduction) | **Complexity:** `O(N^2)` time, `O(1)` extra space

**Trick:** Sort the array (`O(N log N)` is free under `O(N^2)`). Fix one element with outer loop `i`, then run two pointers (`low = i + 1`, `high = n - 1`) on 
the remainder, updating `closest` whenever `sum.abs_diff(target) < closest.abs_diff(target)` and returning immediately on `sum == target`.

**Notes & Gotchas:**
  - Compare **absolute** distance (`sum.abs_diff(target)` -> `u32`, overflow-safe), not raw `target - sum`.
  - Initialize `closest = nums[0] + nums[1] + nums[2]` instead of an `i32::MAX` sentinel—eliminates a separate `smallest_diff` variable.
  - Skip duplicate outer values (`if i > 0 && nums[i] == nums[i - 1] { continue; }`) to avoid redundant work.
 
---

### Sliding Window Vs Opposite Ends Vs Same Direction Two Pointers

1. **Must the element be contiguous (subarray/substring) - meaning I CANNOT sort?**
   * **Yes** + asks for longest / shortest / count of valid windows ==> **Sliding Window** [`low`..`high` expanding and shrinking].

2. **Does order NOT matter (so I can sort), OR is it already sorted / symmetric, AND am I pairing or comparing extremes?**
   * **Yes** $\implies$ **Opposite-Ends Two Pointers (`low = 0`, `high = n`)**.
   * *Signal:* Every step lets you make a greedy decision on the smallest or largest remaining element (e.g., *2Sum Sorted, 3Sum, 3Sum Closest, Boats to Save People,
   Container With Most Water, Valid Palindrome*).

3. **Must I preserve original order while *filtering / overwriting in-place* or tracking *two speeds*?**
   * **Yes** $\implies$ **Same-Direction Two Pointers (`read`/`write` or `slow`/`fast`)**.
   * *Signal:* One pointer scans ahead while the other writes compacted output or marks a group boundary (e.g., *String Compression, Remove Duplicates, Move Zeroes,
   Linked List Cycle*).

---
 
### 881. Boats to Save People

**Pattern:** Sort + Opposite-Ends Two Pointers (Greedy) | **Complexity:** `O(N log N)` time, `O(1)` extra space

**Trick:** Sort `people`, then use opposite-end pointers (`low = 0`, `high = len`). Every iteration takes the heaviest remaining person (`high -= 1`); if a 
distinct lightest person still exists and fits (`low < high && people[low] + people[high] <= limit`), pair them (`low += 1`). Count 1 boat per iteration.

**Notes & Gotchas:**
  - Not a sliding window—contiguity isn't required, so sorting is allowed (and necessary). Pairing adjacent elements after sorting fails on `[1, 2, 4, 5], limit = 6`.
  - Decrementing `high -= 1` first inside `while low < high` avoids `usize` underflow and ensures a lone middle person (`low == high`) isn't paired with themselves.

---

### 532. K-diff Pairs in an Array

**Pattern:** Frequency HashMap (`O(N)` time, `O(N)` space) **or** Sort + Same-Direction Two Pointers (`O(N log N)` time, `O(1)` extra space)

**Trick:**
  1. **(HashMap — Optimal Time)** Build a frequency map first, then loop over unique `(num, freq)` entries: if `k == 0`, count `freq >= 2`; if `k > 0`, count 
  `map.contains_key(&(num + k))`.
  2. **(Sort + Two Pointers — Optimal Space):** Sort `nums` and move `low = 0`, `high = 1` in the **same direction**: diff `< k` -> `high += 1`; 
  diff `> k` -> `low += 1` (if `low == high`, `high += 1`); diff `== k` -> increment count and skip all duplicate `nums[high]` values.

**Notes & Gotchas:**
  - **Opposite-ends two pointers fails for differences:** both `low += 1` and `high -= 1` *decrease* `nums[high] - nums[low]`, breaking monotonicity 
  (e.g., `[1, 3, 4, 6], k = 2`). Always pick pointer directions so one moves the expression UP and the other DOWN.
  - **Naive 1-pass `curr - k` HashMap fails** on unsorted order (`[3, 1], k = 2`), duplicate pairs (`[1, 3, 3], k = 2`), and 3+ copies when `k == 0` (`[1, 1, 1]`).
  - Checking **only** `num + k` on unique keys guarantees each valid pair `{a, a + k}` is counted exactly once regardless of input order.
  - In Two Pointers, skipping duplicate `nums[high]` values inside `diff == k` keeps `high > low` automatically (if you skip `low` or both instead, enforce 
  `high = high.max(low + 1)` for `k == 0`).
  - Compare `nums[low].abs_diff(nums[high])` directly against `k as u32` (after `if k < 0 { return 0; }`) — don't cast `abs_diff` to `i32`, which wraps on large diffs.
