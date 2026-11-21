# Topic Index
| #   | Pattern                                                                    | 
| :-- | :------------------------------------------------------------------------- |
| 1   | [Two Pointers](#topic-two-pointers)                                        |
| 2   | [Sliding Window](#topic-sliding-window)                                    |

## Topic: Two Pointers

**When to Use:**
  - Array/string is sorted (or order doesn't matter so you can sort it) and you are pairing/comparing elements or partitioning in-place.
  - Or array is unsorted and you must filter/compress in-place while preserving original order.
  - **vs. Sliding Window:** If contiguity is *not* required (i.e. you're allowed to sort), it's almost never Sliding Window—it's Sort + Two Pointers.

**The Monotonicity Rule (Picking Pointer Directions):**
  - Always choose pointer directions so **one moves your target expression UP and the other moves it DOWN**:
    - **Sum (`a[low] + a[high]`):** `low += 1` (UP), `high -= 1` (DOWN) -> **Opposite Ends**.
    - **Difference (`a[high] - a[low]`):** `high += 1` (UP), `low += 1` (DOWN) -> **Same Direction**.

**The 4 Core Sub-Patterns:**
  - **1. Opposite Ends (`low = 0`, `high = len` or `len - 1`):** Pairing extremes, greedy matching (Boats), k-Sum (sort + fix outer `i` + opposite-ends on `i+1..n`), 
  bottleneck tracking (Trapping Rain Water).
  - **2. Read / Write (`read`, `write`):** In-place filtering or run-length compression. Safe only when `write <= read` holds at every step. Flush contiguous groups 
  via lookahead (`i + 1 == len || a[i] != a[i + 1]`) or a nested group-by-group `while` to avoid a dangling final group.
  - **3. Same-Direction Chase (`low = 0`, `high = 1`):** Sorted difference pairs (`a[high] - a[low]`). Keep `high > low`, and skip duplicates *inside* the `== k` 
  branch rather than adding top-of-loop `if` guards.
  - **4. Partition & Parallel Streams:**
    - *3-Way Partition:* Maintain 4 half-open zones `[0..low)`, `[low..mid)`, `[mid..high)`, `[high..n)`. Trust the invariant—never add `if` guards to skip harmless 
    `O(1)` swaps.
    - *Parallel Streams (2 inputs):* Never tangle twin state (`i`/`j`, `ci`/`cj`) in one loop—extract a `next_valid(&mut iter)` helper. If future elements decide past 
    elements, reverse direction (`.rev()`) to drop `O(N)` stack space to `O(1)`.

**Rust Gotchas:**
  - Avoid `usize` underflow when moving backwards: use `.rev()` iterators for single-pass consumption, or initialize `high = len` (exclusive) and do `high -= 1` 
  *before* indexing/swapping inside `while low < high`.
  - If initializing `high = len - 1` directly, guard `len < 2` (or `len < 3`) upfront.
  - Use `sort_unstable()` for primitives (`i32`, etc.)—faster with zero extra allocation.

## Two Pointer Revision Index

| #   | Problem                                                                             | Pattern                               | Time / Space            | The Question's Note                                                     |
| :-- | :---------------------------------------------------------------------------------- | :------------------------------------ | :---------------------- | :-----------------------------------------------------------------------|
| 844 | [Backspace String Compare](https://leetcode.com/problems/backspace-string-compare/) | Two Pointers (Reverse Scan)           | `O(N + M)` / `O(1)`     | [Link](#844-backspace-string-compare)                                   |
| 443 | [String Compression](https://leetcode.com/problems/string-compression/)             | Two Pointers (Read / Write)           | `O(N)` / `O(1)`\        | [Link](#443-string-compression)                                         |
| 75  | [Sort Colors](https://leetcode.com/problems/sort-colors/)                           | Three Pointers (3-Way Partition)      | `O(N)` / `O(1)`         | [Link](#75-sort-colors)                                                 |
| 16  | [3Sum Closest](https://leetcode.com/problems/3sum-closest/)                         | Sort + Two Pointers (k-Sum)           | `O(N^2)` / `O(1)`       | [Link](#16-3sum-closest)                                                |
|  *  | Concept                                                                             | Sliding Window \|\| Two Pointer       |  N/A                    | [Link](#sliding-window-vs-opposite-ends-vs-same-direction-two-pointers) |
| 881 | [Boats to Save People](https://leetcode.com/problems/boats-to-save-people/)         | Sort + Two Pointers (Greedy)          | `O(N log N)` / `O(1)`   | [Link](#881-boats-to-save-people)                                       |
| 532 | [K-diff Pairs in an Array](https://leetcode.com/problems/k-diff-pairs-in-an-array/) | HashMap / Sort + Same-Dir 2P          | `O(N)` / `O(N)`         | [Link](#532-k-diff-pairs-in-an-array)                                   |
| 42  | [Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/)           | Opposite-Ends Two Pointers            | `O(N)` / `O(1)`         | [Link](#42-trapping-rain-water)                                         |

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

---

### 42. Trapping Rain Water

**Pattern:** Opposite-Ends Two Pointers (`O(N)` time, `O(1)` space)

**Trick:** Water above index `i` is `min(left_max, right_max) - height[i]`. Instead of precomputing prefix/suffix max arrays (`O(N)` space), keep `left`/`right` 
pointers and running `left_max`/`right_max`; always advance whichever side has the **smaller** max (`left_max < right_max`), because that side is guaranteed to be the 
bottleneck regardless of what's in the middle.

**Notes & Gotchas:**
  - Comparing `left_max.cmp(&right_max)` is cleaner to prove in interviews than `height[left].cmp(&height[right])` (both work because the pointer at the larger max stays 
  pinned, but `left_max < right_max <= true_right_max` makes the bottleneck proof immediate).
  - Updating `left_max = left_max.max(height[left])` *before* `water += left_max - height[left]` guarantees a non-negative contribution without an `if/else`.
  - Guard `if height.len() < 3 { return 0; }` upfront to avoid `height.len() - 1` underflow on empty inputs and skip inputs that cannot trap water.

***

## Topic: Sliding Window

**When to Use (All 3 Must Hold):**
  - Contiguous subarray or substring (order is fixed—you cannot sort).
  - Asking for longest, shortest, max/min, or count of valid subarrays/substrings.
  - **Monotonicity:** Expanding `right += 1` pushes the window property in one consistent direction, and shrinking `left += 1` pushes it in the opposite direction.
  - **When it FAILS:** Negative numbers in sum problems break monotonicity -> use **Prefix Sum + HashMap** instead.

**The 3 Core Templates (Inclusive `[left..=right]`, length = `right - left + 1`, `O(N)` time):**
  - **1. Fixed Size (`k`):**
    - Loop `right` in `0..n`: add `a[right]`, if `right >= k` remove `a[right - k]`, if `right + 1 >= k` update answer.
  - **2. Variable — Longest / Count (Shrink *while invalid*, update *after*):**
    - Add `a[right]` -> `while !is_valid()` remove `a[left]` & `left += 1` -> window is now valid: `best = best.max(right - left + 1)` (or `count += right - left + 1`).
  - **3. Variable — Shortest (Shrink *while valid*, update *inside*):**
    - Add `a[right]` -> `while is_valid()` record `best = best.min(right - left + 1)` *before* shrinking, then remove `a[left]` & `left += 1`.

**Key Math Formulas:**
  - **Counting valid subarrays ending at `right`:** Once `[left..=right]` is valid (for "at most" conditions), all suffixes `[left..=right]` .. `[right..=right]` are 
  valid -> `count += right - left + 1`.
  - **The "Exact `K`" Trick:** Sliding window cannot directly count *exact* `K` distinct/odd/etc. subarrays -> write one Template 2 helper and compute 
  `exact(K) = at_most(K) - at_most(K - 1)`.

**Rust Gotchas:**
  - For ASCII strings, prefer `[usize; 26]` or `[usize; 128]` with `s.as_bytes()` over `HashMap<char, usize>` (stack-allocated, faster, no `.entry()` ceremony; index 
  with `(b - b'a') as usize` or `b as usize`).
  - If using a `HashMap` and relying on `map.len()` for distinct count, **always `map.remove(&key)` when its frequency drops to `0`** so zero-frequency ghosts aren't 
  counted.
  - A `while` inside a `for` is still `O(N)` because `left` and `right` only ever move forward (`<= 2N` total pointer moves).

## Sliding Window Revision Index
