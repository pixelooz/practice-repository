# LeetCode Interview Patterns: Problem Sets

**54 patterns · 632 problems** (about 621 unique; a few are cross-listed on purpose). Most patterns have 10 problems, and the ones that show up most in interviews have 15. 
Digit DP has only 7 because there are few good free problems for it.

## How to use this

- Work **top to bottom within a pattern**. Problems are ordered easy → hard.
- Do the ⭐ **Core** patterns first, then 📘 **Good to know**. Leave 🧗 **Advanced** for last, or skip it for most non-FAANG interviews.
- Your goal is **not** to finish a pattern. Once you can solve fresh mediums from it in 25-30 minutes without hints, move on.
- Redo anything you failed or needed a hint for after 5-7 days, with no hints.
- Tick the ⬜ box as you go (replace with ✅).

**Legend:** 🟢 Easy · 🟡 Medium · 🔴 Hard

> Problems were picked from the free tier. If LeetCode shows one as premium or a link has moved, skip it or search the title.

## Contents

**Arrays & Strings**

- [1. Hashing / Frequency Counting](#1-hashing--frequency-counting) · ⭐ Core · 15
- [2. Two Pointers](#2-two-pointers) · ⭐ Core · 15
- [3. Sliding Window](#3-sliding-window) · ⭐ Core · 15
- [4. Prefix Sum & Difference Array](#4-prefix-sum--difference-array) · ⭐ Core · 10
- [5. Kadane's Algorithm](#5-kadanes-algorithm) · 📘 Good to know · 10
- [6. Merge Intervals & Sweep Line](#6-merge-intervals--sweep-line) · ⭐ Core · 15
- [7. Cyclic Sort](#7-cyclic-sort) · 📘 Good to know · 10
- [8. Matrix Traversal](#8-matrix-traversal) · 📘 Good to know · 10
- [9. String Manipulation & Parsing](#9-string-manipulation--parsing) · 📘 Good to know · 15

**Searching**

- [10. Binary Search](#10-binary-search) · ⭐ Core · 15
- [11. Binary Search on Answer](#11-binary-search-on-answer) · ⭐ Core · 15
- [12. Search in Rotated / Modified Arrays](#12-search-in-rotated--modified-arrays) · 📘 Good to know · 10

**Linked List**

- [13. Fast & Slow Pointers](#13-fast--slow-pointers) · ⭐ Core · 10
- [14. In-Place Reversal & Manipulation](#14-in-place-reversal--manipulation) · ⭐ Core · 15
- [15. Dummy Head & Merging](#15-dummy-head--merging) · 📘 Good to know · 10

**Stack & Queue**

- [16. Stack (Matching & Expression Evaluation)](#16-stack-matching--expression-evaluation) · ⭐ Core · 15
- [17. Monotonic Stack](#17-monotonic-stack) · ⭐ Core · 15
- [18. Queue & Monotonic Deque](#18-queue--monotonic-deque) · 📘 Good to know · 10

**Trees**

- [19. Tree DFS (Traversals & Recursion)](#19-tree-dfs-traversals--recursion) · ⭐ Core · 15
- [20. Tree BFS (Level Order)](#20-tree-bfs-level-order) · ⭐ Core · 10
- [21. Binary Search Tree (BST)](#21-binary-search-tree-bst) · ⭐ Core · 10
- [22. Lowest Common Ancestor & Path Queries](#22-lowest-common-ancestor--path-queries) · ⭐ Core · 10
- [23. Tree Construction & Serialization](#23-tree-construction--serialization) · 📘 Good to know · 10
- [24. Tree DP](#24-tree-dp) · 📘 Good to know · 10

**Graphs**

- [25. DFS / BFS on Grids & Graphs](#25-dfs--bfs-on-grids--graphs) · ⭐ Core · 15
- [26. Connected Components & Graph DFS](#26-connected-components--graph-dfs) · ⭐ Core · 10
- [27. Topological Sort](#27-topological-sort) · ⭐ Core · 10
- [28. Union-Find (DSU)](#28-union-find-dsu) · ⭐ Core · 10
- [29. Shortest Path (BFS, Dijkstra, Bellman-Ford, Floyd-Warshall)](#29-shortest-path-bfs-dijkstra-bellman-ford-floyd-warshall) · ⭐ Core · 15
- [30. Cycle Detection & Bipartite Check](#30-cycle-detection--bipartite-check) · 📘 Good to know · 10
- [31. MST, Bridges & Advanced Graph](#31-mst-bridges--advanced-graph) · 🧗 Advanced / rare · 10

**Heap / Priority Queue**

- [32. Top-K Elements](#32-top-k-elements) · ⭐ Core · 15
- [33. K-Way Merge](#33-k-way-merge) · 📘 Good to know · 10
- [34. Two Heaps & Scheduling with Heaps](#34-two-heaps--scheduling-with-heaps) · ⭐ Core · 10

**Recursion & Backtracking**

- [35. Subsets, Permutations & Combinations](#35-subsets-permutations--combinations) · ⭐ Core · 15
- [36. Constraint Search (Grid & Board Problems)](#36-constraint-search-grid--board-problems) · 📘 Good to know · 10
- [37. Partition Problems](#37-partition-problems) · 📘 Good to know · 10

**Dynamic Programming**

- [38. 1D DP](#38-1d-dp) · ⭐ Core · 15
- [39. Knapsack (0/1, Unbounded, Subset Sum)](#39-knapsack-01-unbounded-subset-sum) · ⭐ Core · 10
- [40. Subsequence DP (LIS, LCS, Edit Distance)](#40-subsequence-dp-lis-lcs-edit-distance) · ⭐ Core · 15
- [41. Grid DP](#41-grid-dp) · ⭐ Core · 10
- [42. Interval DP](#42-interval-dp) · 📘 Good to know · 10
- [43. State Machine DP (Stocks & Friends)](#43-state-machine-dp-stocks--friends) · 📘 Good to know · 10
- [44. Bitmask DP](#44-bitmask-dp) · 🧗 Advanced / rare · 10
- [45. Digit DP](#45-digit-dp) · 🧗 Advanced / rare · 7

**Greedy**

- [46. Greedy (Jump, Gas Station, Sort + Scan, Exchange Arguments)](#46-greedy-jump-gas-station-sort--scan-exchange-arguments) · ⭐ Core · 15

**Other Patterns**

- [47. Trie (Prefix Tree)](#47-trie-prefix-tree) · ⭐ Core · 10
- [48. Bit Manipulation](#48-bit-manipulation) · 📘 Good to know · 10
- [49. Math & Number Theory](#49-math--number-theory) · 📘 Good to know · 10
- [50. Design Problems](#50-design-problems) · ⭐ Core · 15
- [51. Randomization & Reservoir Sampling](#51-randomization--reservoir-sampling) · 📘 Good to know · 10
- [52. Segment Tree & Fenwick Tree](#52-segment-tree--fenwick-tree) · 🧗 Advanced / rare · 10
- [53. String Algorithms (KMP, Rabin-Karp, Z-Function)](#53-string-algorithms-kmp-rabin-karp-z-function) · 🧗 Advanced / rare · 10
- [54. Geometry](#54-geometry) · 🧗 Advanced / rare · 10

---

## Arrays & Strings

### 1. Hashing / Frequency Counting

⭐ Core · 15 problems

*Reach for it when you need O(1) lookups, counts, or "have I seen this before?"*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ✅ | 1 | [Two Sum](https://leetcode.com/problems/two-sum/) | 🟢 Easy |
| ✅ | 2 | [Contains Duplicate](https://leetcode.com/problems/contains-duplicate/) | 🟢 Easy |
| ✅ | 3 | [Valid Anagram](https://leetcode.com/problems/valid-anagram/) | 🟢 Easy |
| ✅ | 4 | [Ransom Note](https://leetcode.com/problems/ransom-note/) | 🟢 Easy |
| ✅ | 5 | [Isomorphic Strings](https://leetcode.com/problems/isomorphic-strings/) | 🟢 Easy |
| ✅ | 6 | [Word Pattern](https://leetcode.com/problems/word-pattern/) | 🟢 Easy |
| ✅ | 7 | [First Unique Character in a String](https://leetcode.com/problems/first-unique-character-in-a-string/) | 🟢 Easy |
| ✅ | 8 | [Group Anagrams](https://leetcode.com/problems/group-anagrams/) | 🟡 Medium |
| ✅ | 9 | [Top K Frequent Elements](https://leetcode.com/problems/top-k-frequent-elements/) | 🟡 Medium |
| ✅ | 10 | [Product of Array Except Self](https://leetcode.com/problems/product-of-array-except-self/) | 🟡 Medium |
| ⬜ | 11 | [Valid Sudoku](https://leetcode.com/problems/valid-sudoku/) | 🟡 Medium |
| ✅ | 12 | [Longest Consecutive Sequence](https://leetcode.com/problems/longest-consecutive-sequence/) | 🟡 Medium |
| ✅ | 13 | [Subarray Sum Equals K](https://leetcode.com/problems/subarray-sum-equals-k/) | 🟡 Medium |
| ⬜ | 14 | [4Sum II](https://leetcode.com/problems/4sum-ii/) | 🟡 Medium |
| ⬜ | 15 | [Insert Delete GetRandom O(1)](https://leetcode.com/problems/insert-delete-getrandom-o1/) | 🟡 Medium |

### 2. Two Pointers

⭐ Core · 15 problems

*Sorted input, pairs/triplets, in-place rewrites, palindromes*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Reverse String](https://leetcode.com/problems/reverse-string/) | 🟢 Easy |
| ⬜ | 2 | [Valid Palindrome](https://leetcode.com/problems/valid-palindrome/) | 🟢 Easy |
| ⬜ | 3 | [Is Subsequence](https://leetcode.com/problems/is-subsequence/) | 🟢 Easy |
| ⬜ | 4 | [Move Zeroes](https://leetcode.com/problems/move-zeroes/) | 🟢 Easy |
| ⬜ | 5 | [Remove Element](https://leetcode.com/problems/remove-element/) | 🟢 Easy |
| ⬜ | 6 | [Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/) | 🟢 Easy |
| ⬜ | 7 | [Squares of a Sorted Array](https://leetcode.com/problems/squares-of-a-sorted-array/) | 🟢 Easy |
| ⬜ | 8 | [Valid Palindrome II](https://leetcode.com/problems/valid-palindrome-ii/) | 🟢 Easy |
| ⬜ | 9 | [Two Sum II - Input Array Is Sorted](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/) | 🟡 Medium |
| ⬜ | 10 | [Sort Colors](https://leetcode.com/problems/sort-colors/) | 🟡 Medium |
| ⬜ | 11 | [Container With Most Water](https://leetcode.com/problems/container-with-most-water/) | 🟡 Medium |
| ⬜ | 12 | [3Sum](https://leetcode.com/problems/3sum/) | 🟡 Medium |
| ⬜ | 13 | [3Sum Closest](https://leetcode.com/problems/3sum-closest/) | 🟡 Medium |
| ⬜ | 14 | [Boats to Save People](https://leetcode.com/problems/boats-to-save-people/) | 🟡 Medium |
| ⬜ | 15 | [Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/) | 🔴 Hard |

### 3. Sliding Window

⭐ Core · 15 problems

*Contiguous subarray/substring with a constraint (longest, shortest, count)*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Best Time to Buy and Sell Stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/) | 🟢 Easy |
| ⬜ | 2 | [Maximum Average Subarray I](https://leetcode.com/problems/maximum-average-subarray-i/) | 🟢 Easy |
| ⬜ | 3 | [Contains Duplicate II](https://leetcode.com/problems/contains-duplicate-ii/) | 🟢 Easy |
| ⬜ | 4 | [Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/) | 🟡 Medium |
| ⬜ | 5 | [Longest Repeating Character Replacement](https://leetcode.com/problems/longest-repeating-character-replacement/) | 🟡 Medium |
| ⬜ | 6 | [Permutation in String](https://leetcode.com/problems/permutation-in-string/) | 🟡 Medium |
| ⬜ | 7 | [Find All Anagrams in a String](https://leetcode.com/problems/find-all-anagrams-in-a-string/) | 🟡 Medium |
| ⬜ | 8 | [Minimum Size Subarray Sum](https://leetcode.com/problems/minimum-size-subarray-sum/) | 🟡 Medium |
| ⬜ | 9 | [Fruit Into Baskets](https://leetcode.com/problems/fruit-into-baskets/) | 🟡 Medium |
| ⬜ | 10 | [Max Consecutive Ones III](https://leetcode.com/problems/max-consecutive-ones-iii/) | 🟡 Medium |
| ⬜ | 11 | [Count Number of Nice Subarrays](https://leetcode.com/problems/count-number-of-nice-subarrays/) | 🟡 Medium |
| ⬜ | 12 | [Get Equal Substrings Within Budget](https://leetcode.com/problems/get-equal-substrings-within-budget/) | 🟡 Medium |
| ⬜ | 13 | [Maximum Points You Can Obtain from Cards](https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards/) | 🟡 Medium |
| ⬜ | 14 | [Minimum Window Substring](https://leetcode.com/problems/minimum-window-substring/) | 🔴 Hard |
| ⬜ | 15 | [Subarrays with K Different Integers](https://leetcode.com/problems/subarrays-with-k-different-integers/) | 🔴 Hard |

### 4. Prefix Sum & Difference Array

⭐ Core · 10 problems

*Repeated range-sum queries, or "subarray sums to X" with a running total*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Running Sum of 1d Array](https://leetcode.com/problems/running-sum-of-1d-array/) | 🟢 Easy |
| ⬜ | 2 | [Find the Highest Altitude](https://leetcode.com/problems/find-the-highest-altitude/) | 🟢 Easy |
| ⬜ | 3 | [Range Sum Query - Immutable](https://leetcode.com/problems/range-sum-query-immutable/) | 🟢 Easy |
| ⬜ | 4 | [Find Pivot Index](https://leetcode.com/problems/find-pivot-index/) | 🟢 Easy |
| ⬜ | 5 | [Range Sum Query 2D - Immutable](https://leetcode.com/problems/range-sum-query-2d-immutable/) | 🟡 Medium |
| ⬜ | 6 | [Contiguous Array](https://leetcode.com/problems/contiguous-array/) | 🟡 Medium |
| ⬜ | 7 | [Continuous Subarray Sum](https://leetcode.com/problems/continuous-subarray-sum/) | 🟡 Medium |
| ⬜ | 8 | [Subarray Sums Divisible by K](https://leetcode.com/problems/subarray-sums-divisible-by-k/) | 🟡 Medium |
| ⬜ | 9 | [Corporate Flight Bookings](https://leetcode.com/problems/corporate-flight-bookings/) | 🟡 Medium |
| ⬜ | 10 | [Car Pooling](https://leetcode.com/problems/car-pooling/) | 🟡 Medium |

### 5. Kadane's Algorithm

📘 Good to know · 10 problems

*Best contiguous subarray where you decide: extend the current run or restart*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Max Consecutive Ones](https://leetcode.com/problems/max-consecutive-ones/) | 🟢 Easy |
| ⬜ | 2 | [Maximum Ascending Subarray Sum](https://leetcode.com/problems/maximum-ascending-subarray-sum/) | 🟢 Easy |
| ⬜ | 3 | [Longest Continuous Increasing Subsequence](https://leetcode.com/problems/longest-continuous-increasing-subsequence/) | 🟢 Easy |
| ⬜ | 4 | [Maximum Subarray](https://leetcode.com/problems/maximum-subarray/) | 🟡 Medium |
| ⬜ | 5 | [Maximum Product Subarray](https://leetcode.com/problems/maximum-product-subarray/) | 🟡 Medium |
| ⬜ | 6 | [Maximum Sum Circular Subarray](https://leetcode.com/problems/maximum-sum-circular-subarray/) | 🟡 Medium |
| ⬜ | 7 | [Maximum Absolute Sum of Any Subarray](https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/) | 🟡 Medium |
| ⬜ | 8 | [Arithmetic Slices](https://leetcode.com/problems/arithmetic-slices/) | 🟡 Medium |
| ⬜ | 9 | [Longest Turbulent Subarray](https://leetcode.com/problems/longest-turbulent-subarray/) | 🟡 Medium |
| ⬜ | 10 | [Maximum Subarray Sum with One Deletion](https://leetcode.com/problems/maximum-subarray-sum-with-one-deletion/) | 🟡 Medium |

### 6. Merge Intervals & Sweep Line

⭐ Core · 15 problems

*Overlaps, merging ranges, scheduling, "max concurrent" questions*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Summary Ranges](https://leetcode.com/problems/summary-ranges/) | 🟢 Easy |
| ⬜ | 2 | [Merge Intervals](https://leetcode.com/problems/merge-intervals/) | 🟡 Medium |
| ⬜ | 3 | [Insert Interval](https://leetcode.com/problems/insert-interval/) | 🟡 Medium |
| ⬜ | 4 | [Non-overlapping Intervals](https://leetcode.com/problems/non-overlapping-intervals/) | 🟡 Medium |
| ⬜ | 5 | [Minimum Number of Arrows to Burst Balloons](https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/) | 🟡 Medium |
| ⬜ | 6 | [Remove Covered Intervals](https://leetcode.com/problems/remove-covered-intervals/) | 🟡 Medium |
| ⬜ | 7 | [Interval List Intersections](https://leetcode.com/problems/interval-list-intersections/) | 🟡 Medium |
| ⬜ | 8 | [Partition Labels](https://leetcode.com/problems/partition-labels/) | 🟡 Medium |
| ⬜ | 9 | [My Calendar I](https://leetcode.com/problems/my-calendar-i/) | 🟡 Medium |
| ⬜ | 10 | [My Calendar II](https://leetcode.com/problems/my-calendar-ii/) | 🟡 Medium |
| ⬜ | 11 | [Video Stitching](https://leetcode.com/problems/video-stitching/) | 🟡 Medium |
| ⬜ | 12 | [Divide Intervals Into Minimum Number of Groups](https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups/) | 🟡 Medium |
| ⬜ | 13 | [Meeting Rooms III](https://leetcode.com/problems/meeting-rooms-iii/) | 🔴 Hard |
| ⬜ | 14 | [The Skyline Problem](https://leetcode.com/problems/the-skyline-problem/) | 🔴 Hard |
| ⬜ | 15 | [Minimum Interval to Include Each Query](https://leetcode.com/problems/minimum-interval-to-include-each-query/) | 🔴 Hard |

### 7. Cyclic Sort

📘 Good to know · 10 problems

*Array of 1..n (or 0..n) with missing/duplicate values: put each value at its index*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Missing Number](https://leetcode.com/problems/missing-number/) | 🟢 Easy |
| ⬜ | 2 | [Find All Numbers Disappeared in an Array](https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/) | 🟢 Easy |
| ⬜ | 3 | [Set Mismatch](https://leetcode.com/problems/set-mismatch/) | 🟢 Easy |
| ⬜ | 4 | [Kth Missing Positive Number](https://leetcode.com/problems/kth-missing-positive-number/) | 🟢 Easy |
| ⬜ | 5 | [Rotate Array](https://leetcode.com/problems/rotate-array/) | 🟡 Medium |
| ⬜ | 6 | [Find the Duplicate Number](https://leetcode.com/problems/find-the-duplicate-number/) | 🟡 Medium |
| ⬜ | 7 | [Find All Duplicates in an Array](https://leetcode.com/problems/find-all-duplicates-in-an-array/) | 🟡 Medium |
| ⬜ | 8 | [Array Nesting](https://leetcode.com/problems/array-nesting/) | 🟡 Medium |
| ⬜ | 9 | [Couples Holding Hands](https://leetcode.com/problems/couples-holding-hands/) | 🔴 Hard |
| ⬜ | 10 | [First Missing Positive](https://leetcode.com/problems/first-missing-positive/) | 🔴 Hard |

### 8. Matrix Traversal

📘 Good to know · 10 problems

*Spiral, rotate, diagonal, and in-place marking tricks*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Transpose Matrix](https://leetcode.com/problems/transpose-matrix/) | 🟢 Easy |
| ⬜ | 2 | [Toeplitz Matrix](https://leetcode.com/problems/toeplitz-matrix/) | 🟢 Easy |
| ⬜ | 3 | [Reshape the Matrix](https://leetcode.com/problems/reshape-the-matrix/) | 🟢 Easy |
| ⬜ | 4 | [Flipping an Image](https://leetcode.com/problems/flipping-an-image/) | 🟢 Easy |
| ⬜ | 5 | [Rotate Image](https://leetcode.com/problems/rotate-image/) | 🟡 Medium |
| ⬜ | 6 | [Spiral Matrix](https://leetcode.com/problems/spiral-matrix/) | 🟡 Medium |
| ⬜ | 7 | [Spiral Matrix II](https://leetcode.com/problems/spiral-matrix-ii/) | 🟡 Medium |
| ⬜ | 8 | [Set Matrix Zeroes](https://leetcode.com/problems/set-matrix-zeroes/) | 🟡 Medium |
| ⬜ | 9 | [Diagonal Traverse](https://leetcode.com/problems/diagonal-traverse/) | 🟡 Medium |
| ⬜ | 10 | [Game of Life](https://leetcode.com/problems/game-of-life/) | 🟡 Medium |

### 9. String Manipulation & Parsing

📘 Good to know · 15 problems

*Careful index handling, simulation, number/string conversion*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Roman to Integer](https://leetcode.com/problems/roman-to-integer/) | 🟢 Easy |
| ⬜ | 2 | [Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/) | 🟢 Easy |
| ⬜ | 3 | [Add Binary](https://leetcode.com/problems/add-binary/) | 🟢 Easy |
| ⬜ | 4 | [Add Strings](https://leetcode.com/problems/add-strings/) | 🟢 Easy |
| ⬜ | 5 | [Find the Index of the First Occurrence in a String](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/) | 🟢 Easy |
| ⬜ | 6 | [Reverse Vowels of a String](https://leetcode.com/problems/reverse-vowels-of-a-string/) | 🟢 Easy |
| ⬜ | 7 | [Reverse Words in a String](https://leetcode.com/problems/reverse-words-in-a-string/) | 🟡 Medium |
| ⬜ | 8 | [String to Integer (atoi)](https://leetcode.com/problems/string-to-integer-atoi/) | 🟡 Medium |
| ⬜ | 9 | [Zigzag Conversion](https://leetcode.com/problems/zigzag-conversion/) | 🟡 Medium |
| ⬜ | 10 | [Integer to Roman](https://leetcode.com/problems/integer-to-roman/) | 🟡 Medium |
| ⬜ | 11 | [Compare Version Numbers](https://leetcode.com/problems/compare-version-numbers/) | 🟡 Medium |
| ⬜ | 12 | [Simplify Path](https://leetcode.com/problems/simplify-path/) | 🟡 Medium |
| ⬜ | 13 | [Multiply Strings](https://leetcode.com/problems/multiply-strings/) | 🟡 Medium |
| ⬜ | 14 | [Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/) | 🟡 Medium |
| ⬜ | 15 | [Text Justification](https://leetcode.com/problems/text-justification/) | 🔴 Hard |

---

## Searching

### 10. Binary Search

⭐ Core · 15 problems

*Sorted data or any monotonic condition: halve the search space each step*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Binary Search](https://leetcode.com/problems/binary-search/) | 🟢 Easy |
| ⬜ | 2 | [Search Insert Position](https://leetcode.com/problems/search-insert-position/) | 🟢 Easy |
| ⬜ | 3 | [Sqrt(x)](https://leetcode.com/problems/sqrtx/) | 🟢 Easy |
| ⬜ | 4 | [First Bad Version](https://leetcode.com/problems/first-bad-version/) | 🟢 Easy |
| ⬜ | 5 | [Guess Number Higher or Lower](https://leetcode.com/problems/guess-number-higher-or-lower/) | 🟢 Easy |
| ⬜ | 6 | [Valid Perfect Square](https://leetcode.com/problems/valid-perfect-square/) | 🟢 Easy |
| ⬜ | 7 | [Find First and Last Position of Element in Sorted Array](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/) | 🟡 Medium |
| ⬜ | 8 | [Search a 2D Matrix](https://leetcode.com/problems/search-a-2d-matrix/) | 🟡 Medium |
| ⬜ | 9 | [Search a 2D Matrix II](https://leetcode.com/problems/search-a-2d-matrix-ii/) | 🟡 Medium |
| ⬜ | 10 | [Find Peak Element](https://leetcode.com/problems/find-peak-element/) | 🟡 Medium |
| ⬜ | 11 | [Single Element in a Sorted Array](https://leetcode.com/problems/single-element-in-a-sorted-array/) | 🟡 Medium |
| ⬜ | 12 | [Time Based Key-Value Store](https://leetcode.com/problems/time-based-key-value-store/) | 🟡 Medium |
| ⬜ | 13 | [Find K Closest Elements](https://leetcode.com/problems/find-k-closest-elements/) | 🟡 Medium |
| ⬜ | 14 | [Kth Smallest Element in a Sorted Matrix](https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/) | 🟡 Medium |
| ⬜ | 15 | [Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/) | 🔴 Hard |

### 11. Binary Search on Answer

⭐ Core · 15 problems

*"Minimize the maximum" / "maximize the minimum": binary search the answer, check feasibility*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Arranging Coins](https://leetcode.com/problems/arranging-coins/) | 🟢 Easy |
| ⬜ | 2 | [Heaters](https://leetcode.com/problems/heaters/) | 🟡 Medium |
| ⬜ | 3 | [Find the Smallest Divisor Given a Threshold](https://leetcode.com/problems/find-the-smallest-divisor-given-a-threshold/) | 🟡 Medium |
| ⬜ | 4 | [Koko Eating Bananas](https://leetcode.com/problems/koko-eating-bananas/) | 🟡 Medium |
| ⬜ | 5 | [Capacity To Ship Packages Within D Days](https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/) | 🟡 Medium |
| ⬜ | 6 | [Minimum Number of Days to Make m Bouquets](https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/) | 🟡 Medium |
| ⬜ | 7 | [Successful Pairs of Spells and Potions](https://leetcode.com/problems/successful-pairs-of-spells-and-potions/) | 🟡 Medium |
| ⬜ | 8 | [Minimum Speed to Arrive on Time](https://leetcode.com/problems/minimum-speed-to-arrive-on-time/) | 🟡 Medium |
| ⬜ | 9 | [Magnetic Force Between Two Balls](https://leetcode.com/problems/magnetic-force-between-two-balls/) | 🟡 Medium |
| ⬜ | 10 | [Minimum Time to Complete Trips](https://leetcode.com/problems/minimum-time-to-complete-trips/) | 🟡 Medium |
| ⬜ | 11 | [Maximum Candies Allocated to K Children](https://leetcode.com/problems/maximum-candies-allocated-to-k-children/) | 🟡 Medium |
| ⬜ | 12 | [Minimized Maximum of Products Distributed to Any Store](https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/) | 🟡 Medium |
| ⬜ | 13 | [Split Array Largest Sum](https://leetcode.com/problems/split-array-largest-sum/) | 🔴 Hard |
| ⬜ | 14 | [Find K-th Smallest Pair Distance](https://leetcode.com/problems/find-k-th-smallest-pair-distance/) | 🔴 Hard |
| ⬜ | 15 | [Kth Smallest Number in Multiplication Table](https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/) | 🔴 Hard |

### 12. Search in Rotated / Modified Arrays

📘 Good to know · 10 problems

*Binary search where the array is rotated, mountain-shaped, or partially sorted*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Check if Array Is Sorted and Rotated](https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/) | 🟢 Easy |
| ⬜ | 2 | [Peak Index in a Mountain Array](https://leetcode.com/problems/peak-index-in-a-mountain-array/) | 🟡 Medium |
| ⬜ | 3 | [Count Negative Numbers in a Sorted Matrix](https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/) | 🟢 Easy |
| ⬜ | 4 | [Find Minimum in Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/) | 🟡 Medium |
| ⬜ | 5 | [Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/) | 🟡 Medium |
| ⬜ | 6 | [Search in Rotated Sorted Array II](https://leetcode.com/problems/search-in-rotated-sorted-array-ii/) | 🟡 Medium |
| ⬜ | 7 | [H-Index II](https://leetcode.com/problems/h-index-ii/) | 🟡 Medium |
| ⬜ | 8 | [Find Right Interval](https://leetcode.com/problems/find-right-interval/) | 🟡 Medium |
| ⬜ | 9 | [Find Minimum in Rotated Sorted Array II](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/) | 🔴 Hard |
| ⬜ | 10 | [Find in Mountain Array](https://leetcode.com/problems/find-in-mountain-array/) | 🔴 Hard |

---

## Linked List

### 13. Fast & Slow Pointers

⭐ Core · 10 problems

*Cycle detection, finding the middle, k-th from the end*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Linked List Cycle](https://leetcode.com/problems/linked-list-cycle/) | 🟢 Easy |
| ⬜ | 2 | [Middle of the Linked List](https://leetcode.com/problems/middle-of-the-linked-list/) | 🟢 Easy |
| ⬜ | 3 | [Happy Number](https://leetcode.com/problems/happy-number/) | 🟢 Easy |
| ⬜ | 4 | [Intersection of Two Linked Lists](https://leetcode.com/problems/intersection-of-two-linked-lists/) | 🟢 Easy |
| ⬜ | 5 | [Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/) | 🟢 Easy |
| ⬜ | 6 | [Linked List Cycle II](https://leetcode.com/problems/linked-list-cycle-ii/) | 🟡 Medium |
| ⬜ | 7 | [Remove Nth Node From End of List](https://leetcode.com/problems/remove-nth-node-from-end-of-list/) | 🟡 Medium |
| ⬜ | 8 | [Delete the Middle Node of a Linked List](https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list/) | 🟡 Medium |
| ⬜ | 9 | [Reorder List](https://leetcode.com/problems/reorder-list/) | 🟡 Medium |
| ⬜ | 10 | [Circular Array Loop](https://leetcode.com/problems/circular-array-loop/) | 🟡 Medium |

### 14. In-Place Reversal & Manipulation

⭐ Core · 15 problems

*Rewire pointers without extra space: reverse, swap, partition, rotate*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/) | 🟢 Easy |
| ⬜ | 2 | [Remove Linked List Elements](https://leetcode.com/problems/remove-linked-list-elements/) | 🟢 Easy |
| ⬜ | 3 | [Remove Duplicates from Sorted List](https://leetcode.com/problems/remove-duplicates-from-sorted-list/) | 🟢 Easy |
| ⬜ | 4 | [Swap Nodes in Pairs](https://leetcode.com/problems/swap-nodes-in-pairs/) | 🟡 Medium |
| ⬜ | 5 | [Reverse Linked List II](https://leetcode.com/problems/reverse-linked-list-ii/) | 🟡 Medium |
| ⬜ | 6 | [Odd Even Linked List](https://leetcode.com/problems/odd-even-linked-list/) | 🟡 Medium |
| ⬜ | 7 | [Rotate List](https://leetcode.com/problems/rotate-list/) | 🟡 Medium |
| ⬜ | 8 | [Partition List](https://leetcode.com/problems/partition-list/) | 🟡 Medium |
| ⬜ | 9 | [Remove Duplicates from Sorted List II](https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/) | 🟡 Medium |
| ⬜ | 10 | [Add Two Numbers](https://leetcode.com/problems/add-two-numbers/) | 🟡 Medium |
| ⬜ | 11 | [Maximum Twin Sum of a Linked List](https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/) | 🟡 Medium |
| ⬜ | 12 | [Sort List](https://leetcode.com/problems/sort-list/) | 🟡 Medium |
| ⬜ | 13 | [Copy List with Random Pointer](https://leetcode.com/problems/copy-list-with-random-pointer/) | 🟡 Medium |
| ⬜ | 14 | [Flatten a Multilevel Doubly Linked List](https://leetcode.com/problems/flatten-a-multilevel-doubly-linked-list/) | 🟡 Medium |
| ⬜ | 15 | [Reverse Nodes in k-Group](https://leetcode.com/problems/reverse-nodes-in-k-group/) | 🔴 Hard |

### 15. Dummy Head & Merging

📘 Good to know · 10 problems

*A dummy node removes head edge-cases when building or merging lists*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Delete Node in a Linked List](https://leetcode.com/problems/delete-node-in-a-linked-list/) | 🟡 Medium |
| ⬜ | 2 | [Convert Binary Number in a Linked List to Integer](https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/) | 🟢 Easy |
| ⬜ | 3 | [Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/) | 🟢 Easy |
| ⬜ | 4 | [Merge In Between Linked Lists](https://leetcode.com/problems/merge-in-between-linked-lists/) | 🟡 Medium |
| ⬜ | 5 | [Merge Nodes in Between Zeros](https://leetcode.com/problems/merge-nodes-in-between-zeros/) | 🟡 Medium |
| ⬜ | 6 | [Swapping Nodes in a Linked List](https://leetcode.com/problems/swapping-nodes-in-a-linked-list/) | 🟡 Medium |
| ⬜ | 7 | [Add Two Numbers II](https://leetcode.com/problems/add-two-numbers-ii/) | 🟡 Medium |
| ⬜ | 8 | [Split Linked List in Parts](https://leetcode.com/problems/split-linked-list-in-parts/) | 🟡 Medium |
| ⬜ | 9 | [Remove Zero Sum Consecutive Nodes from Linked List](https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/) | 🟡 Medium |
| ⬜ | 10 | [Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/) | 🔴 Hard |

---

## Stack & Queue

### 16. Stack (Matching & Expression Evaluation)

⭐ Core · 15 problems

*Nested structure, "most recent unmatched", undo, expression parsing*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Valid Parentheses](https://leetcode.com/problems/valid-parentheses/) | 🟢 Easy |
| ⬜ | 2 | [Baseball Game](https://leetcode.com/problems/baseball-game/) | 🟢 Easy |
| ⬜ | 3 | [Remove All Adjacent Duplicates In String](https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/) | 🟢 Easy |
| ⬜ | 4 | [Implement Queue using Stacks](https://leetcode.com/problems/implement-queue-using-stacks/) | 🟢 Easy |
| ⬜ | 5 | [Implement Stack using Queues](https://leetcode.com/problems/implement-stack-using-queues/) | 🟢 Easy |
| ⬜ | 6 | [Min Stack](https://leetcode.com/problems/min-stack/) | 🟡 Medium |
| ⬜ | 7 | [Evaluate Reverse Polish Notation](https://leetcode.com/problems/evaluate-reverse-polish-notation/) | 🟡 Medium |
| ⬜ | 8 | [Minimum Remove to Make Valid Parentheses](https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/) | 🟡 Medium |
| ⬜ | 9 | [Asteroid Collision](https://leetcode.com/problems/asteroid-collision/) | 🟡 Medium |
| ⬜ | 10 | [Decode String](https://leetcode.com/problems/decode-string/) | 🟡 Medium |
| ⬜ | 11 | [Remove K Digits](https://leetcode.com/problems/remove-k-digits/) | 🟡 Medium |
| ⬜ | 12 | [Basic Calculator II](https://leetcode.com/problems/basic-calculator-ii/) | 🟡 Medium |
| ⬜ | 13 | [Remove Duplicate Letters](https://leetcode.com/problems/remove-duplicate-letters/) | 🟡 Medium |
| ⬜ | 14 | [Longest Valid Parentheses](https://leetcode.com/problems/longest-valid-parentheses/) | 🔴 Hard |
| ⬜ | 15 | [Basic Calculator](https://leetcode.com/problems/basic-calculator/) | 🔴 Hard |

### 17. Monotonic Stack

⭐ Core · 15 problems

*"Next greater / smaller element", spans, histogram-style problems*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Next Greater Element I](https://leetcode.com/problems/next-greater-element-i/) | 🟢 Easy |
| ⬜ | 2 | [Final Prices With a Special Discount in a Shop](https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/) | 🟢 Easy |
| ⬜ | 3 | [Daily Temperatures](https://leetcode.com/problems/daily-temperatures/) | 🟡 Medium |
| ⬜ | 4 | [Next Greater Element II](https://leetcode.com/problems/next-greater-element-ii/) | 🟡 Medium |
| ⬜ | 5 | [Online Stock Span](https://leetcode.com/problems/online-stock-span/) | 🟡 Medium |
| ⬜ | 6 | [Car Fleet](https://leetcode.com/problems/car-fleet/) | 🟡 Medium |
| ⬜ | 7 | [132 Pattern](https://leetcode.com/problems/132-pattern/) | 🟡 Medium |
| ⬜ | 8 | [Steps to Make Array Non-decreasing](https://leetcode.com/problems/steps-to-make-array-non-decreasing/) | 🟡 Medium |
| ⬜ | 9 | [Shortest Unsorted Continuous Subarray](https://leetcode.com/problems/shortest-unsorted-continuous-subarray/) | 🟡 Medium |
| ⬜ | 10 | [Maximum Width Ramp](https://leetcode.com/problems/maximum-width-ramp/) | 🟡 Medium |
| ⬜ | 11 | [Sum of Subarray Minimums](https://leetcode.com/problems/sum-of-subarray-minimums/) | 🟡 Medium |
| ⬜ | 12 | [Sum of Subarray Ranges](https://leetcode.com/problems/sum-of-subarray-ranges/) | 🟡 Medium |
| ⬜ | 13 | [Largest Rectangle in Histogram](https://leetcode.com/problems/largest-rectangle-in-histogram/) | 🔴 Hard |
| ⬜ | 14 | [Maximal Rectangle](https://leetcode.com/problems/maximal-rectangle/) | 🔴 Hard |
| ⬜ | 15 | [Number of Visible People in a Queue](https://leetcode.com/problems/number-of-visible-people-in-a-queue/) | 🔴 Hard |

### 18. Queue & Monotonic Deque

📘 Good to know · 10 problems

*Simulations with queues; sliding-window max/min with a deque*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Number of Recent Calls](https://leetcode.com/problems/number-of-recent-calls/) | 🟢 Easy |
| ⬜ | 2 | [Number of Students Unable to Eat Lunch](https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/) | 🟢 Easy |
| ⬜ | 3 | [Time Needed to Buy Tickets](https://leetcode.com/problems/time-needed-to-buy-tickets/) | 🟢 Easy |
| ⬜ | 4 | [Design Circular Queue](https://leetcode.com/problems/design-circular-queue/) | 🟡 Medium |
| ⬜ | 5 | [Dota2 Senate](https://leetcode.com/problems/dota2-senate/) | 🟡 Medium |
| ⬜ | 6 | [Reveal Cards In Increasing Order](https://leetcode.com/problems/reveal-cards-in-increasing-order/) | 🟡 Medium |
| ⬜ | 7 | [Jump Game VI](https://leetcode.com/problems/jump-game-vi/) | 🟡 Medium |
| ⬜ | 8 | [Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit](https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit/) | 🟡 Medium |
| ⬜ | 9 | [Sliding Window Maximum](https://leetcode.com/problems/sliding-window-maximum/) | 🔴 Hard |
| ⬜ | 10 | [Shortest Subarray with Sum at Least K](https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/) | 🔴 Hard |

---

## Trees

### 19. Tree DFS (Traversals & Recursion)

⭐ Core · 15 problems

*Most tree problems: recurse, return info upward, combine at the parent*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/) | 🟢 Easy |
| ⬜ | 2 | [Invert Binary Tree](https://leetcode.com/problems/invert-binary-tree/) | 🟢 Easy |
| ⬜ | 3 | [Same Tree](https://leetcode.com/problems/same-tree/) | 🟢 Easy |
| ⬜ | 4 | [Symmetric Tree](https://leetcode.com/problems/symmetric-tree/) | 🟢 Easy |
| ⬜ | 5 | [Subtree of Another Tree](https://leetcode.com/problems/subtree-of-another-tree/) | 🟢 Easy |
| ⬜ | 6 | [Balanced Binary Tree](https://leetcode.com/problems/balanced-binary-tree/) | 🟢 Easy |
| ⬜ | 7 | [Diameter of Binary Tree](https://leetcode.com/problems/diameter-of-binary-tree/) | 🟢 Easy |
| ⬜ | 8 | [Path Sum](https://leetcode.com/problems/path-sum/) | 🟢 Easy |
| ⬜ | 9 | [Binary Tree Paths](https://leetcode.com/problems/binary-tree-paths/) | 🟢 Easy |
| ⬜ | 10 | [Binary Tree Inorder Traversal](https://leetcode.com/problems/binary-tree-inorder-traversal/) | 🟢 Easy |
| ⬜ | 11 | [Count Good Nodes in Binary Tree](https://leetcode.com/problems/count-good-nodes-in-binary-tree/) | 🟡 Medium |
| ⬜ | 12 | [Path Sum II](https://leetcode.com/problems/path-sum-ii/) | 🟡 Medium |
| ⬜ | 13 | [Sum Root to Leaf Numbers](https://leetcode.com/problems/sum-root-to-leaf-numbers/) | 🟡 Medium |
| ⬜ | 14 | [Flatten Binary Tree to Linked List](https://leetcode.com/problems/flatten-binary-tree-to-linked-list/) | 🟡 Medium |
| ⬜ | 15 | [Binary Tree Maximum Path Sum](https://leetcode.com/problems/binary-tree-maximum-path-sum/) | 🔴 Hard |

### 20. Tree BFS (Level Order)

⭐ Core · 10 problems

*Anything per-level: right side view, zigzag, min depth, width*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Average of Levels in Binary Tree](https://leetcode.com/problems/average-of-levels-in-binary-tree/) | 🟢 Easy |
| ⬜ | 2 | [Minimum Depth of Binary Tree](https://leetcode.com/problems/minimum-depth-of-binary-tree/) | 🟢 Easy |
| ⬜ | 3 | [Cousins in Binary Tree](https://leetcode.com/problems/cousins-in-binary-tree/) | 🟢 Easy |
| ⬜ | 4 | [Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/) | 🟡 Medium |
| ⬜ | 5 | [Binary Tree Level Order Traversal II](https://leetcode.com/problems/binary-tree-level-order-traversal-ii/) | 🟡 Medium |
| ⬜ | 6 | [Binary Tree Zigzag Level Order Traversal](https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/) | 🟡 Medium |
| ⬜ | 7 | [Binary Tree Right Side View](https://leetcode.com/problems/binary-tree-right-side-view/) | 🟡 Medium |
| ⬜ | 8 | [Populating Next Right Pointers in Each Node](https://leetcode.com/problems/populating-next-right-pointers-in-each-node/) | 🟡 Medium |
| ⬜ | 9 | [Find Largest Value in Each Tree Row](https://leetcode.com/problems/find-largest-value-in-each-tree-row/) | 🟡 Medium |
| ⬜ | 10 | [Maximum Width of Binary Tree](https://leetcode.com/problems/maximum-width-of-binary-tree/) | 🟡 Medium |

### 21. Binary Search Tree (BST)

⭐ Core · 10 problems

*Use the ordering: inorder = sorted, go left/right by comparison*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Search in a Binary Search Tree](https://leetcode.com/problems/search-in-a-binary-search-tree/) | 🟢 Easy |
| ⬜ | 2 | [Range Sum of BST](https://leetcode.com/problems/range-sum-of-bst/) | 🟢 Easy |
| ⬜ | 3 | [Convert Sorted Array to Binary Search Tree](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/) | 🟢 Easy |
| ⬜ | 4 | [Minimum Absolute Difference in BST](https://leetcode.com/problems/minimum-absolute-difference-in-bst/) | 🟢 Easy |
| ⬜ | 5 | [Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree/) | 🟡 Medium |
| ⬜ | 6 | [Insert into a Binary Search Tree](https://leetcode.com/problems/insert-into-a-binary-search-tree/) | 🟡 Medium |
| ⬜ | 7 | [Kth Smallest Element in a BST](https://leetcode.com/problems/kth-smallest-element-in-a-bst/) | 🟡 Medium |
| ⬜ | 8 | [Delete Node in a BST](https://leetcode.com/problems/delete-node-in-a-bst/) | 🟡 Medium |
| ⬜ | 9 | [Binary Search Tree Iterator](https://leetcode.com/problems/binary-search-tree-iterator/) | 🟡 Medium |
| ⬜ | 10 | [Recover Binary Search Tree](https://leetcode.com/problems/recover-binary-search-tree/) | 🟡 Medium |

### 22. Lowest Common Ancestor & Path Queries

⭐ Core · 10 problems

*Paths between nodes, ancestors, distances, subtree-with-property*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Lowest Common Ancestor of a Binary Search Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/) | 🟡 Medium |
| ⬜ | 2 | [Lowest Common Ancestor of a Binary Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/) | 🟡 Medium |
| ⬜ | 3 | [Lowest Common Ancestor of Deepest Leaves](https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/) | 🟡 Medium |
| ⬜ | 4 | [Smallest Subtree with all the Deepest Nodes](https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/) | 🟡 Medium |
| ⬜ | 5 | [Maximum Difference Between Node and Ancestor](https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/) | 🟡 Medium |
| ⬜ | 6 | [All Nodes Distance K in Binary Tree](https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/) | 🟡 Medium |
| ⬜ | 7 | [Amount of Time for Binary Tree to Be Infected](https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected/) | 🟡 Medium |
| ⬜ | 8 | [Step-By-Step Directions From a Binary Tree Node to Another](https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/) | 🟡 Medium |
| ⬜ | 9 | [Longest Univalue Path](https://leetcode.com/problems/longest-univalue-path/) | 🟡 Medium |
| ⬜ | 10 | [Sum of Distances in Tree](https://leetcode.com/problems/sum-of-distances-in-tree/) | 🔴 Hard |

### 23. Tree Construction & Serialization

📘 Good to know · 10 problems

*Build a tree from traversals, or encode/decode it*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Merge Two Binary Trees](https://leetcode.com/problems/merge-two-binary-trees/) | 🟢 Easy |
| ⬜ | 2 | [Construct String from Binary Tree](https://leetcode.com/problems/construct-string-from-binary-tree/) | 🟢 Easy |
| ⬜ | 3 | [Convert Sorted List to Binary Search Tree](https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/) | 🟡 Medium |
| ⬜ | 4 | [Maximum Binary Tree](https://leetcode.com/problems/maximum-binary-tree/) | 🟡 Medium |
| ⬜ | 5 | [Construct Binary Tree from Preorder and Inorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/) | 🟡 Medium |
| ⬜ | 6 | [Construct Binary Tree from Inorder and Postorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/) | 🟡 Medium |
| ⬜ | 7 | [Construct Binary Search Tree from Preorder Traversal](https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/) | 🟡 Medium |
| ⬜ | 8 | [Find Duplicate Subtrees](https://leetcode.com/problems/find-duplicate-subtrees/) | 🟡 Medium |
| ⬜ | 9 | [Serialize and Deserialize BST](https://leetcode.com/problems/serialize-and-deserialize-bst/) | 🟡 Medium |
| ⬜ | 10 | [Serialize and Deserialize Binary Tree](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/) | 🔴 Hard |

### 24. Tree DP

📘 Good to know · 10 problems

*Each node returns a summary (best with/without it) that the parent combines*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Binary Tree Tilt](https://leetcode.com/problems/binary-tree-tilt/) | 🟢 Easy |
| ⬜ | 2 | [House Robber III](https://leetcode.com/problems/house-robber-iii/) | 🟡 Medium |
| ⬜ | 3 | [Most Frequent Subtree Sum](https://leetcode.com/problems/most-frequent-subtree-sum/) | 🟡 Medium |
| ⬜ | 4 | [Longest ZigZag Path in a Binary Tree](https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree/) | 🟡 Medium |
| ⬜ | 5 | [Maximum Product of Splitted Binary Tree](https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/) | 🟡 Medium |
| ⬜ | 6 | [Distribute Coins in Binary Tree](https://leetcode.com/problems/distribute-coins-in-binary-tree/) | 🟡 Medium |
| ⬜ | 7 | [Delete Nodes And Return Forest](https://leetcode.com/problems/delete-nodes-and-return-forest/) | 🟡 Medium |
| ⬜ | 8 | [Unique Binary Search Trees](https://leetcode.com/problems/unique-binary-search-trees/) | 🟡 Medium |
| ⬜ | 9 | [Binary Tree Cameras](https://leetcode.com/problems/binary-tree-cameras/) | 🔴 Hard |
| ⬜ | 10 | [Maximum Sum BST in Binary Tree](https://leetcode.com/problems/maximum-sum-bst-in-binary-tree/) | 🔴 Hard |

---

## Graphs

### 25. DFS / BFS on Grids & Graphs

⭐ Core · 15 problems

*Flood fill, islands, multi-source BFS, shortest steps in unweighted graphs*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Find if Path Exists in Graph](https://leetcode.com/problems/find-if-path-exists-in-graph/) | 🟢 Easy |
| ⬜ | 2 | [Flood Fill](https://leetcode.com/problems/flood-fill/) | 🟢 Easy |
| ⬜ | 3 | [Island Perimeter](https://leetcode.com/problems/island-perimeter/) | 🟢 Easy |
| ⬜ | 4 | [Number of Islands](https://leetcode.com/problems/number-of-islands/) | 🟡 Medium |
| ⬜ | 5 | [Max Area of Island](https://leetcode.com/problems/max-area-of-island/) | 🟡 Medium |
| ⬜ | 6 | [Number of Closed Islands](https://leetcode.com/problems/number-of-closed-islands/) | 🟡 Medium |
| ⬜ | 7 | [Number of Enclaves](https://leetcode.com/problems/number-of-enclaves/) | 🟡 Medium |
| ⬜ | 8 | [Surrounded Regions](https://leetcode.com/problems/surrounded-regions/) | 🟡 Medium |
| ⬜ | 9 | [Pacific Atlantic Water Flow](https://leetcode.com/problems/pacific-atlantic-water-flow/) | 🟡 Medium |
| ⬜ | 10 | [Rotting Oranges](https://leetcode.com/problems/rotting-oranges/) | 🟡 Medium |
| ⬜ | 11 | [01 Matrix](https://leetcode.com/problems/01-matrix/) | 🟡 Medium |
| ⬜ | 12 | [Shortest Path in Binary Matrix](https://leetcode.com/problems/shortest-path-in-binary-matrix/) | 🟡 Medium |
| ⬜ | 13 | [Clone Graph](https://leetcode.com/problems/clone-graph/) | 🟡 Medium |
| ⬜ | 14 | [Open the Lock](https://leetcode.com/problems/open-the-lock/) | 🟡 Medium |
| ⬜ | 15 | [Word Ladder](https://leetcode.com/problems/word-ladder/) | 🔴 Hard |

### 26. Connected Components & Graph DFS

⭐ Core · 10 problems

*Count/label groups, reachability, build adjacency lists from edges*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Find Center of Star Graph](https://leetcode.com/problems/find-center-of-star-graph/) | 🟢 Easy |
| ⬜ | 2 | [Find the Town Judge](https://leetcode.com/problems/find-the-town-judge/) | 🟢 Easy |
| ⬜ | 3 | [Keys and Rooms](https://leetcode.com/problems/keys-and-rooms/) | 🟡 Medium |
| ⬜ | 4 | [Number of Provinces](https://leetcode.com/problems/number-of-provinces/) | 🟡 Medium |
| ⬜ | 5 | [All Paths From Source to Target](https://leetcode.com/problems/all-paths-from-source-to-target/) | 🟡 Medium |
| ⬜ | 6 | [Count Sub Islands](https://leetcode.com/problems/count-sub-islands/) | 🟡 Medium |
| ⬜ | 7 | [Reorder Routes to Make All Paths Lead to the City Zero](https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/) | 🟡 Medium |
| ⬜ | 8 | [Detonate the Maximum Bombs](https://leetcode.com/problems/detonate-the-maximum-bombs/) | 🟡 Medium |
| ⬜ | 9 | [Evaluate Division](https://leetcode.com/problems/evaluate-division/) | 🟡 Medium |
| ⬜ | 10 | [Snakes and Ladders](https://leetcode.com/problems/snakes-and-ladders/) | 🟡 Medium |

### 27. Topological Sort

⭐ Core · 10 problems

*Dependencies, ordering, prerequisites, "can everything be finished?"*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Course Schedule](https://leetcode.com/problems/course-schedule/) | 🟡 Medium |
| ⬜ | 2 | [Course Schedule II](https://leetcode.com/problems/course-schedule-ii/) | 🟡 Medium |
| ⬜ | 3 | [Find Eventual Safe States](https://leetcode.com/problems/find-eventual-safe-states/) | 🟡 Medium |
| ⬜ | 4 | [Course Schedule IV](https://leetcode.com/problems/course-schedule-iv/) | 🟡 Medium |
| ⬜ | 5 | [All Ancestors of a Node in a Directed Acyclic Graph](https://leetcode.com/problems/all-ancestors-of-a-node-in-a-directed-acyclic-graph/) | 🟡 Medium |
| ⬜ | 6 | [Find All Possible Recipes from Given Supplies](https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/) | 🟡 Medium |
| ⬜ | 7 | [Minimum Height Trees](https://leetcode.com/problems/minimum-height-trees/) | 🟡 Medium |
| ⬜ | 8 | [Longest Increasing Path in a Matrix](https://leetcode.com/problems/longest-increasing-path-in-a-matrix/) | 🔴 Hard |
| ⬜ | 9 | [Build a Matrix With Conditions](https://leetcode.com/problems/build-a-matrix-with-conditions/) | 🔴 Hard |
| ⬜ | 10 | [Largest Color Value in a Directed Graph](https://leetcode.com/problems/largest-color-value-in-a-directed-graph/) | 🔴 Hard |

### 28. Union-Find (DSU)

⭐ Core · 10 problems

*Dynamic connectivity, grouping, merging accounts/equivalences*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Number of Operations to Make Network Connected](https://leetcode.com/problems/number-of-operations-to-make-network-connected/) | 🟡 Medium |
| ⬜ | 2 | [Redundant Connection](https://leetcode.com/problems/redundant-connection/) | 🟡 Medium |
| ⬜ | 3 | [Satisfiability of Equality Equations](https://leetcode.com/problems/satisfiability-of-equality-equations/) | 🟡 Medium |
| ⬜ | 4 | [Lexicographically Smallest Equivalent String](https://leetcode.com/problems/lexicographically-smallest-equivalent-string/) | 🟡 Medium |
| ⬜ | 5 | [Accounts Merge](https://leetcode.com/problems/accounts-merge/) | 🟡 Medium |
| ⬜ | 6 | [Smallest String With Swaps](https://leetcode.com/problems/smallest-string-with-swaps/) | 🟡 Medium |
| ⬜ | 7 | [Most Stones Removed with Same Row or Column](https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/) | 🟡 Medium |
| ⬜ | 8 | [Regions Cut By Slashes](https://leetcode.com/problems/regions-cut-by-slashes/) | 🟡 Medium |
| ⬜ | 9 | [Find All People With Secret](https://leetcode.com/problems/find-all-people-with-secret/) | 🔴 Hard |
| ⬜ | 10 | [Redundant Connection II](https://leetcode.com/problems/redundant-connection-ii/) | 🔴 Hard |

### 29. Shortest Path (BFS, Dijkstra, Bellman-Ford, Floyd-Warshall)

⭐ Core · 15 problems

*Weighted edges, constraints on stops/cost, "minimum effort/time" on a grid*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Minimum Genetic Mutation](https://leetcode.com/problems/minimum-genetic-mutation/) | 🟡 Medium |
| ⬜ | 2 | [Shortest Path with Alternating Colors](https://leetcode.com/problems/shortest-path-with-alternating-colors/) | 🟡 Medium |
| ⬜ | 3 | [Network Delay Time](https://leetcode.com/problems/network-delay-time/) | 🟡 Medium |
| ⬜ | 4 | [Path with Maximum Probability](https://leetcode.com/problems/path-with-maximum-probability/) | 🟡 Medium |
| ⬜ | 5 | [Cheapest Flights Within K Stops](https://leetcode.com/problems/cheapest-flights-within-k-stops/) | 🟡 Medium |
| ⬜ | 6 | [Find the City With the Smallest Number of Neighbors at a Threshold Distance](https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold/) | 🟡 Medium |
| ⬜ | 7 | [Path With Minimum Effort](https://leetcode.com/problems/path-with-minimum-effort/) | 🟡 Medium |
| ⬜ | 8 | [Number of Ways to Arrive at Destination](https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/) | 🟡 Medium |
| ⬜ | 9 | [Bus Routes](https://leetcode.com/problems/bus-routes/) | 🔴 Hard |
| ⬜ | 10 | [Swim in Rising Water](https://leetcode.com/problems/swim-in-rising-water/) | 🔴 Hard |
| ⬜ | 11 | [Minimum Obstacle Removal to Reach Corner](https://leetcode.com/problems/minimum-obstacle-removal-to-reach-corner/) | 🔴 Hard |
| ⬜ | 12 | [Minimum Cost to Make at Least One Valid Path in a Grid](https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/) | 🔴 Hard |
| ⬜ | 13 | [Shortest Path Visiting All Nodes](https://leetcode.com/problems/shortest-path-visiting-all-nodes/) | 🔴 Hard |
| ⬜ | 14 | [Word Ladder II](https://leetcode.com/problems/word-ladder-ii/) | 🔴 Hard |
| ⬜ | 15 | [Minimum Cost to Reach Destination in Time](https://leetcode.com/problems/minimum-cost-to-reach-destination-in-time/) | 🔴 Hard |

### 30. Cycle Detection & Bipartite Check

📘 Good to know · 10 problems

*Some problems are cross-listed from other sections on purpose*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Linked List Cycle](https://leetcode.com/problems/linked-list-cycle/) | 🟢 Easy |
| ⬜ | 2 | [Linked List Cycle II](https://leetcode.com/problems/linked-list-cycle-ii/) | 🟡 Medium |
| ⬜ | 3 | [Circular Array Loop](https://leetcode.com/problems/circular-array-loop/) | 🟡 Medium |
| ⬜ | 4 | [Course Schedule](https://leetcode.com/problems/course-schedule/) | 🟡 Medium |
| ⬜ | 5 | [Find Eventual Safe States](https://leetcode.com/problems/find-eventual-safe-states/) | 🟡 Medium |
| ⬜ | 6 | [Redundant Connection](https://leetcode.com/problems/redundant-connection/) | 🟡 Medium |
| ⬜ | 7 | [Is Graph Bipartite?](https://leetcode.com/problems/is-graph-bipartite/) | 🟡 Medium |
| ⬜ | 8 | [Possible Bipartition](https://leetcode.com/problems/possible-bipartition/) | 🟡 Medium |
| ⬜ | 9 | [Detect Cycles in 2D Grid](https://leetcode.com/problems/detect-cycles-in-2d-grid/) | 🟡 Medium |
| ⬜ | 10 | [Longest Cycle in a Graph](https://leetcode.com/problems/longest-cycle-in-a-graph/) | 🔴 Hard |

### 31. MST, Bridges & Advanced Graph

🧗 Advanced / rare · 10 problems

*Rare outside top-tier interviews; learn Kruskal/Prim and Tarjan only after the core graph sections*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Min Cost to Connect All Points](https://leetcode.com/problems/min-cost-to-connect-all-points/) | 🟡 Medium |
| ⬜ | 2 | [Minimum Score of a Path Between Two Cities](https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities/) | 🟡 Medium |
| ⬜ | 3 | [Critical Connections in a Network](https://leetcode.com/problems/critical-connections-in-a-network/) | 🔴 Hard |
| ⬜ | 4 | [Minimum Number of Days to Disconnect Island](https://leetcode.com/problems/minimum-number-of-days-to-disconnect-island/) | 🔴 Hard |
| ⬜ | 5 | [Remove Max Number of Edges to Keep Graph Fully Traversable](https://leetcode.com/problems/remove-max-number-of-edges-to-keep-graph-fully-traversable/) | 🔴 Hard |
| ⬜ | 6 | [Checking Existence of Edge Length Limited Paths](https://leetcode.com/problems/checking-existence-of-edge-length-limited-paths/) | 🔴 Hard |
| ⬜ | 7 | [Find Critical and Pseudo-Critical Edges in Minimum Spanning Tree](https://leetcode.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree/) | 🔴 Hard |
| ⬜ | 8 | [Number of Good Paths](https://leetcode.com/problems/number-of-good-paths/) | 🔴 Hard |
| ⬜ | 9 | [Reconstruct Itinerary](https://leetcode.com/problems/reconstruct-itinerary/) | 🔴 Hard |
| ⬜ | 10 | [Cracking the Safe](https://leetcode.com/problems/cracking-the-safe/) | 🔴 Hard |

---

## Heap / Priority Queue

### 32. Top-K Elements

⭐ Core · 15 problems

*K largest/smallest/most frequent: keep a heap of size K*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Kth Largest Element in a Stream](https://leetcode.com/problems/kth-largest-element-in-a-stream/) | 🟢 Easy |
| ⬜ | 2 | [Last Stone Weight](https://leetcode.com/problems/last-stone-weight/) | 🟢 Easy |
| ⬜ | 3 | [Relative Ranks](https://leetcode.com/problems/relative-ranks/) | 🟢 Easy |
| ⬜ | 4 | [The K Weakest Rows in a Matrix](https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/) | 🟢 Easy |
| ⬜ | 5 | [Kth Largest Element in an Array](https://leetcode.com/problems/kth-largest-element-in-an-array/) | 🟡 Medium |
| ⬜ | 6 | [K Closest Points to Origin](https://leetcode.com/problems/k-closest-points-to-origin/) | 🟡 Medium |
| ⬜ | 7 | [Top K Frequent Words](https://leetcode.com/problems/top-k-frequent-words/) | 🟡 Medium |
| ⬜ | 8 | [Sort Characters By Frequency](https://leetcode.com/problems/sort-characters-by-frequency/) | 🟡 Medium |
| ⬜ | 9 | [Reorganize String](https://leetcode.com/problems/reorganize-string/) | 🟡 Medium |
| ⬜ | 10 | [Task Scheduler](https://leetcode.com/problems/task-scheduler/) | 🟡 Medium |
| ⬜ | 11 | [Least Number of Unique Integers after K Removals](https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/) | 🟡 Medium |
| ⬜ | 12 | [Furthest Building You Can Reach](https://leetcode.com/problems/furthest-building-you-can-reach/) | 🟡 Medium |
| ⬜ | 13 | [Ugly Number II](https://leetcode.com/problems/ugly-number-ii/) | 🟡 Medium |
| ⬜ | 14 | [Find K Pairs with Smallest Sums](https://leetcode.com/problems/find-k-pairs-with-smallest-sums/) | 🟡 Medium |
| ⬜ | 15 | [Minimum Cost to Hire K Workers](https://leetcode.com/problems/minimum-cost-to-hire-k-workers/) | 🔴 Hard |

### 33. K-Way Merge

📘 Good to know · 10 problems

*Merge several sorted sources by always popping the smallest head*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/) | 🟢 Easy |
| ⬜ | 2 | [Merge Two 2D Arrays by Summing Values](https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values/) | 🟢 Easy |
| ⬜ | 3 | [Intersection of Two Arrays II](https://leetcode.com/problems/intersection-of-two-arrays-ii/) | 🟢 Easy |
| ⬜ | 4 | [Super Ugly Number](https://leetcode.com/problems/super-ugly-number/) | 🟡 Medium |
| ⬜ | 5 | [Find K Pairs with Smallest Sums](https://leetcode.com/problems/find-k-pairs-with-smallest-sums/) | 🟡 Medium |
| ⬜ | 6 | [Kth Smallest Element in a Sorted Matrix](https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/) | 🟡 Medium |
| ⬜ | 7 | [Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/) | 🔴 Hard |
| ⬜ | 8 | [Smallest Range Covering Elements from K Lists](https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/) | 🔴 Hard |
| ⬜ | 9 | [Find the K-th Smallest Sum of a Matrix With Sorted Rows](https://leetcode.com/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows/) | 🔴 Hard |
| ⬜ | 10 | [Kth Smallest Product of Two Sorted Arrays](https://leetcode.com/problems/kth-smallest-product-of-two-sorted-arrays/) | 🔴 Hard |

### 34. Two Heaps & Scheduling with Heaps

⭐ Core · 10 problems

*Running median, "always pick the best available", resource scheduling*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Seat Reservation Manager](https://leetcode.com/problems/seat-reservation-manager/) | 🟡 Medium |
| ⬜ | 2 | [Smallest Number in Infinite Set](https://leetcode.com/problems/smallest-number-in-infinite-set/) | 🟡 Medium |
| ⬜ | 3 | [Total Cost to Hire K Workers](https://leetcode.com/problems/total-cost-to-hire-k-workers/) | 🟡 Medium |
| ⬜ | 4 | [Single-Threaded CPU](https://leetcode.com/problems/single-threaded-cpu/) | 🟡 Medium |
| ⬜ | 5 | [Process Tasks Using Servers](https://leetcode.com/problems/process-tasks-using-servers/) | 🟡 Medium |
| ⬜ | 6 | [Maximum Subsequence Score](https://leetcode.com/problems/maximum-subsequence-score/) | 🟡 Medium |
| ⬜ | 7 | [IPO](https://leetcode.com/problems/ipo/) | 🔴 Hard |
| ⬜ | 8 | [Minimum Number of Refueling Stops](https://leetcode.com/problems/minimum-number-of-refueling-stops/) | 🔴 Hard |
| ⬜ | 9 | [Find Median from Data Stream](https://leetcode.com/problems/find-median-from-data-stream/) | 🔴 Hard |
| ⬜ | 10 | [Sliding Window Median](https://leetcode.com/problems/sliding-window-median/) | 🔴 Hard |

---

## Recursion & Backtracking

### 35. Subsets, Permutations & Combinations

⭐ Core · 15 problems

*Enumerate all choices: choose, recurse, un-choose*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Subsets](https://leetcode.com/problems/subsets/) | 🟡 Medium |
| ⬜ | 2 | [Subsets II](https://leetcode.com/problems/subsets-ii/) | 🟡 Medium |
| ⬜ | 3 | [Combinations](https://leetcode.com/problems/combinations/) | 🟡 Medium |
| ⬜ | 4 | [Permutations](https://leetcode.com/problems/permutations/) | 🟡 Medium |
| ⬜ | 5 | [Permutations II](https://leetcode.com/problems/permutations-ii/) | 🟡 Medium |
| ⬜ | 6 | [Combination Sum](https://leetcode.com/problems/combination-sum/) | 🟡 Medium |
| ⬜ | 7 | [Combination Sum II](https://leetcode.com/problems/combination-sum-ii/) | 🟡 Medium |
| ⬜ | 8 | [Combination Sum III](https://leetcode.com/problems/combination-sum-iii/) | 🟡 Medium |
| ⬜ | 9 | [Letter Combinations of a Phone Number](https://leetcode.com/problems/letter-combinations-of-a-phone-number/) | 🟡 Medium |
| ⬜ | 10 | [Generate Parentheses](https://leetcode.com/problems/generate-parentheses/) | 🟡 Medium |
| ⬜ | 11 | [Letter Case Permutation](https://leetcode.com/problems/letter-case-permutation/) | 🟡 Medium |
| ⬜ | 12 | [Non-decreasing Subsequences](https://leetcode.com/problems/non-decreasing-subsequences/) | 🟡 Medium |
| ⬜ | 13 | [Beautiful Arrangement](https://leetcode.com/problems/beautiful-arrangement/) | 🟡 Medium |
| ⬜ | 14 | [Gray Code](https://leetcode.com/problems/gray-code/) | 🟡 Medium |
| ⬜ | 15 | [Permutation Sequence](https://leetcode.com/problems/permutation-sequence/) | 🔴 Hard |

### 36. Constraint Search (Grid & Board Problems)

📘 Good to know · 10 problems

*Place/choose with pruning: boards, words, grids*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Path with Maximum Gold](https://leetcode.com/problems/path-with-maximum-gold/) | 🟡 Medium |
| ⬜ | 2 | [Word Search](https://leetcode.com/problems/word-search/) | 🟡 Medium |
| ⬜ | 3 | [Maximum Length of a Concatenated String with Unique Characters](https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/) | 🟡 Medium |
| ⬜ | 4 | [Split a String Into the Max Number of Unique Substrings](https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/) | 🟡 Medium |
| ⬜ | 5 | [Matchsticks to Square](https://leetcode.com/problems/matchsticks-to-square/) | 🟡 Medium |
| ⬜ | 6 | [Unique Paths III](https://leetcode.com/problems/unique-paths-iii/) | 🔴 Hard |
| ⬜ | 7 | [N-Queens](https://leetcode.com/problems/n-queens/) | 🔴 Hard |
| ⬜ | 8 | [N-Queens II](https://leetcode.com/problems/n-queens-ii/) | 🔴 Hard |
| ⬜ | 9 | [Sudoku Solver](https://leetcode.com/problems/sudoku-solver/) | 🔴 Hard |
| ⬜ | 10 | [Remove Invalid Parentheses](https://leetcode.com/problems/remove-invalid-parentheses/) | 🔴 Hard |

### 37. Partition Problems

📘 Good to know · 10 problems

*Split a string/array into valid pieces, or into k groups*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Palindrome Partitioning](https://leetcode.com/problems/palindrome-partitioning/) | 🟡 Medium |
| ⬜ | 2 | [Restore IP Addresses](https://leetcode.com/problems/restore-ip-addresses/) | 🟡 Medium |
| ⬜ | 3 | [Additive Number](https://leetcode.com/problems/additive-number/) | 🟡 Medium |
| ⬜ | 4 | [Splitting a String Into Descending Consecutive Values](https://leetcode.com/problems/splitting-a-string-into-descending-consecutive-values/) | 🟡 Medium |
| ⬜ | 5 | [Fair Distribution of Cookies](https://leetcode.com/problems/fair-distribution-of-cookies/) | 🟡 Medium |
| ⬜ | 6 | [Partition to K Equal Sum Subsets](https://leetcode.com/problems/partition-to-k-equal-sum-subsets/) | 🟡 Medium |
| ⬜ | 7 | [Find Unique Binary String](https://leetcode.com/problems/find-unique-binary-string/) | 🟡 Medium |
| ⬜ | 8 | [The Number of Beautiful Subsets](https://leetcode.com/problems/the-number-of-beautiful-subsets/) | 🟡 Medium |
| ⬜ | 9 | [Word Break II](https://leetcode.com/problems/word-break-ii/) | 🔴 Hard |
| ⬜ | 10 | [Expression Add Operators](https://leetcode.com/problems/expression-add-operators/) | 🔴 Hard |

---

## Dynamic Programming

### 38. 1D DP

⭐ Core · 15 problems

*dp[i] depends on a few earlier states: stairs, rob, decode, break*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Fibonacci Number](https://leetcode.com/problems/fibonacci-number/) | 🟢 Easy |
| ⬜ | 2 | [Climbing Stairs](https://leetcode.com/problems/climbing-stairs/) | 🟢 Easy |
| ⬜ | 3 | [Min Cost Climbing Stairs](https://leetcode.com/problems/min-cost-climbing-stairs/) | 🟢 Easy |
| ⬜ | 4 | [N-th Tribonacci Number](https://leetcode.com/problems/n-th-tribonacci-number/) | 🟢 Easy |
| ⬜ | 5 | [Counting Bits](https://leetcode.com/problems/counting-bits/) | 🟢 Easy |
| ⬜ | 6 | [House Robber](https://leetcode.com/problems/house-robber/) | 🟡 Medium |
| ⬜ | 7 | [House Robber II](https://leetcode.com/problems/house-robber-ii/) | 🟡 Medium |
| ⬜ | 8 | [Decode Ways](https://leetcode.com/problems/decode-ways/) | 🟡 Medium |
| ⬜ | 9 | [Integer Break](https://leetcode.com/problems/integer-break/) | 🟡 Medium |
| ⬜ | 10 | [Delete and Earn](https://leetcode.com/problems/delete-and-earn/) | 🟡 Medium |
| ⬜ | 11 | [Minimum Cost For Tickets](https://leetcode.com/problems/minimum-cost-for-tickets/) | 🟡 Medium |
| ⬜ | 12 | [Solving Questions With Brainpower](https://leetcode.com/problems/solving-questions-with-brainpower/) | 🟡 Medium |
| ⬜ | 13 | [Domino and Tromino Tiling](https://leetcode.com/problems/domino-and-tromino-tiling/) | 🟡 Medium |
| ⬜ | 14 | [Count Ways To Build Good Strings](https://leetcode.com/problems/count-ways-to-build-good-strings/) | 🟡 Medium |
| ⬜ | 15 | [Word Break](https://leetcode.com/problems/word-break/) | 🟡 Medium |

### 39. Knapsack (0/1, Unbounded, Subset Sum)

⭐ Core · 10 problems

*Pick items under a capacity/target; count ways or find min/max*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Perfect Squares](https://leetcode.com/problems/perfect-squares/) | 🟡 Medium |
| ⬜ | 2 | [Coin Change](https://leetcode.com/problems/coin-change/) | 🟡 Medium |
| ⬜ | 3 | [Partition Equal Subset Sum](https://leetcode.com/problems/partition-equal-subset-sum/) | 🟡 Medium |
| ⬜ | 4 | [Coin Change II](https://leetcode.com/problems/coin-change-ii/) | 🟡 Medium |
| ⬜ | 5 | [Combination Sum IV](https://leetcode.com/problems/combination-sum-iv/) | 🟡 Medium |
| ⬜ | 6 | [Target Sum](https://leetcode.com/problems/target-sum/) | 🟡 Medium |
| ⬜ | 7 | [Last Stone Weight II](https://leetcode.com/problems/last-stone-weight-ii/) | 🟡 Medium |
| ⬜ | 8 | [Ones and Zeroes](https://leetcode.com/problems/ones-and-zeroes/) | 🟡 Medium |
| ⬜ | 9 | [Number of Dice Rolls With Target Sum](https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/) | 🟡 Medium |
| ⬜ | 10 | [Profitable Schemes](https://leetcode.com/problems/profitable-schemes/) | 🔴 Hard |

### 40. Subsequence DP (LIS, LCS, Edit Distance)

⭐ Core · 15 problems

*Two strings/arrays compared index by index, or best subsequence ending at i*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Longest Increasing Subsequence](https://leetcode.com/problems/longest-increasing-subsequence/) | 🟡 Medium |
| ⬜ | 2 | [Longest Common Subsequence](https://leetcode.com/problems/longest-common-subsequence/) | 🟡 Medium |
| ⬜ | 3 | [Maximum Length of Repeated Subarray](https://leetcode.com/problems/maximum-length-of-repeated-subarray/) | 🟡 Medium |
| ⬜ | 4 | [Uncrossed Lines](https://leetcode.com/problems/uncrossed-lines/) | 🟡 Medium |
| ⬜ | 5 | [Longest Palindromic Subsequence](https://leetcode.com/problems/longest-palindromic-subsequence/) | 🟡 Medium |
| ⬜ | 6 | [Delete Operation for Two Strings](https://leetcode.com/problems/delete-operation-for-two-strings/) | 🟡 Medium |
| ⬜ | 7 | [Minimum ASCII Delete Sum for Two Strings](https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/) | 🟡 Medium |
| ⬜ | 8 | [Edit Distance](https://leetcode.com/problems/edit-distance/) | 🟡 Medium |
| ⬜ | 9 | [Number of Longest Increasing Subsequence](https://leetcode.com/problems/number-of-longest-increasing-subsequence/) | 🟡 Medium |
| ⬜ | 10 | [Longest String Chain](https://leetcode.com/problems/longest-string-chain/) | 🟡 Medium |
| ⬜ | 11 | [Longest Arithmetic Subsequence](https://leetcode.com/problems/longest-arithmetic-subsequence/) | 🟡 Medium |
| ⬜ | 12 | [Interleaving String](https://leetcode.com/problems/interleaving-string/) | 🟡 Medium |
| ⬜ | 13 | [Distinct Subsequences](https://leetcode.com/problems/distinct-subsequences/) | 🔴 Hard |
| ⬜ | 14 | [Russian Doll Envelopes](https://leetcode.com/problems/russian-doll-envelopes/) | 🔴 Hard |
| ⬜ | 15 | [Regular Expression Matching](https://leetcode.com/problems/regular-expression-matching/) | 🔴 Hard |

### 41. Grid DP

⭐ Core · 10 problems

*Paths and best values on a matrix, moving right/down*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Unique Paths](https://leetcode.com/problems/unique-paths/) | 🟡 Medium |
| ⬜ | 2 | [Unique Paths II](https://leetcode.com/problems/unique-paths-ii/) | 🟡 Medium |
| ⬜ | 3 | [Minimum Path Sum](https://leetcode.com/problems/minimum-path-sum/) | 🟡 Medium |
| ⬜ | 4 | [Triangle](https://leetcode.com/problems/triangle/) | 🟡 Medium |
| ⬜ | 5 | [Minimum Falling Path Sum](https://leetcode.com/problems/minimum-falling-path-sum/) | 🟡 Medium |
| ⬜ | 6 | [Maximal Square](https://leetcode.com/problems/maximal-square/) | 🟡 Medium |
| ⬜ | 7 | [Count Square Submatrices with All Ones](https://leetcode.com/problems/count-square-submatrices-with-all-ones/) | 🟡 Medium |
| ⬜ | 8 | [Out of Boundary Paths](https://leetcode.com/problems/out-of-boundary-paths/) | 🟡 Medium |
| ⬜ | 9 | [Knight Probability in Chessboard](https://leetcode.com/problems/knight-probability-in-chessboard/) | 🟡 Medium |
| ⬜ | 10 | [Dungeon Game](https://leetcode.com/problems/dungeon-game/) | 🔴 Hard |

### 42. Interval DP

📘 Good to know · 10 problems

*`dp[l][r]` over a range, splitting at every k inside it*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Stone Game](https://leetcode.com/problems/stone-game/) | 🟡 Medium |
| ⬜ | 2 | [Predict the Winner](https://leetcode.com/problems/predict-the-winner/) | 🟡 Medium |
| ⬜ | 3 | [Stone Game II](https://leetcode.com/problems/stone-game-ii/) | 🟡 Medium |
| ⬜ | 4 | [Guess Number Higher or Lower II](https://leetcode.com/problems/guess-number-higher-or-lower-ii/) | 🟡 Medium |
| ⬜ | 5 | [Minimum Score Triangulation of Polygon](https://leetcode.com/problems/minimum-score-triangulation-of-polygon/) | 🟡 Medium |
| ⬜ | 6 | [Minimum Cost Tree From Leaf Values](https://leetcode.com/problems/minimum-cost-tree-from-leaf-values/) | 🟡 Medium |
| ⬜ | 7 | [Minimum Cost to Cut a Stick](https://leetcode.com/problems/minimum-cost-to-cut-a-stick/) | 🔴 Hard |
| ⬜ | 8 | [Palindrome Partitioning II](https://leetcode.com/problems/palindrome-partitioning-ii/) | 🔴 Hard |
| ⬜ | 9 | [Burst Balloons](https://leetcode.com/problems/burst-balloons/) | 🔴 Hard |
| ⬜ | 10 | [Strange Printer](https://leetcode.com/problems/strange-printer/) | 🔴 Hard |

### 43. State Machine DP (Stocks & Friends)

📘 Good to know · 10 problems

*Track a small "mode" (holding, cooldown, k transactions) per index*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Best Time to Buy and Sell Stock II](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/) | 🟡 Medium |
| ⬜ | 2 | [Best Sightseeing Pair](https://leetcode.com/problems/best-sightseeing-pair/) | 🟡 Medium |
| ⬜ | 3 | [Best Time to Buy and Sell Stock with Cooldown](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/) | 🟡 Medium |
| ⬜ | 4 | [Best Time to Buy and Sell Stock with Transaction Fee](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/) | 🟡 Medium |
| ⬜ | 5 | [Wiggle Subsequence](https://leetcode.com/problems/wiggle-subsequence/) | 🟡 Medium |
| ⬜ | 6 | [Knight Dialer](https://leetcode.com/problems/knight-dialer/) | 🟡 Medium |
| ⬜ | 7 | [Best Time to Buy and Sell Stock III](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/) | 🔴 Hard |
| ⬜ | 8 | [Best Time to Buy and Sell Stock IV](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/) | 🔴 Hard |
| ⬜ | 9 | [Stone Game III](https://leetcode.com/problems/stone-game-iii/) | 🔴 Hard |
| ⬜ | 10 | [Student Attendance Record II](https://leetcode.com/problems/student-attendance-record-ii/) | 🔴 Hard |

### 44. Bitmask DP

🧗 Advanced / rare · 10 problems

*n is small (about 20): encode the chosen subset as a bitmask*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Count Number of Maximum Bitwise-OR Subsets](https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/) | 🟡 Medium |
| ⬜ | 2 | [Can I Win](https://leetcode.com/problems/can-i-win/) | 🟡 Medium |
| ⬜ | 3 | [Minimum Number of Work Sessions to Finish the Tasks](https://leetcode.com/problems/minimum-number-of-work-sessions-to-finish-the-tasks/) | 🟡 Medium |
| ⬜ | 4 | [Stickers to Spell Word](https://leetcode.com/problems/stickers-to-spell-word/) | 🔴 Hard |
| ⬜ | 5 | [Smallest Sufficient Team](https://leetcode.com/problems/smallest-sufficient-team/) | 🔴 Hard |
| ⬜ | 6 | [Find the Shortest Superstring](https://leetcode.com/problems/find-the-shortest-superstring/) | 🔴 Hard |
| ⬜ | 7 | [Maximum AND Sum of Array](https://leetcode.com/problems/maximum-and-sum-of-array/) | 🔴 Hard |
| ⬜ | 8 | [Number of Ways to Wear Different Hats to Each Other](https://leetcode.com/problems/number-of-ways-to-wear-different-hats-to-each-other/) | 🔴 Hard |
| ⬜ | 9 | [Minimum Cost to Connect Two Groups of Points](https://leetcode.com/problems/minimum-cost-to-connect-two-groups-of-points/) | 🔴 Hard |
| ⬜ | 10 | [Maximum Students Taking Exam](https://leetcode.com/problems/maximum-students-taking-exam/) | 🔴 Hard |

### 45. Digit DP

🧗 Advanced / rare · 7 problems

*Count numbers up to N with a digit property; rare, so only 7 here*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Rotated Digits](https://leetcode.com/problems/rotated-digits/) | 🟡 Medium |
| ⬜ | 2 | [Count Numbers with Unique Digits](https://leetcode.com/problems/count-numbers-with-unique-digits/) | 🟡 Medium |
| ⬜ | 3 | [Number of Digit One](https://leetcode.com/problems/number-of-digit-one/) | 🔴 Hard |
| ⬜ | 4 | [Numbers At Most N Given Digit Set](https://leetcode.com/problems/numbers-at-most-n-given-digit-set/) | 🔴 Hard |
| ⬜ | 5 | [Non-negative Integers without Consecutive Ones](https://leetcode.com/problems/non-negative-integers-without-consecutive-ones/) | 🔴 Hard |
| ⬜ | 6 | [Numbers With Repeated Digits](https://leetcode.com/problems/numbers-with-repeated-digits/) | 🔴 Hard |
| ⬜ | 7 | [Count Special Integers](https://leetcode.com/problems/count-special-integers/) | 🔴 Hard |

---

## Greedy

### 46. Greedy (Jump, Gas Station, Sort + Scan, Exchange Arguments)

⭐ Core · 15 problems

*A locally best choice is provably globally best; usually sort first*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Assign Cookies](https://leetcode.com/problems/assign-cookies/) | 🟢 Easy |
| ⬜ | 2 | [Lemonade Change](https://leetcode.com/problems/lemonade-change/) | 🟢 Easy |
| ⬜ | 3 | [Maximum Units on a Truck](https://leetcode.com/problems/maximum-units-on-a-truck/) | 🟢 Easy |
| ⬜ | 4 | [Jump Game](https://leetcode.com/problems/jump-game/) | 🟡 Medium |
| ⬜ | 5 | [Jump Game II](https://leetcode.com/problems/jump-game-ii/) | 🟡 Medium |
| ⬜ | 6 | [Gas Station](https://leetcode.com/problems/gas-station/) | 🟡 Medium |
| ⬜ | 7 | [Hand of Straights](https://leetcode.com/problems/hand-of-straights/) | 🟡 Medium |
| ⬜ | 8 | [Merge Triplets to Form Target Triplet](https://leetcode.com/problems/merge-triplets-to-form-target-triplet/) | 🟡 Medium |
| ⬜ | 9 | [Valid Parenthesis String](https://leetcode.com/problems/valid-parenthesis-string/) | 🟡 Medium |
| ⬜ | 10 | [Two City Scheduling](https://leetcode.com/problems/two-city-scheduling/) | 🟡 Medium |
| ⬜ | 11 | [Queue Reconstruction by Height](https://leetcode.com/problems/queue-reconstruction-by-height/) | 🟡 Medium |
| ⬜ | 12 | [Largest Number](https://leetcode.com/problems/largest-number/) | 🟡 Medium |
| ⬜ | 13 | [Bag of Tokens](https://leetcode.com/problems/bag-of-tokens/) | 🟡 Medium |
| ⬜ | 14 | [Split Array into Consecutive Subsequences](https://leetcode.com/problems/split-array-into-consecutive-subsequences/) | 🟡 Medium |
| ⬜ | 15 | [Candy](https://leetcode.com/problems/candy/) | 🔴 Hard |

---

## Other Patterns

### 47. Trie (Prefix Tree)

⭐ Core · 10 problems

*Prefix search, autocomplete, word dictionaries, XOR tricks*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Implement Trie (Prefix Tree)](https://leetcode.com/problems/implement-trie-prefix-tree/) | 🟡 Medium |
| ⬜ | 2 | [Replace Words](https://leetcode.com/problems/replace-words/) | 🟡 Medium |
| ⬜ | 3 | [Map Sum Pairs](https://leetcode.com/problems/map-sum-pairs/) | 🟡 Medium |
| ⬜ | 4 | [Longest Word in Dictionary](https://leetcode.com/problems/longest-word-in-dictionary/) | 🟡 Medium |
| ⬜ | 5 | [Design Add and Search Words Data Structure](https://leetcode.com/problems/design-add-and-search-words-data-structure/) | 🟡 Medium |
| ⬜ | 6 | [Search Suggestions System](https://leetcode.com/problems/search-suggestions-system/) | 🟡 Medium |
| ⬜ | 7 | [Short Encoding of Words](https://leetcode.com/problems/short-encoding-of-words/) | 🟡 Medium |
| ⬜ | 8 | [Maximum XOR of Two Numbers in an Array](https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/) | 🟡 Medium |
| ⬜ | 9 | [Word Search II](https://leetcode.com/problems/word-search-ii/) | 🔴 Hard |
| ⬜ | 10 | [Stream of Characters](https://leetcode.com/problems/stream-of-characters/) | 🔴 Hard |

### 48. Bit Manipulation

📘 Good to know · 10 problems

*XOR cancellation, masks, shifting, power-of-two tricks*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Single Number](https://leetcode.com/problems/single-number/) | 🟢 Easy |
| ⬜ | 2 | [Number of 1 Bits](https://leetcode.com/problems/number-of-1-bits/) | 🟢 Easy |
| ⬜ | 3 | [Power of Two](https://leetcode.com/problems/power-of-two/) | 🟢 Easy |
| ⬜ | 4 | [Hamming Distance](https://leetcode.com/problems/hamming-distance/) | 🟢 Easy |
| ⬜ | 5 | [Number Complement](https://leetcode.com/problems/number-complement/) | 🟢 Easy |
| ⬜ | 6 | [Reverse Bits](https://leetcode.com/problems/reverse-bits/) | 🟢 Easy |
| ⬜ | 7 | [Sum of Two Integers](https://leetcode.com/problems/sum-of-two-integers/) | 🟡 Medium |
| ⬜ | 8 | [Single Number II](https://leetcode.com/problems/single-number-ii/) | 🟡 Medium |
| ⬜ | 9 | [Single Number III](https://leetcode.com/problems/single-number-iii/) | 🟡 Medium |
| ⬜ | 10 | [Bitwise AND of Numbers Range](https://leetcode.com/problems/bitwise-and-of-numbers-range/) | 🟡 Medium |

### 49. Math & Number Theory

📘 Good to know · 10 problems

*Digits, primes, gcd, fast exponentiation, overflow care*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Palindrome Number](https://leetcode.com/problems/palindrome-number/) | 🟢 Easy |
| ⬜ | 2 | [Fizz Buzz](https://leetcode.com/problems/fizz-buzz/) | 🟢 Easy |
| ⬜ | 3 | [Plus One](https://leetcode.com/problems/plus-one/) | 🟢 Easy |
| ⬜ | 4 | [Excel Sheet Column Number](https://leetcode.com/problems/excel-sheet-column-number/) | 🟢 Easy |
| ⬜ | 5 | [Greatest Common Divisor of Strings](https://leetcode.com/problems/greatest-common-divisor-of-strings/) | 🟢 Easy |
| ⬜ | 6 | [Reverse Integer](https://leetcode.com/problems/reverse-integer/) | 🟡 Medium |
| ⬜ | 7 | [Factorial Trailing Zeroes](https://leetcode.com/problems/factorial-trailing-zeroes/) | 🟡 Medium |
| ⬜ | 8 | [Count Primes](https://leetcode.com/problems/count-primes/) | 🟡 Medium |
| ⬜ | 9 | [Pow(x, n)](https://leetcode.com/problems/powx-n/) | 🟡 Medium |
| ⬜ | 10 | [Fraction to Recurring Decimal](https://leetcode.com/problems/fraction-to-recurring-decimal/) | 🟡 Medium |

### 50. Design Problems

⭐ Core · 15 problems

*Combine data structures to hit target complexities (hash map + list, heap, etc.)*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Design Parking System](https://leetcode.com/problems/design-parking-system/) | 🟢 Easy |
| ⬜ | 2 | [Design HashSet](https://leetcode.com/problems/design-hashset/) | 🟢 Easy |
| ⬜ | 3 | [Design HashMap](https://leetcode.com/problems/design-hashmap/) | 🟢 Easy |
| ⬜ | 4 | [Design an Ordered Stream](https://leetcode.com/problems/design-an-ordered-stream/) | 🟢 Easy |
| ⬜ | 5 | [Design Browser History](https://leetcode.com/problems/design-browser-history/) | 🟡 Medium |
| ⬜ | 6 | [LRU Cache](https://leetcode.com/problems/lru-cache/) | 🟡 Medium |
| ⬜ | 7 | [Time Based Key-Value Store](https://leetcode.com/problems/time-based-key-value-store/) | 🟡 Medium |
| ⬜ | 8 | [Design Underground System](https://leetcode.com/problems/design-underground-system/) | 🟡 Medium |
| ⬜ | 9 | [Design Twitter](https://leetcode.com/problems/design-twitter/) | 🟡 Medium |
| ⬜ | 10 | [Snapshot Array](https://leetcode.com/problems/snapshot-array/) | 🟡 Medium |
| ⬜ | 11 | [Product of the Last K Numbers](https://leetcode.com/problems/product-of-the-last-k-numbers/) | 🟡 Medium |
| ⬜ | 12 | [Stock Price Fluctuation](https://leetcode.com/problems/stock-price-fluctuation/) | 🟡 Medium |
| ⬜ | 13 | [Design a Food Rating System](https://leetcode.com/problems/design-a-food-rating-system/) | 🟡 Medium |
| ⬜ | 14 | [LFU Cache](https://leetcode.com/problems/lfu-cache/) | 🔴 Hard |
| ⬜ | 15 | [All O'one Data Structure](https://leetcode.com/problems/all-oone-data-structure/) | 🔴 Hard |

### 51. Randomization & Reservoir Sampling

📘 Good to know · 10 problems

*Uniform random picks from streams, weighted picks, rejection sampling*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Shuffle an Array](https://leetcode.com/problems/shuffle-an-array/) | 🟡 Medium |
| ⬜ | 2 | [Linked List Random Node](https://leetcode.com/problems/linked-list-random-node/) | 🟡 Medium |
| ⬜ | 3 | [Random Pick Index](https://leetcode.com/problems/random-pick-index/) | 🟡 Medium |
| ⬜ | 4 | [Random Pick with Weight](https://leetcode.com/problems/random-pick-with-weight/) | 🟡 Medium |
| ⬜ | 5 | [Implement Rand10() Using Rand7()](https://leetcode.com/problems/implement-rand10-using-rand7/) | 🟡 Medium |
| ⬜ | 6 | [Generate Random Point in a Circle](https://leetcode.com/problems/generate-random-point-in-a-circle/) | 🟡 Medium |
| ⬜ | 7 | [Random Flip Matrix](https://leetcode.com/problems/random-flip-matrix/) | 🟡 Medium |
| ⬜ | 8 | [Random Point in Non-overlapping Rectangles](https://leetcode.com/problems/random-point-in-non-overlapping-rectangles/) | 🟡 Medium |
| ⬜ | 9 | [Random Pick with Blacklist](https://leetcode.com/problems/random-pick-with-blacklist/) | 🔴 Hard |
| ⬜ | 10 | [Insert Delete GetRandom O(1) - Duplicates allowed](https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/) | 🔴 Hard |

### 52. Segment Tree & Fenwick Tree

🧗 Advanced / rare · 10 problems

*Range queries with point/range updates, counting inversions*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Range Sum Query - Mutable](https://leetcode.com/problems/range-sum-query-mutable/) | 🟡 Medium |
| ⬜ | 2 | [Count of Smaller Numbers After Self](https://leetcode.com/problems/count-of-smaller-numbers-after-self/) | 🔴 Hard |
| ⬜ | 3 | [Reverse Pairs](https://leetcode.com/problems/reverse-pairs/) | 🔴 Hard |
| ⬜ | 4 | [Count of Range Sum](https://leetcode.com/problems/count-of-range-sum/) | 🔴 Hard |
| ⬜ | 5 | [My Calendar III](https://leetcode.com/problems/my-calendar-iii/) | 🔴 Hard |
| ⬜ | 6 | [Falling Squares](https://leetcode.com/problems/falling-squares/) | 🔴 Hard |
| ⬜ | 7 | [Create Sorted Array through Instructions](https://leetcode.com/problems/create-sorted-array-through-instructions/) | 🔴 Hard |
| ⬜ | 8 | [Longest Increasing Subsequence II](https://leetcode.com/problems/longest-increasing-subsequence-ii/) | 🔴 Hard |
| ⬜ | 9 | [Range Module](https://leetcode.com/problems/range-module/) | 🔴 Hard |
| ⬜ | 10 | [Count Good Triplets in an Array](https://leetcode.com/problems/count-good-triplets-in-an-array/) | 🔴 Hard |

### 53. String Algorithms (KMP, Rabin-Karp, Z-Function)

🧗 Advanced / rare · 10 problems

*Fast substring matching, borders, rolling hashes*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Rotate String](https://leetcode.com/problems/rotate-string/) | 🟢 Easy |
| ⬜ | 2 | [Repeated Substring Pattern](https://leetcode.com/problems/repeated-substring-pattern/) | 🟢 Easy |
| ⬜ | 3 | [Find the Index of the First Occurrence in a String](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/) | 🟢 Easy |
| ⬜ | 4 | [Repeated String Match](https://leetcode.com/problems/repeated-string-match/) | 🟡 Medium |
| ⬜ | 5 | [Palindromic Substrings](https://leetcode.com/problems/palindromic-substrings/) | 🟡 Medium |
| ⬜ | 6 | [Longest Happy Prefix](https://leetcode.com/problems/longest-happy-prefix/) | 🔴 Hard |
| ⬜ | 7 | [Shortest Palindrome](https://leetcode.com/problems/shortest-palindrome/) | 🔴 Hard |
| ⬜ | 8 | [Sum of Scores of Built Strings](https://leetcode.com/problems/sum-of-scores-of-built-strings/) | 🔴 Hard |
| ⬜ | 9 | [Longest Duplicate Substring](https://leetcode.com/problems/longest-duplicate-substring/) | 🔴 Hard |
| ⬜ | 10 | [Distinct Echo Substrings](https://leetcode.com/problems/distinct-echo-substrings/) | 🔴 Hard |

### 54. Geometry

🧗 Advanced / rare · 10 problems

*Rare; mostly arithmetic on points, slopes, and rectangles*

| Done | # | Problem | Difficulty |
|:---:|:---:|---|:---:|
| ⬜ | 1 | [Check If It Is a Straight Line](https://leetcode.com/problems/check-if-it-is-a-straight-line/) | 🟢 Easy |
| ⬜ | 2 | [Minimum Time Visiting All Points](https://leetcode.com/problems/minimum-time-visiting-all-points/) | 🟢 Easy |
| ⬜ | 3 | [Valid Boomerang](https://leetcode.com/problems/valid-boomerang/) | 🟢 Easy |
| ⬜ | 4 | [Largest Triangle Area](https://leetcode.com/problems/largest-triangle-area/) | 🟢 Easy |
| ⬜ | 5 | [Rectangle Overlap](https://leetcode.com/problems/rectangle-overlap/) | 🟢 Easy |
| ⬜ | 6 | [Rectangle Area](https://leetcode.com/problems/rectangle-area/) | 🟡 Medium |
| ⬜ | 7 | [Valid Square](https://leetcode.com/problems/valid-square/) | 🟡 Medium |
| ⬜ | 8 | [Minimum Area Rectangle](https://leetcode.com/problems/minimum-area-rectangle/) | 🟡 Medium |
| ⬜ | 9 | [Max Points on a Line](https://leetcode.com/problems/max-points-on-a-line/) | 🔴 Hard |
| ⬜ | 10 | [Erect the Fence](https://leetcode.com/problems/erect-the-fence/) | 🔴 Hard |

---
