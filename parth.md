## DSA Revision Index

| #   | Problem                                                                             | Pattern                     | Time / Space        | The Question's Note                                                                       |
| :-- | :---------------------------------------------------------------------------------- | :-------------------------- | :------------------ | :---------------------------------------------------------------------------------------- |
| 844 | [Backspace String Compare](https://leetcode.com/problems/backspace-string-compare/) | Two Pointers (Reverse Scan) | `O(N + M)` / `O(1)` | [Link](#844-backspace-string-comparehttpsleetcodecomproblemsbackspace-string-compare)     |
| 443 | [String Compression](https://leetcode.com/problems/string-compression/)             | Two Pointers (Read / Write) | `O(N)` / `O(1)`\    | [Link](#443-string-compressionhttpsleetcodecomproblemsstring-compression)                 |

---

### [844. Backspace String Compare](https://leetcode.com/problems/backspace-string-compare/)

**Pattern:** Two Pointers (Reverse Scan) | **Complexity:** `O(N + M)` time, `O(1)` space

**Trick:** Scan **right-to-left** with a `skips` counter (`#` increments `skips`, normal char decrements if `> 0`, else survives). Extract a `next_valid(&mut iter)` 
helper and compare both in a `loop`/`match`.

**Notes & Gotchas:**
  - Future `#` decides past chars -> reversing direction drops the `O(N)` stack to `O(1)`.
  - Use `.bytes().rev()` for a free cursor and zero `usize` underflow at index `0`.
  - Never compare raw lengths (`"ab#c"` == `"ac"`), and don't stop early—both iterators must hit `None`.

### [443. String Compression](https://leetcode.com/problems/string-compression/)

**Pattern:** Two Pointers (Read / Write) | **Complexity:** `O(N)` time, `O(1)`\* space

**Trick:** Keep `read` & `write` pointers. Flush each run on its **last element** via lookahead (`i + 1 == len || chars[i] != chars[i + 1]`) — or consume one full 
group per outer iteration with a nested `while` — writing `ch` + `count` digits only when `count > 1`.

**Notes & Gotchas:**
  - Lookahead (or group-by-group nested `while`) avoids the "dangling final group" and removes `last` / `is_empty()` checks.
  - Only write digits when `count > 1`; writing `'1'` breaks `write <= read` and corrupts unread input.
  - Skip any `temp` buffer—write directly into `chars[write]` using `count.to_string().chars()`.
  - *Strictly `O(log N)` temporary space for `count.to_string()`, treated as `O(1)` in practice.*
