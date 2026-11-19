# LeetCode Interview Patterns: Problem Set (ranked by real interview-frequency data)

**55 patterns · 559 problems** (549 unique). Problems were chosen and ranked from company-tagged frequency data pulled from four independent datasets, not from a curated list. **Coverage:** 100 of the 100 most-asked free problems, 200 of the top 200, and 300 of the top 300 are in this file.

## Index

- [Pattern Based Index](#pattern-based-index)
- [Top 100](#the-100-most-asked-problems)

## How the data was built

LeetCode shows which companies asked a problem and how often, but only to premium users, and I can't call its API from here. Community members have scraped that data into public GitHub repos. I used four of them, from different dates, so one scraper's gaps or one era's trends can't dominate:

| Dataset                                                                                                                           | Snapshot | What it holds                                                                                       |
| --------------------------------------------------------------------------------------------------------------------------------- | -------- | --------------------------------------------------------------------------------------------------- |
| [liquidslr/leetcode-company-wise-problems](https://github.com/liquidslr/leetcode-company-wise-problems)                           | Oct 2026 | 470 companies, with 30-day / 3-month / 6-month / older / all-time windows and per-company frequency |
| [snehasishroy/leetcode-companywise-interview-questions](https://github.com/snehasishroy/leetcode-companywise-interview-questions) | Jul 2026 | 660 companies, same windows, independent scrape                                                     |
| [krishnadey30/LeetCode-Questions-CompanyWise](https://github.com/krishnadey30/LeetCode-Questions-CompanyWise)                     | Jun 2024 | about 200 companies, all-time and 1-year windows                                                    |
| [hxu296/leetcode-company-wise-problems-2022](https://github.com/hxu296/leetcode-company-wise-problems-2022)                       | May 2022 | raw per-company occurrence counts                                                                   |

**Score** (0-100, higher = asked more): a weighted percentile across 3,503 problems that appear in these lists. Weights: 40% frequency in the last six months at the two newest datasets, 30% number of companies that asked it (all-time), 15% each for the 2024 and 2022 datasets (only counted for problems that existed then). The **#rank** you see is the global rank by that score.

**Free problems only.** I checked every problem against a [database of LeetCode's free problems](https://github.com/neenza/leetcode-problems) (Aug 2025 snapshot), so you shouldn't hit a paywall. Problems added after that snapshot are excluded.

**Quality bar:** a problem has to be in the global top 1,000 (roughly: asked at 5 or more companies) to be included.

**Caveats:** company tags are crowd-reported, not official interview logs. Frequency is relative within each company. Big-tech and online-assessment questions are over-represented, and many of the 470-660 companies are small. Treat the numbers as a strong signal, not ground truth.

## How to use this

- Go **Core → Common → Occasional**. Tiers come from the data: a pattern is **Core** if several of its problems are among the 300 most-asked overall (or 3+ in the top 100).
- Inside each pattern, problems run easy → hard, so you build up. Use the **#rank** and **Asked at** columns to prioritise if you're short on time.
- 🧗 **Occasional** doesn't mean never: these patterns show up at stronger companies and in systems-flavoured interviews. They're just asked less often than the basics.
- You're done with a pattern when you can solve fresh mediums from it in 25-30 minutes without hints. Redo anything you failed after 5-7 days.

**Legend:** 🟢 Easy · 🟡 Medium · 🔴 Hard · 🔥🔥🔥 top 100 overall · 🔥🔥 top 400 · 🔥 top 1,000 · **Asked at** = companies (all-time) / companies in the last six months, averaged over the two newest datasets

## The 100 most-asked problems

|  #  | Problem                                                                                                                         | Difficulty | Asked at | In pattern                                                  |
| :-: | ------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | ----------------------------------------------------------- |
|  1  | [LRU Cache](https://leetcode.com/problems/lru-cache/)                                                                           | 🟡 Medium  | 111 / 29 | Design Problems                                             |
|  2  | [Two Sum](https://leetcode.com/problems/two-sum/)                                                                               |  🟢 Easy   | 120 / 25 | Hashing / Frequency Counting                                |
|  3  | [Merge Intervals](https://leetcode.com/problems/merge-intervals/)                                                               | 🟡 Medium  | 106 / 29 | Merge Intervals & Sweep Line                                |
|  4  | [Number of Islands](https://leetcode.com/problems/number-of-islands/)                                                           | 🟡 Medium  | 82 / 24  | DFS / BFS on Grids & Graphs                                 |
|  5  | [Valid Parentheses](https://leetcode.com/problems/valid-parentheses/)                                                           |  🟢 Easy   | 102 / 18 | Stack (Matching & Expression Evaluation)                    |
|  6  | [Best Time to Buy and Sell Stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/)                               |  🟢 Easy   | 86 / 20  | Sliding Window                                              |
|  7  | [Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/) | 🟡 Medium  | 100 / 28 | Sliding Window                                              |
|  8  | [Group Anagrams](https://leetcode.com/problems/group-anagrams/)                                                                 | 🟡 Medium  | 80 / 22  | Hashing / Frequency Counting                                |
|  9  | [Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/)                                                       |  🔴 Hard   | 71 / 14  | Two Pointers                                                |
| 10  | [3Sum](https://leetcode.com/problems/3sum/)                                                                                     | 🟡 Medium  | 52 / 12  | Two Pointers                                                |
| 11  | [Maximum Subarray](https://leetcode.com/problems/maximum-subarray/)                                                             | 🟡 Medium  | 60 / 11  | Kadane's Algorithm                                          |
| 12  | [Sliding Window Maximum](https://leetcode.com/problems/sliding-window-maximum/)                                                 |  🔴 Hard   | 48 / 13  | Queue & Monotonic Deque                                     |
| 13  | [Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/)                                                     |  🔴 Hard   | 48 / 12  | Dummy Head & Merging                                        |
| 14  | [Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/)                                 | 🟡 Medium  | 49 / 11  | Search in Rotated / Modified Arrays                         |
| 15  | [Subarray Sum Equals K](https://leetcode.com/problems/subarray-sum-equals-k/)                                                   | 🟡 Medium  | 44 / 14  | Hashing / Frequency Counting                                |
| 16  | [Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/)                                       |  🔴 Hard   | 42 / 10  | Binary Search                                               |
| 17  | [Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/)                                                   |  🟢 Easy   | 56 / 12  | String Manipulation & Parsing                               |
| 18  | [Move Zeroes](https://leetcode.com/problems/move-zeroes/)                                                                       |  🟢 Easy   | 42 / 12  | Two Pointers                                                |
| 19  | [Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/)                                   | 🟡 Medium  | 73 / 13  | String Manipulation & Parsing                               |
| 20  | [Decode String](https://leetcode.com/problems/decode-string/)                                                                   | 🟡 Medium  | 40 / 11  | Stack (Matching & Expression Evaluation)                    |
| 21  | [Top K Frequent Elements](https://leetcode.com/problems/top-k-frequent-elements/)                                               | 🟡 Medium  | 51 / 13  | Hashing / Frequency Counting                                |
| 22  | [Add Two Numbers](https://leetcode.com/problems/add-two-numbers/)                                                               | 🟡 Medium  |  34 / 8  | In-Place Reversal & Manipulation                            |
| 23  | [Course Schedule II](https://leetcode.com/problems/course-schedule-ii/)                                                         | 🟡 Medium  | 36 / 14  | Topological Sort                                            |
| 24  | [Course Schedule](https://leetcode.com/problems/course-schedule/)                                                               | 🟡 Medium  | 44 / 12  | Topological Sort                                            |
| 25  | [Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/)                                                         |  🟢 Easy   |  36 / 8  | K-Way Merge                                                 |
| 26  | [Text Justification](https://leetcode.com/problems/text-justification/)                                                         |  🔴 Hard   |  39 / 9  | String Manipulation & Parsing                               |
| 28  | [Spiral Matrix](https://leetcode.com/problems/spiral-matrix/)                                                                   | 🟡 Medium  | 46 / 10  | Matrix Traversal                                            |
| 29  | [Insert Delete GetRandom O(1)](https://leetcode.com/problems/insert-delete-getrandom-o1/)                                       | 🟡 Medium  | 36 / 10  | Hashing / Frequency Counting                                |
| 30  | [Container With Most Water](https://leetcode.com/problems/container-with-most-water/)                                           | 🟡 Medium  | 52 / 12  | Two Pointers                                                |
| 31  | [Kth Largest Element in an Array](https://leetcode.com/problems/kth-largest-element-in-an-array/)                               | 🟡 Medium  | 39 / 10  | Top-K Elements                                              |
| 32  | [String Compression](https://leetcode.com/problems/string-compression/)                                                         | 🟡 Medium  | 40 / 10  | Two Pointers                                                |
| 33  | [Generate Parentheses](https://leetcode.com/problems/generate-parentheses/)                                                     | 🟡 Medium  |  38 / 7  | Subsets, Permutations & Combinations                        |
| 34  | [Valid Palindrome](https://leetcode.com/problems/valid-palindrome/)                                                             |  🟢 Easy   | 39 / 10  | Two Pointers                                                |
| 35  | [Coin Change](https://leetcode.com/problems/coin-change/)                                                                       | 🟡 Medium  |  38 / 8  | Knapsack (0/1, Unbounded, Subset Sum)                       |
| 36  | [Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/)                                                       |  🟢 Easy   |  34 / 9  | In-Place Reversal & Manipulation                            |
| 37  | [Sort Colors](https://leetcode.com/problems/sort-colors/)                                                                       | 🟡 Medium  |  37 / 9  | Two Pointers                                                |
| 38  | [Minimum Window Substring](https://leetcode.com/problems/minimum-window-substring/)                                             |  🔴 Hard   |  34 / 8  | Sliding Window                                              |
| 39  | [House Robber](https://leetcode.com/problems/house-robber/)                                                                     | 🟡 Medium  | 37 / 10  | 1D DP                                                       |
| 40  | [Rotate Image](https://leetcode.com/problems/rotate-image/)                                                                     | 🟡 Medium  |  34 / 8  | Matrix Traversal                                            |
| 41  | [Valid Anagram](https://leetcode.com/problems/valid-anagram/)                                                                   |  🟢 Easy   |  37 / 8  | Hashing / Frequency Counting                                |
| 42  | [Min Stack](https://leetcode.com/problems/min-stack/)                                                                           | 🟡 Medium  |  36 / 9  | Stack (Matching & Expression Evaluation)                    |
| 43  | [Climbing Stairs](https://leetcode.com/problems/climbing-stairs/)                                                               |  🟢 Easy   |  34 / 9  | 1D DP                                                       |
| 44  | [Longest Consecutive Sequence](https://leetcode.com/problems/longest-consecutive-sequence/)                                     | 🟡 Medium  |  38 / 8  | Hashing / Frequency Counting                                |
| 45  | [Word Ladder](https://leetcode.com/problems/word-ladder/)                                                                       |  🔴 Hard   |  36 / 8  | DFS / BFS on Grids & Graphs                                 |
| 46  | [Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/)                                                 |  🟢 Easy   |  34 / 6  | Dummy Head & Merging                                        |
| 47  | [Product of Array Except Self](https://leetcode.com/problems/product-of-array-except-self/)                                     | 🟡 Medium  |  35 / 7  | Hashing / Frequency Counting                                |
| 48  | [Word Search](https://leetcode.com/problems/word-search/)                                                                       | 🟡 Medium  |  34 / 8  | Constraint Search (Grid & Board Problems)                   |
| 49  | [Letter Combinations of a Phone Number](https://leetcode.com/problems/letter-combinations-of-a-phone-number/)                   | 🟡 Medium  |  33 / 8  | Subsets, Permutations & Combinations                        |
| 50  | [Time Based Key-Value Store](https://leetcode.com/problems/time-based-key-value-store/)                                         | 🟡 Medium  | 33 / 12  | Binary Search                                               |
| 51  | [Word Break](https://leetcode.com/problems/word-break/)                                                                         | 🟡 Medium  |  29 / 8  | 1D DP                                                       |
| 52  | [Palindrome Number](https://leetcode.com/problems/palindrome-number/)                                                           |  🟢 Easy   | 31 / 10  | Math & Number Theory                                        |
| 53  | [Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/)                       |  🟢 Easy   |  24 / 8  | Two Pointers                                                |
| 55  | [Basic Calculator II](https://leetcode.com/problems/basic-calculator-ii/)                                                       | 🟡 Medium  |  29 / 7  | Stack (Matching & Expression Evaluation)                    |
| 56  | [Reverse Nodes in k-Group](https://leetcode.com/problems/reverse-nodes-in-k-group/)                                             |  🔴 Hard   |  26 / 8  | In-Place Reversal & Manipulation                            |
| 57  | [Roman to Integer](https://leetcode.com/problems/roman-to-integer/)                                                             |  🟢 Easy   |  35 / 6  | String Manipulation & Parsing                               |
| 58  | [LFU Cache](https://leetcode.com/problems/lfu-cache/)                                                                           |  🔴 Hard   |  30 / 7  | Design Problems                                             |
| 59  | [Valid Sudoku](https://leetcode.com/problems/valid-sudoku/)                                                                     | 🟡 Medium  |  24 / 8  | Hashing / Frequency Counting                                |
| 60  | [Jump Game](https://leetcode.com/problems/jump-game/)                                                                           | 🟡 Medium  |  32 / 7  | Greedy (Jump, Gas Station, Sort + Scan, Exchange Arguments) |
| 61  | [Find Peak Element](https://leetcode.com/problems/find-peak-element/)                                                           | 🟡 Medium  |  30 / 8  | Binary Search                                               |
| 62  | [Integer to Roman](https://leetcode.com/problems/integer-to-roman/)                                                             | 🟡 Medium  |  30 / 8  | String Manipulation & Parsing                               |
| 63  | [Best Time to Buy and Sell Stock II](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/)                         | 🟡 Medium  |  27 / 8  | State Machine DP (Stocks & Friends)                         |
| 64  | [First Unique Character in a String](https://leetcode.com/problems/first-unique-character-in-a-string/)                         |  🟢 Easy   |  24 / 8  | Hashing / Frequency Counting                                |
| 65  | [Random Pick with Weight](https://leetcode.com/problems/random-pick-with-weight/)                                               | 🟡 Medium  |  24 / 6  | Randomization & Reservoir Sampling                          |
| 66  | [Permutations](https://leetcode.com/problems/permutations/)                                                                     | 🟡 Medium  |  24 / 6  | Subsets, Permutations & Combinations                        |
| 67  | [Binary Tree Maximum Path Sum](https://leetcode.com/problems/binary-tree-maximum-path-sum/)                                     |  🔴 Hard   |  24 / 7  | Tree DFS (Traversals & Recursion)                           |
| 68  | [Daily Temperatures](https://leetcode.com/problems/daily-temperatures/)                                                         | 🟡 Medium  |  28 / 9  | Monotonic Stack                                             |
| 69  | [Reverse Integer](https://leetcode.com/problems/reverse-integer/)                                                               | 🟡 Medium  |  18 / 7  | Math & Number Theory                                        |
| 70  | [Majority Element](https://leetcode.com/problems/majority-element/)                                                             |  🟢 Easy   |  22 / 8  | Hashing / Frequency Counting                                |
| 71  | [Subsets](https://leetcode.com/problems/subsets/)                                                                               | 🟡 Medium  |  20 / 7  | Subsets, Permutations & Combinations                        |
| 72  | [Edit Distance](https://leetcode.com/problems/edit-distance/)                                                                   | 🟡 Medium  |  23 / 8  | Subsequence DP (LIS, LCS, Edit Distance)                    |
| 73  | [Asteroid Collision](https://leetcode.com/problems/asteroid-collision/)                                                         | 🟡 Medium  |  34 / 8  | Stack (Matching & Expression Evaluation)                    |
| 74  | [Rotate Array](https://leetcode.com/problems/rotate-array/)                                                                     | 🟡 Medium  |  26 / 9  | Cyclic Sort                                                 |
| 75  | [Happy Number](https://leetcode.com/problems/happy-number/)                                                                     |  🟢 Easy   |  26 / 7  | Fast & Slow Pointers                                        |
| 76  | [Regular Expression Matching](https://leetcode.com/problems/regular-expression-matching/)                                       |  🔴 Hard   |  24 / 6  | Subsequence DP (LIS, LCS, Edit Distance)                    |
| 77  | [Maximum Product Subarray](https://leetcode.com/problems/maximum-product-subarray/)                                             | 🟡 Medium  |  22 / 8  | Kadane's Algorithm                                          |
| 78  | [Jump Game II](https://leetcode.com/problems/jump-game-ii/)                                                                     | 🟡 Medium  |  27 / 6  | Greedy (Jump, Gas Station, Sort + Scan, Exchange Arguments) |
| 79  | [Decode Ways](https://leetcode.com/problems/decode-ways/)                                                                       | 🟡 Medium  |  21 / 6  | 1D DP                                                       |
| 80  | [Longest Valid Parentheses](https://leetcode.com/problems/longest-valid-parentheses/)                                           |  🔴 Hard   |  25 / 7  | Stack (Matching & Expression Evaluation)                    |
| 81  | [String to Integer (atoi)](https://leetcode.com/problems/string-to-integer-atoi/)                                               | 🟡 Medium  |  20 / 6  | String Manipulation & Parsing                               |
| 82  | [Gas Station](https://leetcode.com/problems/gas-station/)                                                                       | 🟡 Medium  |  28 / 8  | Greedy (Jump, Gas Station, Sort + Scan, Exchange Arguments) |
| 83  | [Largest Rectangle in Histogram](https://leetcode.com/problems/largest-rectangle-in-histogram/)                                 |  🔴 Hard   |  29 / 7  | Monotonic Stack                                             |
| 84  | [Basic Calculator](https://leetcode.com/problems/basic-calculator/)                                                             |  🔴 Hard   |  24 / 6  | Stack (Matching & Expression Evaluation)                    |
| 85  | [Lowest Common Ancestor of a Binary Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/)               | 🟡 Medium  |  22 / 5  | Lowest Common Ancestor & Path Queries                       |
| 86  | [Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/)                                                 |  🟢 Easy   |  16 / 6  | Fast & Slow Pointers                                        |
| 87  | [Squares of a Sorted Array](https://leetcode.com/problems/squares-of-a-sorted-array/)                                           |  🟢 Easy   |  16 / 8  | Two Pointers                                                |
| 88  | [4Sum](https://leetcode.com/problems/4sum/)                                                                                     | 🟡 Medium  |  20 / 6  | Two Pointers                                                |
| 89  | [Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/)                                     |  🟢 Easy   |  18 / 8  | Tree DFS (Traversals & Recursion)                           |
| 90  | [First Missing Positive](https://leetcode.com/problems/first-missing-positive/)                                                 |  🔴 Hard   |  31 / 5  | Cyclic Sort                                                 |
| 91  | [Diameter of Binary Tree](https://leetcode.com/problems/diameter-of-binary-tree/)                                               |  🟢 Easy   |  20 / 6  | Tree DFS (Traversals & Recursion)                           |
| 92  | [Largest Number](https://leetcode.com/problems/largest-number/)                                                                 | 🟡 Medium  |  20 / 6  | Greedy (Jump, Gas Station, Sort + Scan, Exchange Arguments) |
| 93  | [Serialize and Deserialize Binary Tree](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/)                   |  🔴 Hard   |  23 / 6  | Tree Construction & Serialization                           |
| 94  | [Pow(x, n)](https://leetcode.com/problems/powx-n/)                                                                              | 🟡 Medium  |  22 / 5  | Math & Number Theory                                        |
| 95  | [Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree/)                                       | 🟡 Medium  |  18 / 6  | Binary Search Tree (BST)                                    |
| 96  | [Reverse Words in a String](https://leetcode.com/problems/reverse-words-in-a-string/)                                           | 🟡 Medium  |  28 / 6  | String Manipulation & Parsing                               |
| 98  | [Rotting Oranges](https://leetcode.com/problems/rotting-oranges/)                                                               | 🟡 Medium  | 50 / 13  | DFS / BFS on Grids & Graphs                                 |
| 99  | [Linked List Cycle](https://leetcode.com/problems/linked-list-cycle/)                                                           |  🟢 Easy   |  24 / 6  | Fast & Slow Pointers                                        |
| 100 | [Copy List with Random Pointer](https://leetcode.com/problems/copy-list-with-random-pointer/)                                   | 🟡 Medium  |  20 / 6  | In-Place Reversal & Manipulation                            |
| 101 | [Minimum Path Sum](https://leetcode.com/problems/minimum-path-sum/)                                                             | 🟡 Medium  |  16 / 7  | Grid DP                                                     |
| 102 | [Unique Paths](https://leetcode.com/problems/unique-paths/)                                                                     | 🟡 Medium  |  19 / 6  | Grid DP                                                     |
| 103 | [Task Scheduler](https://leetcode.com/problems/task-scheduler/)                                                                 | 🟡 Medium  |  23 / 6  | Top-K Elements                                              |

## Pattern Based Index

**Arrays & Strings**

- [Hashing / Frequency Counting](#hashing--frequency-counting) · ⭐ Core · 15
- [Two Pointers](#two-pointers) · ⭐ Core · 20
- [Sliding Window](#sliding-window) · ⭐ Core · 15
- [Prefix Sum & Difference Array](#prefix-sum--difference-array) · 📘 Common · 10
- [Kadane's Algorithm](#kadanes-algorithm) · 📘 Common · 8
- [Merge Intervals & Sweep Line](#merge-intervals--sweep-line) · 📘 Common · 10
- [Cyclic Sort](#cyclic-sort) · 📘 Common · 9
- [Matrix Traversal](#matrix-traversal) · 📘 Common · 10
- [String Manipulation & Parsing](#string-manipulation--parsing) · ⭐ Core · 15
- [Frequently Asked Array, Sorting & Counting Basics](#frequently-asked-array-sorting--counting-basics) · 📘 Common · 21

**Searching**

- [Binary Search](#binary-search) · ⭐ Core · 15
- [Binary Search on Answer](#binary-search-on-answer) · 📘 Common · 8
- [Search in Rotated / Modified Arrays](#search-in-rotated--modified-arrays) · 📘 Common · 8

**Linked List**

- [Fast & Slow Pointers](#fast--slow-pointers) · ⭐ Core · 9
- [In-Place Reversal & Manipulation](#in-place-reversal--manipulation) · ⭐ Core · 15
- [Dummy Head & Merging](#dummy-head--merging) · 📘 Common · 8

**Stack & Queue**

- [Stack (Matching & Expression Evaluation)](#stack-matching--expression-evaluation) · ⭐ Core · 15
- [Monotonic Stack](#monotonic-stack) · 📘 Common · 10
- [Queue & Monotonic Deque](#queue--monotonic-deque) · 📘 Common · 8

**Trees**

- [Tree DFS (Traversals & Recursion)](#tree-dfs-traversals--recursion) · ⭐ Core · 15
- [Tree BFS (Level Order)](#tree-bfs-level-order) · 📘 Common · 10
- [Binary Search Tree (BST)](#binary-search-tree-bst) · 📘 Common · 10
- [Lowest Common Ancestor & Path Queries](#lowest-common-ancestor--path-queries) · 📘 Common · 8
- [Tree Construction & Serialization](#tree-construction--serialization) · 📘 Common · 9
- [Tree DP](#tree-dp) · 🧗 Occasional · 7

**Graphs**

- [DFS / BFS on Grids & Graphs](#dfs--bfs-on-grids--graphs) · ⭐ Core · 15
- [Connected Components & Graph DFS](#connected-components--graph-dfs) · 🧗 Occasional · 8
- [Topological Sort](#topological-sort) · 📘 Common · 9
- [Union-Find (DSU)](#union-find-dsu) · 🧗 Occasional · 7
- [Shortest Path (BFS, Dijkstra, Bellman-Ford, Floyd-Warshall)](#shortest-path-bfs-dijkstra-bellman-ford-floyd-warshall) · 🧗 Occasional · 9
- [Cycle Detection & Bipartite Check](#cycle-detection--bipartite-check) · 📘 Common · 9
- [MST, Bridges & Advanced Graph](#mst-bridges--advanced-graph) · 🧗 Occasional · 2

**Heap / Priority Queue**

- [Top-K Elements](#top-k-elements) · ⭐ Core · 14
- [K-Way Merge](#k-way-merge) · 📘 Common · 7
- [Two Heaps & Scheduling with Heaps](#two-heaps--scheduling-with-heaps) · 🧗 Occasional · 10

**Recursion & Backtracking**

- [Subsets, Permutations & Combinations](#subsets-permutations--combinations) · ⭐ Core · 10
- [Constraint Search (Grid & Board Problems)](#constraint-search-grid--board-problems) · 📘 Common · 8
- [Partition Problems](#partition-problems) · 📘 Common · 7

**Dynamic Programming**

- [1D DP](#1d-dp) · ⭐ Core · 15
- [Knapsack (0/1, Unbounded, Subset Sum)](#knapsack-01-unbounded-subset-sum) · 📘 Common · 8
- [Subsequence DP (LIS, LCS, Edit Distance)](#subsequence-dp-lis-lcs-edit-distance) · ⭐ Core · 10
- [Grid DP](#grid-dp) · 📘 Common · 9
- [Interval DP](#interval-dp) · 🧗 Occasional · 9
- [State Machine DP (Stocks & Friends)](#state-machine-dp-stocks--friends) · 📘 Common · 8
- [Bitmask DP](#bitmask-dp) · 🧗 Occasional · 3
- [Digit DP](#digit-dp) · 🧗 Occasional · 2

**Greedy**

- [Greedy (Jump, Gas Station, Sort + Scan, Exchange Arguments)](#greedy-jump-gas-station-sort--scan-exchange-arguments) · ⭐ Core · 15

**Other Patterns**

- [Trie (Prefix Tree)](#trie-prefix-tree) · 🧗 Occasional · 10
- [Bit Manipulation](#bit-manipulation) · 📘 Common · 10
- [Math & Number Theory](#math--number-theory) · ⭐ Core · 15
- [Design Problems](#design-problems) · ⭐ Core · 15
- [Randomization & Reservoir Sampling](#randomization--reservoir-sampling) · 📘 Common · 5
- [Segment Tree & Fenwick Tree](#segment-tree--fenwick-tree) · 🧗 Occasional · 5
- [String Algorithms (KMP, Rabin-Karp, Z-Function)](#string-algorithms-kmp-rabin-karp-z-function) · 📘 Common · 10
- [Geometry](#geometry) · 🧗 Occasional · 7

---

## Arrays & Strings

### Hashing / Frequency Counting

⭐ Core · 15 problems

_Reach for it when you need O(1) lookups, counts, or "have I seen this before?"_

| Done |  #  | Problem                                                                                                 | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ✅  |  1  | [Two Sum](https://leetcode.com/problems/two-sum/)                                                       |  🟢 Easy   | 120 / 25 | 🔥🔥🔥 #2  |
|  ✅  |  2  | [Valid Anagram](https://leetcode.com/problems/valid-anagram/)                                           |  🟢 Easy   |  37 / 8  | 🔥🔥🔥 #41 |
|  ✅  |  3  | [First Unique Character in a String](https://leetcode.com/problems/first-unique-character-in-a-string/) |  🟢 Easy   |  24 / 8  | 🔥🔥🔥 #64 |
|  ✅  |  4  | [Majority Element](https://leetcode.com/problems/majority-element/)                                     |  🟢 Easy   |  22 / 8  | 🔥🔥🔥 #70 |
|  ✅  |  5  | [Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)                                 |  🟢 Easy   |  20 / 5  | 🔥🔥 #107  |
|  ✅  |  6  | [Isomorphic Strings](https://leetcode.com/problems/isomorphic-strings/)                                 |  🟢 Easy   |  15 / 6  | 🔥🔥 #175  |
|  ✅  |  7  | [Ransom Note](https://leetcode.com/problems/ransom-note/)                                               |  🟢 Easy   |  14 / 6  | 🔥🔥 #196  |
|  ✅  |  8  | [Group Anagrams](https://leetcode.com/problems/group-anagrams/)                                         | 🟡 Medium  | 80 / 22  | 🔥🔥🔥 #8  |
|  ✅  |  9  | [Subarray Sum Equals K](https://leetcode.com/problems/subarray-sum-equals-k/)                           | 🟡 Medium  | 44 / 14  | 🔥🔥🔥 #15 |
|  ✅  | 10  | [Top K Frequent Elements](https://leetcode.com/problems/top-k-frequent-elements/)                       | 🟡 Medium  | 51 / 13  | 🔥🔥🔥 #21 |
|  ✅  | 11  | [Insert Delete GetRandom O(1)](https://leetcode.com/problems/insert-delete-getrandom-o1/)               | 🟡 Medium  | 36 / 10  | 🔥🔥🔥 #29 |
|  ✅  | 12  | [Longest Consecutive Sequence](https://leetcode.com/problems/longest-consecutive-sequence/)             | 🟡 Medium  |  38 / 8  | 🔥🔥🔥 #44 |
|  ✅  | 13  | [Product of Array Except Self](https://leetcode.com/problems/product-of-array-except-self/)             | 🟡 Medium  |  35 / 7  | 🔥🔥🔥 #47 |
|  ✅  | 14  | [Valid Sudoku](https://leetcode.com/problems/valid-sudoku/)                                             | 🟡 Medium  |  24 / 8  | 🔥🔥🔥 #59 |
|  ↪️  | 15  | [Majority Element II](https://leetcode.com/problems/majority-element-ii/)                               | 🟡 Medium  |  10 / 4  | 🔥🔥 #229  |

### Two Pointers

⭐ Core · 20 problems

_Sorted input, pairs/triplets, in-place rewrites, palindromes_

| Done |  #  | Problem                                                                                                   | Difficulty | Asked at |    Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ↪️  |  1  | [Move Zeroes](https://leetcode.com/problems/move-zeroes/)                                                 |  🟢 Easy   | 42 / 12  | 🔥🔥🔥 #18 |
|  ✅  |  2  | [Valid Palindrome](https://leetcode.com/problems/valid-palindrome/)                                       |  🟢 Easy   | 39 / 10  | 🔥🔥🔥 #34 |
|  ↪️  |  3  | [Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/) |  🟢 Easy   |  24 / 8  | 🔥🔥🔥 #53 |
|  ↪️  |  4  | [Squares of a Sorted Array](https://leetcode.com/problems/squares-of-a-sorted-array/)                     |  🟢 Easy   |  16 / 8  | 🔥🔥🔥 #87 |
|  ✅  |  5  | [Reverse String](https://leetcode.com/problems/reverse-string/)                                           |  🟢 Easy   |  16 / 6  | 🔥🔥 #104  |
|  ✅  |  6  | [Intersection of Two Arrays](https://leetcode.com/problems/intersection-of-two-arrays/)                   |  🟢 Easy   |  15 / 6  | 🔥🔥 #106  |
|  ↪️  |  7  | [Valid Palindrome II](https://leetcode.com/problems/valid-palindrome-ii/)                                 |  🟢 Easy   |  16 / 8  | 🔥🔥 #149  |
|  ✅  |  8  | [Remove Element](https://leetcode.com/problems/remove-element/)                                           |  🟢 Easy   |  10 / 6  | 🔥🔥 #205  |
|  ✅  |  9  | [Is Subsequence](https://leetcode.com/problems/is-subsequence/)                                           |  🟢 Easy   |  14 / 4  | 🔥🔥 #214  |
|  ↪️  | 10  | [Backspace String Compare](https://leetcode.com/problems/backspace-string-compare/)                       |  🟢 Easy   |  12 / 2  | 🔥🔥 #310  |
|  ⬜  | 11  | [3Sum](https://leetcode.com/problems/3sum/)                                                               | 🟡 Medium  | 52 / 12  | 🔥🔥🔥 #10 |
|  ✅  | 12  | [Container With Most Water](https://leetcode.com/problems/container-with-most-water/)                     | 🟡 Medium  | 52 / 12  | 🔥🔥🔥 #30 |
|  ⬜  | 13  | [String Compression](https://leetcode.com/problems/string-compression/)                                   | 🟡 Medium  | 40 / 10  | 🔥🔥🔥 #32 |
|  ⬜  | 14  | [Sort Colors](https://leetcode.com/problems/sort-colors/)                                                 | 🟡 Medium  |  37 / 9  | 🔥🔥🔥 #37 |
|  ⬜  | 15  | [4Sum](https://leetcode.com/problems/4sum/)                                                               | 🟡 Medium  |  20 / 6  | 🔥🔥🔥 #88 |
|  ⬜  | 16  | [Two Sum II - Input Array Is Sorted](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)     | 🟡 Medium  |  14 / 6  | 🔥🔥 #140  |
|  ⬜  | 17  | [3Sum Closest](https://leetcode.com/problems/3sum-closest/)                                               | 🟡 Medium  |  12 / 6  | 🔥🔥 #160  |
|  ⬜  | 18  | [Boats to Save People](https://leetcode.com/problems/boats-to-save-people/)                               | 🟡 Medium  |  12 / 5  | 🔥🔥 #224  |
|  ⬜  | 19  | [K-diff Pairs in an Array](https://leetcode.com/problems/k-diff-pairs-in-an-array/)                       | 🟡 Medium  |  7 / 2   | 🔥🔥 #316  |
|  ⬜  | 20  | [Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/)                                 |  🔴 Hard   | 71 / 14  | 🔥🔥🔥 #9  |

### Sliding Window

⭐ Core · 15 problems

_Contiguous subarray/substring with a constraint (longest, shortest, count)_

| Done |  #  | Problem                                                                                                                         | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Best Time to Buy and Sell Stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/)                               |  🟢 Easy   | 86 / 20  | 🔥🔥🔥 #6  |
|  ⬜  |  2  | [Contains Duplicate II](https://leetcode.com/problems/contains-duplicate-ii/)                                                   |  🟢 Easy   |  14 / 6  | 🔥🔥 #154  |
|  ⬜  |  3  | [Maximum Average Subarray I](https://leetcode.com/problems/maximum-average-subarray-i/)                                         |  🟢 Easy   |  11 / 6  | 🔥🔥 #323  |
|  ⬜  |  4  | [Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/) | 🟡 Medium  | 100 / 28 | 🔥🔥🔥 #7  |
|  ⬜  |  5  | [Permutation in String](https://leetcode.com/problems/permutation-in-string/)                                                   | 🟡 Medium  |  16 / 8  | 🔥🔥 #111  |
|  ⬜  |  6  | [Minimum Size Subarray Sum](https://leetcode.com/problems/minimum-size-subarray-sum/)                                           | 🟡 Medium  |  16 / 6  | 🔥🔥 #124  |
|  ⬜  |  7  | [Find All Anagrams in a String](https://leetcode.com/problems/find-all-anagrams-in-a-string/)                                   | 🟡 Medium  |  14 / 7  | 🔥🔥 #125  |
|  ⬜  |  8  | [Max Consecutive Ones III](https://leetcode.com/problems/max-consecutive-ones-iii/)                                             | 🟡 Medium  |  20 / 9  | 🔥🔥 #133  |
|  ⬜  |  9  | [Subarray Product Less Than K](https://leetcode.com/problems/subarray-product-less-than-k/)                                     | 🟡 Medium  |  20 / 8  | 🔥🔥 #147  |
|  ⬜  | 10  | [Longest Repeating Character Replacement](https://leetcode.com/problems/longest-repeating-character-replacement/)               | 🟡 Medium  |  20 / 8  | 🔥🔥 #167  |
|  ⬜  | 11  | [Maximum Points You Can Obtain from Cards](https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards/)             | 🟡 Medium  |  12 / 5  | 🔥🔥 #253  |
|  ⬜  | 12  | [Fruit Into Baskets](https://leetcode.com/problems/fruit-into-baskets/)                                                         | 🟡 Medium  |  12 / 7  | 🔥🔥 #297  |
|  ⬜  | 13  | [Alternating Groups II](https://leetcode.com/problems/alternating-groups-ii/)                                                   | 🟡 Medium  |  6 / 2   | 🔥🔥 #374  |
|  ⬜  | 14  | [Minimum Window Substring](https://leetcode.com/problems/minimum-window-substring/)                                             |  🔴 Hard   |  34 / 8  | 🔥🔥🔥 #38 |
|  ⬜  | 15  | [Substring with Concatenation of All Words](https://leetcode.com/problems/substring-with-concatenation-of-all-words/)           |  🔴 Hard   |  10 / 4  | 🔥🔥 #233  |

### Prefix Sum & Difference Array

📘 Common · 10 problems

_Repeated range-sum queries, or "subarray sums to X" with a running total_

| Done |  #  | Problem                                                                                       | Difficulty | Asked at |   Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Find Pivot Index](https://leetcode.com/problems/find-pivot-index/)                           |  🟢 Easy   |  16 / 4  | 🔥🔥 #166 |
|  ⬜  |  2  | [Range Sum Query - Immutable](https://leetcode.com/problems/range-sum-query-immutable/)       |  🟢 Easy   |  8 / 4   | 🔥🔥 #365 |
|  ⬜  |  3  | [Running Sum of 1d Array](https://leetcode.com/problems/running-sum-of-1d-array/)             |  🟢 Easy   |  6 / 4   |  🔥 #424  |
|  ⬜  |  4  | [Find the Highest Altitude](https://leetcode.com/problems/find-the-highest-altitude/)         |  🟢 Easy   |  8 / 4   |  🔥 #676  |
|  ⬜  |  5  | [Contiguous Array](https://leetcode.com/problems/contiguous-array/)                           | 🟡 Medium  |  15 / 7  | 🔥🔥 #150 |
|  ⬜  |  6  | [Subarray Sums Divisible by K](https://leetcode.com/problems/subarray-sums-divisible-by-k/)   | 🟡 Medium  |  10 / 4  | 🔥🔥 #245 |
|  ⬜  |  7  | [Continuous Subarray Sum](https://leetcode.com/problems/continuous-subarray-sum/)             | 🟡 Medium  |  10 / 4  | 🔥🔥 #250 |
|  ⬜  |  8  | [Car Pooling](https://leetcode.com/problems/car-pooling/)                                     | 🟡 Medium  |  12 / 3  | 🔥🔥 #276 |
|  ⬜  |  9  | [Range Sum Query 2D - Immutable](https://leetcode.com/problems/range-sum-query-2d-immutable/) | 🟡 Medium  |  10 / 2  | 🔥🔥 #337 |
|  ⬜  | 10  | [Zero Array Transformation II](https://leetcode.com/problems/zero-array-transformation-ii/)   | 🟡 Medium  |  5 / 2   |  🔥 #619  |

### Kadane's Algorithm

📘 Common · 8 problems

_Best contiguous subarray where you decide: extend the current run or restart_

| Done |  #  | Problem                                                                                                               | Difficulty | Asked at |    Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Max Consecutive Ones](https://leetcode.com/problems/max-consecutive-ones/)                                           |  🟢 Easy   |  14 / 5  | 🔥🔥 #238  |
|  ⬜  |  2  | [Longest Continuous Increasing Subsequence](https://leetcode.com/problems/longest-continuous-increasing-subsequence/) |  🟢 Easy   |  5 / 1   |  🔥 #755   |
|  ⬜  |  3  | [Maximum Subarray](https://leetcode.com/problems/maximum-subarray/)                                                   | 🟡 Medium  | 60 / 11  | 🔥🔥🔥 #11 |
|  ⬜  |  4  | [Maximum Product Subarray](https://leetcode.com/problems/maximum-product-subarray/)                                   | 🟡 Medium  |  22 / 8  | 🔥🔥🔥 #77 |
|  ⬜  |  5  | [Maximum Sum Circular Subarray](https://leetcode.com/problems/maximum-sum-circular-subarray/)                         | 🟡 Medium  |  12 / 4  | 🔥🔥 #294  |
|  ⬜  |  6  | [Arithmetic Slices](https://leetcode.com/problems/arithmetic-slices/)                                                 | 🟡 Medium  |  6 / 2   |  🔥 #474   |
|  ⬜  |  7  | [Maximum Absolute Sum of Any Subarray](https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/)           | 🟡 Medium  |  6 / 2   |  🔥 #912   |
|  ⬜  |  8  | [Maximum Subarray Sum with One Deletion](https://leetcode.com/problems/maximum-subarray-sum-with-one-deletion/)       | 🟡 Medium  |  4 / 0   |   #1223    |

### Merge Intervals & Sweep Line

📘 Common · 10 problems

_Overlaps, merging ranges, scheduling, "max concurrent" questions_

| Done |  #  | Problem                                                                                                                 | Difficulty | Asked at |   Rank    |
| :--: | :-: | ----------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Summary Ranges](https://leetcode.com/problems/summary-ranges/)                                                         |  🟢 Easy   |  9 / 4   | 🔥🔥 #256 |
|  ⬜  |  2  | [Maximum Number of Balloons](https://leetcode.com/problems/maximum-number-of-balloons/)                                 |  🟢 Easy   |  6 / 3   |  🔥 #518  |
|  ⬜  |  3  | [Merge Intervals](https://leetcode.com/problems/merge-intervals/)                                                       | 🟡 Medium  | 106 / 29 | 🔥🔥🔥 #3 |
|  ⬜  |  4  | [Non-overlapping Intervals](https://leetcode.com/problems/non-overlapping-intervals/)                                   | 🟡 Medium  |  18 / 5  | 🔥🔥 #184 |
|  ⬜  |  5  | [Insert Interval](https://leetcode.com/problems/insert-interval/)                                                       | 🟡 Medium  |  16 / 3  | 🔥🔥 #203 |
|  ⬜  |  6  | [Minimum Number of Arrows to Burst Balloons](https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/) | 🟡 Medium  |  8 / 2   | 🔥🔥 #279 |
|  ⬜  |  7  | [Partition Labels](https://leetcode.com/problems/partition-labels/)                                                     | 🟡 Medium  |  8 / 3   | 🔥🔥 #330 |
|  ⬜  |  8  | [Interval List Intersections](https://leetcode.com/problems/interval-list-intersections/)                               | 🟡 Medium  |  12 / 5  |  🔥 #439  |
|  ⬜  |  9  | [My Calendar I](https://leetcode.com/problems/my-calendar-i/)                                                           | 🟡 Medium  |  10 / 1  |  🔥 #448  |
|  ⬜  | 10  | [The Skyline Problem](https://leetcode.com/problems/the-skyline-problem/)                                               |  🔴 Hard   |  12 / 0  | 🔥🔥 #395 |

### Cyclic Sort

📘 Common · 9 problems

_Array of 1..n (or 0..n) with missing/duplicate values: put each value at its index_

| Done |  #  | Problem                                                                                                             | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Missing Number](https://leetcode.com/problems/missing-number/)                                                     |  🟢 Easy   |  18 / 4  | 🔥🔥 #145  |
|  ⬜  |  2  | [Find All Numbers Disappeared in an Array](https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/) |  🟢 Easy   |  6 / 4   | 🔥🔥 #318  |
|  ⬜  |  3  | [Kth Missing Positive Number](https://leetcode.com/problems/kth-missing-positive-number/)                           |  🟢 Easy   |  9 / 4   | 🔥🔥 #331  |
|  ⬜  |  4  | [Set Mismatch](https://leetcode.com/problems/set-mismatch/)                                                         |  🟢 Easy   |  7 / 6   |  🔥 #436   |
|  ⬜  |  5  | [Rotate Array](https://leetcode.com/problems/rotate-array/)                                                         | 🟡 Medium  |  26 / 9  | 🔥🔥🔥 #74 |
|  ⬜  |  6  | [Find the Duplicate Number](https://leetcode.com/problems/find-the-duplicate-number/)                               | 🟡 Medium  |  20 / 6  | 🔥🔥 #128  |
|  ⬜  |  7  | [Find All Duplicates in an Array](https://leetcode.com/problems/find-all-duplicates-in-an-array/)                   | 🟡 Medium  |  9 / 2   | 🔥🔥 #344  |
|  ⬜  |  8  | [First Missing Positive](https://leetcode.com/problems/first-missing-positive/)                                     |  🔴 Hard   |  31 / 5  | 🔥🔥🔥 #90 |
|  ⬜  |  9  | [Couples Holding Hands](https://leetcode.com/problems/couples-holding-hands/)                                       |  🔴 Hard   |  6 / 1   |  🔥 #646   |

### Matrix Traversal

📘 Common · 10 problems

_Spiral, rotate, diagonal, and in-place marking tricks_

| Done |  #  | Problem                                                                 | Difficulty | Asked at |    Rank    |
| :--: | :-: | ----------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Transpose Matrix](https://leetcode.com/problems/transpose-matrix/)     |  🟢 Easy   |  6 / 2   |  🔥 #433   |
|  ⬜  |  2  | [Toeplitz Matrix](https://leetcode.com/problems/toeplitz-matrix/)       |  🟢 Easy   |  4 / 2   |  🔥 #511   |
|  ⬜  |  3  | [Flipping an Image](https://leetcode.com/problems/flipping-an-image/)   |  🟢 Easy   |  6 / 2   |  🔥 #614   |
|  ⬜  |  4  | [Reshape the Matrix](https://leetcode.com/problems/reshape-the-matrix/) |  🟢 Easy   |  5 / 2   |  🔥 #655   |
|  ⬜  |  5  | [Spiral Matrix](https://leetcode.com/problems/spiral-matrix/)           | 🟡 Medium  | 46 / 10  | 🔥🔥🔥 #28 |
|  ⬜  |  6  | [Rotate Image](https://leetcode.com/problems/rotate-image/)             | 🟡 Medium  |  34 / 8  | 🔥🔥🔥 #40 |
|  ⬜  |  7  | [Set Matrix Zeroes](https://leetcode.com/problems/set-matrix-zeroes/)   | 🟡 Medium  |  20 / 6  | 🔥🔥 #132  |
|  ⬜  |  8  | [Game of Life](https://leetcode.com/problems/game-of-life/)             | 🟡 Medium  |  14 / 2  | 🔥🔥 #244  |
|  ⬜  |  9  | [Diagonal Traverse](https://leetcode.com/problems/diagonal-traverse/)   | 🟡 Medium  |  11 / 2  | 🔥🔥 #286  |
|  ⬜  | 10  | [Spiral Matrix II](https://leetcode.com/problems/spiral-matrix-ii/)     | 🟡 Medium  |  10 / 2  | 🔥🔥 #366  |

### String Manipulation & Parsing

⭐ Core · 15 problems

_Careful index handling, simulation, number/string conversion_

| Done |  #  | Problem                                                                                       | Difficulty | Asked at |    Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/)                 |  🟢 Easy   | 56 / 12  | 🔥🔥🔥 #17 |
|  ⬜  |  2  | [Roman to Integer](https://leetcode.com/problems/roman-to-integer/)                           |  🟢 Easy   |  35 / 6  | 🔥🔥🔥 #57 |
|  ⬜  |  3  | [Add Binary](https://leetcode.com/problems/add-binary/)                                       |  🟢 Easy   |  16 / 6  | 🔥🔥 #159  |
|  ⬜  |  4  | [Add Strings](https://leetcode.com/problems/add-strings/)                                     |  🟢 Easy   |  14 / 2  | 🔥🔥 #242  |
|  ⬜  |  5  | [Reverse Vowels of a String](https://leetcode.com/problems/reverse-vowels-of-a-string/)       |  🟢 Easy   |  13 / 4  | 🔥🔥 #299  |
|  ⬜  |  6  | [Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/) | 🟡 Medium  | 73 / 13  | 🔥🔥🔥 #19 |
|  ⬜  |  7  | [Integer to Roman](https://leetcode.com/problems/integer-to-roman/)                           | 🟡 Medium  |  30 / 8  | 🔥🔥🔥 #62 |
|  ⬜  |  8  | [String to Integer (atoi)](https://leetcode.com/problems/string-to-integer-atoi/)             | 🟡 Medium  |  20 / 6  | 🔥🔥🔥 #81 |
|  ⬜  |  9  | [Reverse Words in a String](https://leetcode.com/problems/reverse-words-in-a-string/)         | 🟡 Medium  |  28 / 6  | 🔥🔥🔥 #96 |
|  ⬜  | 10  | [Multiply Strings](https://leetcode.com/problems/multiply-strings/)                           | 🟡 Medium  |  18 / 5  | 🔥🔥 #134  |
|  ⬜  | 11  | [Zigzag Conversion](https://leetcode.com/problems/zigzag-conversion/)                         | 🟡 Medium  |  18 / 8  | 🔥🔥 #135  |
|  ⬜  | 12  | [Simplify Path](https://leetcode.com/problems/simplify-path/)                                 | 🟡 Medium  |  22 / 6  | 🔥🔥 #153  |
|  ⬜  | 13  | [Count and Say](https://leetcode.com/problems/count-and-say/)                                 | 🟡 Medium  |  12 / 4  | 🔥🔥 #197  |
|  ⬜  | 14  | [Validate IP Address](https://leetcode.com/problems/validate-ip-address/)                     | 🟡 Medium  |  12 / 2  | 🔥🔥 #257  |
|  ⬜  | 15  | [Text Justification](https://leetcode.com/problems/text-justification/)                       |  🔴 Hard   |  39 / 9  | 🔥🔥🔥 #26 |

### Frequently Asked Array, Sorting & Counting Basics

📘 Common · 21 problems

_Very commonly asked easy/medium problems that don't need a named technique: sorting tricks, counting, simulation, and careful array scans_

| Done |  #  | Problem                                                                                                                                     | Difficulty | Asked at |   Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Minimum Absolute Difference](https://leetcode.com/problems/minimum-absolute-difference/)                                                   |  🟢 Easy   |  16 / 6  | 🔥🔥 #259 |
|  ⬜  |  2  | [Third Maximum Number](https://leetcode.com/problems/third-maximum-number/)                                                                 |  🟢 Easy   |  9 / 3   | 🔥🔥 #335 |
|  ⬜  |  3  | [Degree of an Array](https://leetcode.com/problems/degree-of-an-array/)                                                                     |  🟢 Easy   |  12 / 0  | 🔥🔥 #340 |
|  ⬜  |  4  | [Rank Transform of an Array](https://leetcode.com/problems/rank-transform-of-an-array/)                                                     |  🟢 Easy   |  7 / 4   | 🔥🔥 #375 |
|  ⬜  |  5  | [How Many Numbers Are Smaller Than the Current Number](https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/) |  🟢 Easy   |  6 / 4   | 🔥🔥 #386 |
|  ⬜  |  6  | [Shuffle the Array](https://leetcode.com/problems/shuffle-the-array/)                                                                       |  🟢 Easy   |  6 / 3   | 🔥🔥 #397 |
|  ⬜  |  7  | [Word Pattern](https://leetcode.com/problems/word-pattern/)                                                                                 |  🟢 Easy   |  8 / 2   |  🔥 #401  |
|  ⬜  |  8  | [Jewels and Stones](https://leetcode.com/problems/jewels-and-stones/)                                                                       |  🟢 Easy   |  6 / 2   |  🔥 #419  |
|  ⬜  |  9  | [Verifying an Alien Dictionary](https://leetcode.com/problems/verifying-an-alien-dictionary/)                                               |  🟢 Easy   |  8 / 2   |  🔥 #458  |
|  ⬜  | 10  | [Find Numbers with Even Number of Digits](https://leetcode.com/problems/find-numbers-with-even-number-of-digits/)                           |  🟢 Easy   |  7 / 4   |  🔥 #464  |
|  ⬜  | 11  | [Kids With the Greatest Number of Candies](https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/)                         |  🟢 Easy   |  6 / 4   |  🔥 #492  |
|  ⬜  | 12  | [Maximize Distance to Closest Person](https://leetcode.com/problems/maximize-distance-to-closest-person/)                                   | 🟡 Medium  |  9 / 3   | 🔥🔥 #258 |
|  ⬜  | 13  | [Sort an Array](https://leetcode.com/problems/sort-an-array/)                                                                               | 🟡 Medium  |  11 / 4  | 🔥🔥 #284 |
|  ⬜  | 14  | [Rank Teams by Votes](https://leetcode.com/problems/rank-teams-by-votes/)                                                                   | 🟡 Medium  |  8 / 2   | 🔥🔥 #320 |
|  ⬜  | 15  | [Bulls and Cows](https://leetcode.com/problems/bulls-and-cows/)                                                                             | 🟡 Medium  |  7 / 4   | 🔥🔥 #342 |
|  ⬜  | 16  | [H-Index](https://leetcode.com/problems/h-index/)                                                                                           | 🟡 Medium  |  8 / 2   | 🔥🔥 #352 |
|  ⬜  | 17  | [Minimum Moves to Equal Array Elements](https://leetcode.com/problems/minimum-moves-to-equal-array-elements/)                               | 🟡 Medium  |  8 / 1   | 🔥🔥 #393 |
|  ⬜  | 18  | [Find Duplicate File in System](https://leetcode.com/problems/find-duplicate-file-in-system/)                                               | 🟡 Medium  |  7 / 0   |  🔥 #441  |
|  ⬜  | 19  | [Minimum Number of Steps to Make Two Strings Anagram](https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/)   | 🟡 Medium  |  10 / 0  |  🔥 #491  |
|  ⬜  | 20  | [Pairs of Songs With Total Durations Divisible by 60](https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/)   | 🟡 Medium  |  8 / 0   |  🔥 #495  |
|  ⬜  | 21  | [Valid Number](https://leetcode.com/problems/valid-number/)                                                                                 |  🔴 Hard   |  8 / 2   | 🔥🔥 #307 |

---

## Searching

### Binary Search

⭐ Core · 15 problems

_Sorted data or any monotonic condition: halve the search space each step_

| Done |  #  | Problem                                                                                                                                           | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Sqrt(x)](https://leetcode.com/problems/sqrtx/)                                                                                                   |  🟢 Easy   |  16 / 6  | 🔥🔥 #137  |
|  ⬜  |  2  | [Search Insert Position](https://leetcode.com/problems/search-insert-position/)                                                                   |  🟢 Easy   |  12 / 8  | 🔥🔥 #146  |
|  ⬜  |  3  | [Binary Search](https://leetcode.com/problems/binary-search/)                                                                                     |  🟢 Easy   |  11 / 5  | 🔥🔥 #222  |
|  ⬜  |  4  | [First Bad Version](https://leetcode.com/problems/first-bad-version/)                                                                             |  🟢 Easy   |  7 / 4   | 🔥🔥 #266  |
|  ⬜  |  5  | [Valid Perfect Square](https://leetcode.com/problems/valid-perfect-square/)                                                                       |  🟢 Easy   |  8 / 4   | 🔥🔥 #325  |
|  ⬜  |  6  | [Find Smallest Letter Greater Than Target](https://leetcode.com/problems/find-smallest-letter-greater-than-target/)                               |  🟢 Easy   |  6 / 1   |  🔥 #555   |
|  ⬜  |  7  | [Time Based Key-Value Store](https://leetcode.com/problems/time-based-key-value-store/)                                                           | 🟡 Medium  | 33 / 12  | 🔥🔥🔥 #50 |
|  ⬜  |  8  | [Find Peak Element](https://leetcode.com/problems/find-peak-element/)                                                                             | 🟡 Medium  |  30 / 8  | 🔥🔥🔥 #61 |
|  ⬜  |  9  | [Search a 2D Matrix](https://leetcode.com/problems/search-a-2d-matrix/)                                                                           | 🟡 Medium  |  18 / 6  | 🔥🔥 #109  |
|  ⬜  | 10  | [Single Element in a Sorted Array](https://leetcode.com/problems/single-element-in-a-sorted-array/)                                               | 🟡 Medium  |  17 / 6  | 🔥🔥 #127  |
|  ⬜  | 11  | [Search a 2D Matrix II](https://leetcode.com/problems/search-a-2d-matrix-ii/)                                                                     | 🟡 Medium  |  14 / 6  | 🔥🔥 #141  |
|  ⬜  | 12  | [Find K Closest Elements](https://leetcode.com/problems/find-k-closest-elements/)                                                                 | 🟡 Medium  |  18 / 4  | 🔥🔥 #172  |
|  ⬜  | 13  | [Kth Smallest Element in a Sorted Matrix](https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/)                                 | 🟡 Medium  |  10 / 2  | 🔥🔥 #347  |
|  ⬜  | 14  | [Find First and Last Position of Element in Sorted Array](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/) | 🟡 Medium  | 30 / 10  | 🔥🔥 #369  |
|  ⬜  | 15  | [Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/)                                                         |  🔴 Hard   | 42 / 10  | 🔥🔥🔥 #16 |

### Binary Search on Answer

📘 Common · 8 problems

_"Minimize the maximum" / "maximize the minimum": binary search the answer, check feasibility_

| Done |  #  | Problem                                                                                                                   | Difficulty | Asked at |   Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Arranging Coins](https://leetcode.com/problems/arranging-coins/)                                                         |  🟢 Easy   |  6 / 2   |  🔥 #794  |
|  ⬜  |  2  | [Find the Smallest Divisor Given a Threshold](https://leetcode.com/problems/find-the-smallest-divisor-given-a-threshold/) | 🟡 Medium  |  12 / 4  | 🔥🔥 #287 |
|  ⬜  |  3  | [Heaters](https://leetcode.com/problems/heaters/)                                                                         | 🟡 Medium  |  12 / 2  | 🔥🔥 #291 |
|  ⬜  |  4  | [Koko Eating Bananas](https://leetcode.com/problems/koko-eating-bananas/)                                                 | 🟡 Medium  | 44 / 10  | 🔥🔥 #387 |
|  ⬜  |  5  | [Minimum Number of Days to Make m Bouquets](https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/)     | 🟡 Medium  |  9 / 4   |  🔥 #404  |
|  ⬜  |  6  | [Capacity To Ship Packages Within D Days](https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/)         | 🟡 Medium  |  20 / 6  |  🔥 #428  |
|  ⬜  |  7  | [Split Array Largest Sum](https://leetcode.com/problems/split-array-largest-sum/)                                         |  🔴 Hard   |  21 / 8  | 🔥🔥 #139 |
|  ⬜  |  8  | [Find K-th Smallest Pair Distance](https://leetcode.com/problems/find-k-th-smallest-pair-distance/)                       |  🔴 Hard   |  6 / 0   |  🔥 #661  |

### Search in Rotated / Modified Arrays

📘 Common · 8 problems

_Binary search where the array is rotated, mountain-shaped, or partially sorted_

| Done |  #  | Problem                                                                                                               | Difficulty | Asked at |    Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Check if Array Is Sorted and Rotated](https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/)           |  🟢 Easy   |  10 / 4  |  🔥 #568   |
|  ⬜  |  2  | [Count Negative Numbers in a Sorted Matrix](https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/) |  🟢 Easy   |  6 / 2   |  🔥 #706   |
|  ⬜  |  3  | [Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/)                       | 🟡 Medium  | 49 / 11  | 🔥🔥🔥 #14 |
|  ⬜  |  4  | [Find Minimum in Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/)           | 🟡 Medium  |  16 / 5  | 🔥🔥 #163  |
|  ⬜  |  5  | [Peak Index in a Mountain Array](https://leetcode.com/problems/peak-index-in-a-mountain-array/)                       | 🟡 Medium  |  10 / 6  | 🔥🔥 #208  |
|  ⬜  |  6  | [Search in Rotated Sorted Array II](https://leetcode.com/problems/search-in-rotated-sorted-array-ii/)                 | 🟡 Medium  |  10 / 3  | 🔥🔥 #270  |
|  ⬜  |  7  | [Find Minimum in Rotated Sorted Array II](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/)     |  🔴 Hard   |  5 / 2   |  🔥 #630   |
|  ⬜  |  8  | [Find in Mountain Array](https://leetcode.com/problems/find-in-mountain-array/)                                       |  🔴 Hard   |  6 / 2   |  🔥 #889   |

---

## Linked List

### Fast & Slow Pointers

⭐ Core · 9 problems

_Cycle detection, finding the middle, k-th from the end_

| Done |  #  | Problem                                                                                             | Difficulty | Asked at |    Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Happy Number](https://leetcode.com/problems/happy-number/)                                         |  🟢 Easy   |  26 / 7  | 🔥🔥🔥 #75 |
|  ⬜  |  2  | [Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/)                     |  🟢 Easy   |  16 / 6  | 🔥🔥🔥 #86 |
|  ⬜  |  3  | [Linked List Cycle](https://leetcode.com/problems/linked-list-cycle/)                               |  🟢 Easy   |  24 / 6  | 🔥🔥🔥 #99 |
|  ⬜  |  4  | [Intersection of Two Linked Lists](https://leetcode.com/problems/intersection-of-two-linked-lists/) |  🟢 Easy   |  12 / 5  | 🔥🔥 #169  |
|  ⬜  |  5  | [Middle of the Linked List](https://leetcode.com/problems/middle-of-the-linked-list/)               |  🟢 Easy   |  12 / 5  | 🔥🔥 #176  |
|  ⬜  |  6  | [Remove Nth Node From End of List](https://leetcode.com/problems/remove-nth-node-from-end-of-list/) | 🟡 Medium  |  17 / 6  | 🔥🔥 #130  |
|  ⬜  |  7  | [Reorder List](https://leetcode.com/problems/reorder-list/)                                         | 🟡 Medium  |  14 / 6  | 🔥🔥 #143  |
|  ⬜  |  8  | [Linked List Cycle II](https://leetcode.com/problems/linked-list-cycle-ii/)                         | 🟡 Medium  |  12 / 4  | 🔥🔥 #239  |
|  ⬜  |  9  | [Circular Array Loop](https://leetcode.com/problems/circular-array-loop/)                           | 🟡 Medium  |  4 / 2   |  🔥 #731   |

### In-Place Reversal & Manipulation

⭐ Core · 15 problems

_Rewire pointers without extra space: reverse, swap, partition, rotate_

| Done |  #  | Problem                                                                                                           | Difficulty | Asked at |    Rank     |
| :--: | :-: | ----------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :---------: |
|  ⬜  |  1  | [Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/)                                         |  🟢 Easy   |  34 / 9  | 🔥🔥🔥 #36  |
|  ⬜  |  2  | [Remove Duplicates from Sorted List](https://leetcode.com/problems/remove-duplicates-from-sorted-list/)           |  🟢 Easy   |  8 / 4   |  🔥🔥 #225  |
|  ⬜  |  3  | [Remove Linked List Elements](https://leetcode.com/problems/remove-linked-list-elements/)                         |  🟢 Easy   |  8 / 2   |  🔥🔥 #358  |
|  ⬜  |  4  | [Add Two Numbers](https://leetcode.com/problems/add-two-numbers/)                                                 | 🟡 Medium  |  34 / 8  | 🔥🔥🔥 #22  |
|  ⬜  |  5  | [Copy List with Random Pointer](https://leetcode.com/problems/copy-list-with-random-pointer/)                     | 🟡 Medium  |  20 / 6  | 🔥🔥🔥 #100 |
|  ⬜  |  6  | [Reverse Linked List II](https://leetcode.com/problems/reverse-linked-list-ii/)                                   | 🟡 Medium  |  16 / 5  |  🔥🔥 #164  |
|  ⬜  |  7  | [Swap Nodes in Pairs](https://leetcode.com/problems/swap-nodes-in-pairs/)                                         | 🟡 Medium  |  14 / 4  |  🔥🔥 #178  |
|  ⬜  |  8  | [Odd Even Linked List](https://leetcode.com/problems/odd-even-linked-list/)                                       | 🟡 Medium  |  12 / 4  |  🔥🔥 #212  |
|  ⬜  |  9  | [Sort List](https://leetcode.com/problems/sort-list/)                                                             | 🟡 Medium  |  9 / 4   |  🔥🔥 #237  |
|  ⬜  | 10  | [Rotate List](https://leetcode.com/problems/rotate-list/)                                                         | 🟡 Medium  |  10 / 6  |  🔥🔥 #252  |
|  ⬜  | 11  | [Remove Duplicates from Sorted List II](https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/)     | 🟡 Medium  |  10 / 4  |  🔥🔥 #295  |
|  ⬜  | 12  | [Flatten a Multilevel Doubly Linked List](https://leetcode.com/problems/flatten-a-multilevel-doubly-linked-list/) | 🟡 Medium  |  6 / 3   |  🔥🔥 #334  |
|  ⬜  | 13  | [Partition List](https://leetcode.com/problems/partition-list/)                                                   | 🟡 Medium  |  5 / 4   |   🔥 #505   |
|  ⬜  | 14  | [Insertion Sort List](https://leetcode.com/problems/insertion-sort-list/)                                         | 🟡 Medium  |  5 / 1   |   🔥 #735   |
|  ⬜  | 15  | [Reverse Nodes in k-Group](https://leetcode.com/problems/reverse-nodes-in-k-group/)                               |  🔴 Hard   |  26 / 8  | 🔥🔥🔥 #56  |

### Dummy Head & Merging

📘 Common · 8 problems

_A dummy node removes head edge-cases when building or merging lists_

| Done |  #  | Problem                                                                                                                               | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/)                                                       |  🟢 Easy   |  34 / 6  | 🔥🔥🔥 #46 |
|  ⬜  |  2  | [Convert Binary Number in a Linked List to Integer](https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/) |  🟢 Easy   |  9 / 1   |  🔥 #483   |
|  ⬜  |  3  | [Add Two Numbers II](https://leetcode.com/problems/add-two-numbers-ii/)                                                               | 🟡 Medium  |  8 / 2   | 🔥🔥 #277  |
|  ⬜  |  4  | [Delete Node in a Linked List](https://leetcode.com/problems/delete-node-in-a-linked-list/)                                           | 🟡 Medium  |  8 / 4   | 🔥🔥 #293  |
|  ⬜  |  5  | [Swapping Nodes in a Linked List](https://leetcode.com/problems/swapping-nodes-in-a-linked-list/)                                     | 🟡 Medium  |  6 / 2   |  🔥 #877   |
|  ⬜  |  6  | [Merge In Between Linked Lists](https://leetcode.com/problems/merge-in-between-linked-lists/)                                         | 🟡 Medium  |  10 / 0  |   #1124    |
|  ⬜  |  7  | [Split Linked List in Parts](https://leetcode.com/problems/split-linked-list-in-parts/)                                               | 🟡 Medium  |  5 / 0   |   #1438    |
|  ⬜  |  8  | [Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/)                                                           |  🔴 Hard   | 48 / 12  | 🔥🔥🔥 #13 |

---

## Stack & Queue

### Stack (Matching & Expression Evaluation)

⭐ Core · 15 problems

_Nested structure, "most recent unmatched", undo, expression parsing_

| Done |  #  | Problem                                                                                                                   | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Valid Parentheses](https://leetcode.com/problems/valid-parentheses/)                                                     |  🟢 Easy   | 102 / 18 | 🔥🔥🔥 #5  |
|  ⬜  |  2  | [Remove All Adjacent Duplicates In String](https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/)       |  🟢 Easy   |  14 / 4  | 🔥🔥 #210  |
|  ⬜  |  3  | [Implement Queue using Stacks](https://leetcode.com/problems/implement-queue-using-stacks/)                               |  🟢 Easy   |  12 / 4  | 🔥🔥 #223  |
|  ⬜  |  4  | [Decode String](https://leetcode.com/problems/decode-string/)                                                             | 🟡 Medium  | 40 / 11  | 🔥🔥🔥 #20 |
|  ⬜  |  5  | [Min Stack](https://leetcode.com/problems/min-stack/)                                                                     | 🟡 Medium  |  36 / 9  | 🔥🔥🔥 #42 |
|  ⬜  |  6  | [Basic Calculator II](https://leetcode.com/problems/basic-calculator-ii/)                                                 | 🟡 Medium  |  29 / 7  | 🔥🔥🔥 #55 |
|  ⬜  |  7  | [Asteroid Collision](https://leetcode.com/problems/asteroid-collision/)                                                   | 🟡 Medium  |  34 / 8  | 🔥🔥🔥 #73 |
|  ⬜  |  8  | [Remove K Digits](https://leetcode.com/problems/remove-k-digits/)                                                         | 🟡 Medium  |  21 / 6  | 🔥🔥 #108  |
|  ⬜  |  9  | [Evaluate Reverse Polish Notation](https://leetcode.com/problems/evaluate-reverse-polish-notation/)                       | 🟡 Medium  |  18 / 6  | 🔥🔥 #122  |
|  ⬜  | 10  | [Remove All Adjacent Duplicates in String II](https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string-ii/) | 🟡 Medium  |  18 / 6  | 🔥🔥 #138  |
|  ⬜  | 11  | [Exclusive Time of Functions](https://leetcode.com/problems/exclusive-time-of-functions/)                                 | 🟡 Medium  |  12 / 4  | 🔥🔥 #192  |
|  ⬜  | 12  | [Remove Duplicate Letters](https://leetcode.com/problems/remove-duplicate-letters/)                                       | 🟡 Medium  |  13 / 4  | 🔥🔥 #211  |
|  ⬜  | 13  | [Minimum Remove to Make Valid Parentheses](https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/)       | 🟡 Medium  |  10 / 2  | 🔥🔥 #262  |
|  ⬜  | 14  | [Longest Valid Parentheses](https://leetcode.com/problems/longest-valid-parentheses/)                                     |  🔴 Hard   |  25 / 7  | 🔥🔥🔥 #80 |
|  ⬜  | 15  | [Basic Calculator](https://leetcode.com/problems/basic-calculator/)                                                       |  🔴 Hard   |  24 / 6  | 🔥🔥🔥 #84 |

### Monotonic Stack

📘 Common · 10 problems

_"Next greater / smaller element", spans, histogram-style problems_

| Done |  #  | Problem                                                                                                                         | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Next Greater Element I](https://leetcode.com/problems/next-greater-element-i/)                                                 |  🟢 Easy   |  16 / 6  |  🔥 #427   |
|  ⬜  |  2  | [Final Prices With a Special Discount in a Shop](https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/) |  🟢 Easy   |  7 / 4   |  🔥 #642   |
|  ⬜  |  3  | [Daily Temperatures](https://leetcode.com/problems/daily-temperatures/)                                                         | 🟡 Medium  |  28 / 9  | 🔥🔥🔥 #68 |
|  ⬜  |  4  | [Online Stock Span](https://leetcode.com/problems/online-stock-span/)                                                           | 🟡 Medium  |  8 / 4   | 🔥🔥 #329  |
|  ⬜  |  5  | [132 Pattern](https://leetcode.com/problems/132-pattern/)                                                                       | 🟡 Medium  |  8 / 2   | 🔥🔥 #378  |
|  ⬜  |  6  | [Next Greater Element II](https://leetcode.com/problems/next-greater-element-ii/)                                               | 🟡 Medium  |  16 / 6  |  🔥 #430   |
|  ⬜  |  7  | [Shortest Unsorted Continuous Subarray](https://leetcode.com/problems/shortest-unsorted-continuous-subarray/)                   | 🟡 Medium  |  9 / 1   |  🔥 #466   |
|  ⬜  |  8  | [Sum of Subarray Minimums](https://leetcode.com/problems/sum-of-subarray-minimums/)                                             | 🟡 Medium  |  14 / 3  |  🔥 #522   |
|  ⬜  |  9  | [Largest Rectangle in Histogram](https://leetcode.com/problems/largest-rectangle-in-histogram/)                                 |  🔴 Hard   |  29 / 7  | 🔥🔥🔥 #83 |
|  ⬜  | 10  | [Maximal Rectangle](https://leetcode.com/problems/maximal-rectangle/)                                                           |  🔴 Hard   |  15 / 4  | 🔥🔥 #195  |

### Queue & Monotonic Deque

📘 Common · 8 problems

_Simulations with queues; sliding-window max/min with a deque_

| Done |  #  | Problem                                                                                                                                                                                 | Difficulty | Asked at |    Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Number of Recent Calls](https://leetcode.com/problems/number-of-recent-calls/)                                                                                                         |  🟢 Easy   |  8 / 4   | 🔥🔥 #309  |
|  ⬜  |  2  | [Number of Students Unable to Eat Lunch](https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/)                                                                         |  🟢 Easy   |  5 / 3   |  🔥 #888   |
|  ⬜  |  3  | [Time Needed to Buy Tickets](https://leetcode.com/problems/time-needed-to-buy-tickets/)                                                                                                 |  🟢 Easy   |  6 / 1   |  🔥 #944   |
|  ⬜  |  4  | [Design Circular Queue](https://leetcode.com/problems/design-circular-queue/)                                                                                                           | 🟡 Medium  |  20 / 6  | 🔥🔥 #112  |
|  ⬜  |  5  | [Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit](https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit/) | 🟡 Medium  |  14 / 4  | 🔥🔥 #243  |
|  ⬜  |  6  | [Reveal Cards In Increasing Order](https://leetcode.com/problems/reveal-cards-in-increasing-order/)                                                                                     | 🟡 Medium  |  4 / 0   |  🔥 #956   |
|  ⬜  |  7  | [Sliding Window Maximum](https://leetcode.com/problems/sliding-window-maximum/)                                                                                                         |  🔴 Hard   | 48 / 13  | 🔥🔥🔥 #12 |
|  ⬜  |  8  | [Shortest Subarray with Sum at Least K](https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/)                                                                           |  🔴 Hard   |  8 / 2   |  🔥 #597   |

---

## Trees

### Tree DFS (Traversals & Recursion)

⭐ Core · 15 problems

_Most tree problems: recurse, return info upward, combine at the parent_

| Done |  #  | Problem                                                                                                 | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/)             |  🟢 Easy   |  18 / 8  | 🔥🔥🔥 #89 |
|  ⬜  |  2  | [Diameter of Binary Tree](https://leetcode.com/problems/diameter-of-binary-tree/)                       |  🟢 Easy   |  20 / 6  | 🔥🔥🔥 #91 |
|  ⬜  |  3  | [Symmetric Tree](https://leetcode.com/problems/symmetric-tree/)                                         |  🟢 Easy   |  10 / 6  | 🔥🔥 #158  |
|  ⬜  |  4  | [Balanced Binary Tree](https://leetcode.com/problems/balanced-binary-tree/)                             |  🟢 Easy   |  9 / 6   | 🔥🔥 #186  |
|  ⬜  |  5  | [Same Tree](https://leetcode.com/problems/same-tree/)                                                   |  🟢 Easy   |  9 / 5   | 🔥🔥 #202  |
|  ⬜  |  6  | [Binary Tree Inorder Traversal](https://leetcode.com/problems/binary-tree-inorder-traversal/)           |  🟢 Easy   |  8 / 4   | 🔥🔥 #234  |
|  ⬜  |  7  | [Invert Binary Tree](https://leetcode.com/problems/invert-binary-tree/)                                 |  🟢 Easy   |  9 / 4   | 🔥🔥 #268  |
|  ⬜  |  8  | [Binary Tree Paths](https://leetcode.com/problems/binary-tree-paths/)                                   |  🟢 Easy   |  8 / 2   | 🔥🔥 #282  |
|  ⬜  |  9  | [Subtree of Another Tree](https://leetcode.com/problems/subtree-of-another-tree/)                       |  🟢 Easy   |  8 / 4   | 🔥🔥 #315  |
|  ⬜  | 10  | [Path Sum](https://leetcode.com/problems/path-sum/)                                                     |  🟢 Easy   |  8 / 2   | 🔥🔥 #322  |
|  ⬜  | 11  | [Path Sum II](https://leetcode.com/problems/path-sum-ii/)                                               | 🟡 Medium  |  10 / 4  | 🔥🔥 #220  |
|  ⬜  | 12  | [Path Sum III](https://leetcode.com/problems/path-sum-iii/)                                             | 🟡 Medium  |  10 / 4  | 🔥🔥 #227  |
|  ⬜  | 13  | [Flatten Binary Tree to Linked List](https://leetcode.com/problems/flatten-binary-tree-to-linked-list/) | 🟡 Medium  |  8 / 4   | 🔥🔥 #283  |
|  ⬜  | 14  | [Sum Root to Leaf Numbers](https://leetcode.com/problems/sum-root-to-leaf-numbers/)                     | 🟡 Medium  |  8 / 2   | 🔥🔥 #288  |
|  ⬜  | 15  | [Binary Tree Maximum Path Sum](https://leetcode.com/problems/binary-tree-maximum-path-sum/)             |  🔴 Hard   |  24 / 7  | 🔥🔥🔥 #67 |

### Tree BFS (Level Order)

📘 Common · 10 problems

_Anything per-level: right side view, zigzag, min depth, width_

| Done |  #  | Problem                                                                                                                   | Difficulty | Asked at |   Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Minimum Depth of Binary Tree](https://leetcode.com/problems/minimum-depth-of-binary-tree/)                               |  🟢 Easy   |  6 / 4   | 🔥🔥 #357 |
|  ⬜  |  2  | [Sum of Left Leaves](https://leetcode.com/problems/sum-of-left-leaves/)                                                   |  🟢 Easy   |  6 / 1   |  🔥 #574  |
|  ⬜  |  3  | [Binary Tree Right Side View](https://leetcode.com/problems/binary-tree-right-side-view/)                                 | 🟡 Medium  |  14 / 6  | 🔥🔥 #121 |
|  ⬜  |  4  | [Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/)                     | 🟡 Medium  |  16 / 4  | 🔥🔥 #161 |
|  ⬜  |  5  | [Binary Tree Zigzag Level Order Traversal](https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/)       | 🟡 Medium  |  18 / 4  | 🔥🔥 #165 |
|  ⬜  |  6  | [Maximum Width of Binary Tree](https://leetcode.com/problems/maximum-width-of-binary-tree/)                               | 🟡 Medium  |  8 / 3   | 🔥🔥 #303 |
|  ⬜  |  7  | [Employee Importance](https://leetcode.com/problems/employee-importance/)                                                 | 🟡 Medium  |  6 / 1   |  🔥 #558  |
|  ⬜  |  8  | [Populating Next Right Pointers in Each Node](https://leetcode.com/problems/populating-next-right-pointers-in-each-node/) | 🟡 Medium  |  8 / 2   |  🔥 #607  |
|  ⬜  |  9  | [Binary Tree Level Order Traversal II](https://leetcode.com/problems/binary-tree-level-order-traversal-ii/)               | 🟡 Medium  |  6 / 1   |  🔥 #623  |
|  ⬜  | 10  | [Vertical Order Traversal of a Binary Tree](https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/)     |  🔴 Hard   |  14 / 6  | 🔥🔥 #174 |

### Binary Search Tree (BST)

📘 Common · 10 problems

_Use the ordering: inorder = sorted, go left/right by comparison_

| Done |  #  | Problem                                                                                                                 | Difficulty | Asked at |    Rank    |
| :--: | :-: | ----------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Convert Sorted Array to Binary Search Tree](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/) |  🟢 Easy   |  9 / 4   | 🔥🔥 #254  |
|  ⬜  |  2  | [Two Sum IV - Input is a BST](https://leetcode.com/problems/two-sum-iv-input-is-a-bst/)                                 |  🟢 Easy   |  8 / 4   | 🔥🔥 #348  |
|  ⬜  |  3  | [Range Sum of BST](https://leetcode.com/problems/range-sum-of-bst/)                                                     |  🟢 Easy   |  5 / 2   |  🔥 #493   |
|  ⬜  |  4  | [Search in a Binary Search Tree](https://leetcode.com/problems/search-in-a-binary-search-tree/)                         |  🟢 Easy   |  6 / 2   |  🔥 #541   |
|  ⬜  |  5  | [Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree/)                               | 🟡 Medium  |  18 / 6  | 🔥🔥🔥 #95 |
|  ⬜  |  6  | [Kth Smallest Element in a BST](https://leetcode.com/problems/kth-smallest-element-in-a-bst/)                           | 🟡 Medium  |  11 / 6  | 🔥🔥 #187  |
|  ⬜  |  7  | [Delete Node in a BST](https://leetcode.com/problems/delete-node-in-a-bst/)                                             | 🟡 Medium  |  11 / 2  | 🔥🔥 #289  |
|  ⬜  |  8  | [Binary Search Tree Iterator](https://leetcode.com/problems/binary-search-tree-iterator/)                               | 🟡 Medium  |  8 / 2   | 🔥🔥 #314  |
|  ⬜  |  9  | [Insert into a Binary Search Tree](https://leetcode.com/problems/insert-into-a-binary-search-tree/)                     | 🟡 Medium  |  7 / 3   | 🔥🔥 #321  |
|  ⬜  | 10  | [Recover Binary Search Tree](https://leetcode.com/problems/recover-binary-search-tree/)                                 | 🟡 Medium  |  8 / 2   | 🔥🔥 #332  |

### Lowest Common Ancestor & Path Queries

📘 Common · 8 problems

_Paths between nodes, ancestors, distances, subtree-with-property_

| Done |  #  | Problem                                                                                                                                                 | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Lowest Common Ancestor of a Binary Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/)                                       | 🟡 Medium  |  22 / 5  | 🔥🔥🔥 #85 |
|  ⬜  |  2  | [Lowest Common Ancestor of a Binary Search Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)                         | 🟡 Medium  |  10 / 4  | 🔥🔥 #207  |
|  ⬜  |  3  | [All Nodes Distance K in Binary Tree](https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/)                                               | 🟡 Medium  |  16 / 6  |  🔥 #414   |
|  ⬜  |  4  | [Amount of Time for Binary Tree to Be Infected](https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected/)                           | 🟡 Medium  |  12 / 2  |  🔥 #610   |
|  ⬜  |  5  | [Step-By-Step Directions From a Binary Tree Node to Another](https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/) | 🟡 Medium  |  6 / 2   |   #1022    |
|  ⬜  |  6  | [Longest Univalue Path](https://leetcode.com/problems/longest-univalue-path/)                                                                           | 🟡 Medium  |  6 / 0   |   #1387    |
|  ⬜  |  7  | [Maximum Difference Between Node and Ancestor](https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/)                             | 🟡 Medium  |  4 / 0   |   #1439    |
|  ⬜  |  8  | [Sum of Distances in Tree](https://leetcode.com/problems/sum-of-distances-in-tree/)                                                                     |  🔴 Hard   |  6 / 1   |   #1063    |

### Tree Construction & Serialization

📘 Common · 9 problems

_Build a tree from traversals, or encode/decode it_

| Done |  #  | Problem                                                                                                                                                 | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Merge Two Binary Trees](https://leetcode.com/problems/merge-two-binary-trees/)                                                                         |  🟢 Easy   |  8 / 0   |  🔥 #405   |
|  ⬜  |  2  | [Construct Binary Tree from Preorder and Inorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/)   | 🟡 Medium  |  10 / 5  | 🔥🔥 #190  |
|  ⬜  |  3  | [Convert Sorted List to Binary Search Tree](https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/)                                   | 🟡 Medium  |  10 / 2  | 🔥🔥 #328  |
|  ⬜  |  4  | [Find Duplicate Subtrees](https://leetcode.com/problems/find-duplicate-subtrees/)                                                                       | 🟡 Medium  |  8 / 0   |  🔥 #402   |
|  ⬜  |  5  | [Construct Binary Search Tree from Preorder Traversal](https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/)             | 🟡 Medium  |  6 / 2   |  🔥 #422   |
|  ⬜  |  6  | [Construct Binary Tree from Inorder and Postorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/) | 🟡 Medium  |  7 / 1   |  🔥 #583   |
|  ⬜  |  7  | [Serialize and Deserialize BST](https://leetcode.com/problems/serialize-and-deserialize-bst/)                                                           | 🟡 Medium  |  4 / 0   |  🔥 #749   |
|  ⬜  |  8  | [Maximum Binary Tree](https://leetcode.com/problems/maximum-binary-tree/)                                                                               | 🟡 Medium  |  4 / 0   |  🔥 #941   |
|  ⬜  |  9  | [Serialize and Deserialize Binary Tree](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/)                                           |  🔴 Hard   |  23 / 6  | 🔥🔥🔥 #93 |

### Tree DP

🧗 Occasional · 7 problems

_Each node returns a summary (best with/without it) that the parent combines_

| Done |  #  | Problem                                                                                                           | Difficulty | Asked at |   Rank    |
| :--: | :-: | ----------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [House Robber III](https://leetcode.com/problems/house-robber-iii/)                                               | 🟡 Medium  |  12 / 3  | 🔥🔥 #273 |
|  ⬜  |  2  | [Unique Binary Search Trees](https://leetcode.com/problems/unique-binary-search-trees/)                           | 🟡 Medium  |  10 / 2  | 🔥🔥 #399 |
|  ⬜  |  3  | [Distribute Coins in Binary Tree](https://leetcode.com/problems/distribute-coins-in-binary-tree/)                 | 🟡 Medium  |  8 / 2   |  🔥 #459  |
|  ⬜  |  4  | [Delete Nodes And Return Forest](https://leetcode.com/problems/delete-nodes-and-return-forest/)                   | 🟡 Medium  |  4 / 2   |  🔥 #551  |
|  ⬜  |  5  | [Maximum Product of Splitted Binary Tree](https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/) | 🟡 Medium  |  5 / 0   |   #1440   |
|  ⬜  |  6  | [Binary Tree Cameras](https://leetcode.com/problems/binary-tree-cameras/)                                         |  🔴 Hard   |  14 / 4  |  🔥 #462  |
|  ⬜  |  7  | [Maximum Sum BST in Binary Tree](https://leetcode.com/problems/maximum-sum-bst-in-binary-tree/)                   |  🔴 Hard   |  5 / 2   |  🔥 #768  |

---

## Graphs

### DFS / BFS on Grids & Graphs

⭐ Core · 15 problems

_Flood fill, islands, multi-source BFS, shortest steps in unweighted graphs_

| Done |  #  | Problem                                                                                         | Difficulty | Asked at |    Rank    |
| :--: | :-: | ----------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Island Perimeter](https://leetcode.com/problems/island-perimeter/)                             |  🟢 Easy   |  9 / 2   |  🔥 #434   |
|  ⬜  |  2  | [Flood Fill](https://leetcode.com/problems/flood-fill/)                                         |  🟢 Easy   |  12 / 4  |  🔥 #471   |
|  ⬜  |  3  | [Number of Islands](https://leetcode.com/problems/number-of-islands/)                           | 🟡 Medium  | 82 / 24  | 🔥🔥🔥 #4  |
|  ⬜  |  4  | [Rotting Oranges](https://leetcode.com/problems/rotting-oranges/)                               | 🟡 Medium  | 50 / 13  | 🔥🔥🔥 #98 |
|  ⬜  |  5  | [Max Area of Island](https://leetcode.com/problems/max-area-of-island/)                         | 🟡 Medium  |  19 / 5  | 🔥🔥 #119  |
|  ⬜  |  6  | [Shortest Path in Binary Matrix](https://leetcode.com/problems/shortest-path-in-binary-matrix/) | 🟡 Medium  |  14 / 6  | 🔥🔥 #188  |
|  ⬜  |  7  | [Surrounded Regions](https://leetcode.com/problems/surrounded-regions/)                         | 🟡 Medium  |  12 / 5  | 🔥🔥 #206  |
|  ⬜  |  8  | [Clone Graph](https://leetcode.com/problems/clone-graph/)                                       | 🟡 Medium  |  18 / 3  | 🔥🔥 #217  |
|  ⬜  |  9  | [Minesweeper](https://leetcode.com/problems/minesweeper/)                                       | 🟡 Medium  |  9 / 2   | 🔥🔥 #228  |
|  ⬜  | 10  | [Pacific Atlantic Water Flow](https://leetcode.com/problems/pacific-atlantic-water-flow/)       | 🟡 Medium  |  12 / 2  | 🔥🔥 #251  |
|  ⬜  | 11  | [01 Matrix](https://leetcode.com/problems/01-matrix/)                                           | 🟡 Medium  |  11 / 4  | 🔥🔥 #272  |
|  ⬜  | 12  | [Battleships in a Board](https://leetcode.com/problems/battleships-in-a-board/)                 | 🟡 Medium  |  8 / 2   | 🔥🔥 #338  |
|  ⬜  | 13  | [Open the Lock](https://leetcode.com/problems/open-the-lock/)                                   | 🟡 Medium  |  15 / 3  |  🔥 #478   |
|  ⬜  | 14  | [Word Ladder](https://leetcode.com/problems/word-ladder/)                                       |  🔴 Hard   |  36 / 8  | 🔥🔥🔥 #45 |
|  ⬜  | 15  | [Making A Large Island](https://leetcode.com/problems/making-a-large-island/)                   |  🔴 Hard   |  16 / 6  |  🔥 #451   |

### Connected Components & Graph DFS

🧗 Occasional · 8 problems

_Count/label groups, reachability, build adjacency lists from edges_

| Done |  #  | Problem                                                                                                                                         | Difficulty | Asked at |   Rank    |
| :--: | :-: | ----------------------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Find the Town Judge](https://leetcode.com/problems/find-the-town-judge/)                                                                       |  🟢 Easy   |  6 / 0   |  🔥 #594  |
|  ⬜  |  2  | [Find Center of Star Graph](https://leetcode.com/problems/find-center-of-star-graph/)                                                           |  🟢 Easy   |  4 / 2   |  🔥 #985  |
|  ⬜  |  3  | [Evaluate Division](https://leetcode.com/problems/evaluate-division/)                                                                           | 🟡 Medium  |  22 / 5  | 🔥🔥 #144 |
|  ⬜  |  4  | [Keys and Rooms](https://leetcode.com/problems/keys-and-rooms/)                                                                                 | 🟡 Medium  |  9 / 2   | 🔥🔥 #396 |
|  ⬜  |  5  | [Snakes and Ladders](https://leetcode.com/problems/snakes-and-ladders/)                                                                         | 🟡 Medium  |  11 / 2  |  🔥 #514  |
|  ⬜  |  6  | [Reorder Routes to Make All Paths Lead to the City Zero](https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/) | 🟡 Medium  |  5 / 1   |  🔥 #694  |
|  ⬜  |  7  | [Number of Provinces](https://leetcode.com/problems/number-of-provinces/)                                                                       | 🟡 Medium  |  9 / 4   |  🔥 #859  |
|  ⬜  |  8  | [All Paths From Source to Target](https://leetcode.com/problems/all-paths-from-source-to-target/)                                               | 🟡 Medium  |  5 / 2   |  🔥 #906  |

### Topological Sort

📘 Common · 9 problems

_Dependencies, ordering, prerequisites, "can everything be finished?"_

| Done |  #  | Problem                                                                                                                       | Difficulty | Asked at |    Rank    |
| :--: | :-: | ----------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Course Schedule II](https://leetcode.com/problems/course-schedule-ii/)                                                       | 🟡 Medium  | 36 / 14  | 🔥🔥🔥 #23 |
|  ⬜  |  2  | [Course Schedule](https://leetcode.com/problems/course-schedule/)                                                             | 🟡 Medium  | 44 / 12  | 🔥🔥🔥 #24 |
|  ⬜  |  3  | [Minimum Height Trees](https://leetcode.com/problems/minimum-height-trees/)                                                   | 🟡 Medium  |  7 / 2   |  🔥 #415   |
|  ⬜  |  4  | [Find Eventual Safe States](https://leetcode.com/problems/find-eventual-safe-states/)                                         | 🟡 Medium  |  7 / 2   |  🔥 #785   |
|  ⬜  |  5  | [Find All Possible Recipes from Given Supplies](https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/) | 🟡 Medium  |  8 / 2   |  🔥 #914   |
|  ⬜  |  6  | [Loud and Rich](https://leetcode.com/problems/loud-and-rich/)                                                                 | 🟡 Medium  |  7 / 0   |  🔥 #921   |
|  ⬜  |  7  | [Longest Increasing Path in a Matrix](https://leetcode.com/problems/longest-increasing-path-in-a-matrix/)                     |  🔴 Hard   |  18 / 5  |  🔥 #432   |
|  ⬜  |  8  | [Parallel Courses III](https://leetcode.com/problems/parallel-courses-iii/)                                                   |  🔴 Hard   |  10 / 4  |  🔥 #557   |
|  ⬜  |  9  | [Collect Coins in a Tree](https://leetcode.com/problems/collect-coins-in-a-tree/)                                             |  🔴 Hard   |  12 / 0  |  🔥 #772   |

### Union-Find (DSU)

🧗 Occasional · 7 problems

_Dynamic connectivity, grouping, merging accounts/equivalences_

| Done |  #  | Problem                                                                                                                         | Difficulty | Asked at |   Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Accounts Merge](https://leetcode.com/problems/accounts-merge/)                                                                 | 🟡 Medium  |  17 / 6  | 🔥🔥 #123 |
|  ⬜  |  2  | [Redundant Connection](https://leetcode.com/problems/redundant-connection/)                                                     | 🟡 Medium  |  8 / 4   | 🔥🔥 #341 |
|  ⬜  |  3  | [Smallest String With Swaps](https://leetcode.com/problems/smallest-string-with-swaps/)                                         | 🟡 Medium  |  8 / 2   |  🔥 #644  |
|  ⬜  |  4  | [Most Stones Removed with Same Row or Column](https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/)       | 🟡 Medium  |  10 / 2  |  🔥 #702  |
|  ⬜  |  5  | [Number of Operations to Make Network Connected](https://leetcode.com/problems/number-of-operations-to-make-network-connected/) | 🟡 Medium  |  10 / 1  |  🔥 #756  |
|  ⬜  |  6  | [Satisfiability of Equality Equations](https://leetcode.com/problems/satisfiability-of-equality-equations/)                     | 🟡 Medium  |  6 / 2   |  🔥 #966  |
|  ⬜  |  7  | [Find All People With Secret](https://leetcode.com/problems/find-all-people-with-secret/)                                       |  🔴 Hard   |  2 / 1   |   #1500   |

### Shortest Path (BFS, Dijkstra, Bellman-Ford, Floyd-Warshall)

🧗 Occasional · 9 problems

_Weighted edges, constraints on stops/cost, "minimum effort/time" on a grid_

| Done |  #  | Problem                                                                                           | Difficulty | Asked at |   Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Network Delay Time](https://leetcode.com/problems/network-delay-time/)                           | 🟡 Medium  |  10 / 3  | 🔥🔥 #285 |
|  ⬜  |  2  | [Cheapest Flights Within K Stops](https://leetcode.com/problems/cheapest-flights-within-k-stops/) | 🟡 Medium  |  18 / 6  |  🔥 #426  |
|  ⬜  |  3  | [Path With Minimum Effort](https://leetcode.com/problems/path-with-minimum-effort/)               | 🟡 Medium  |  9 / 4   |  🔥 #878  |
|  ⬜  |  4  | [Minimum Genetic Mutation](https://leetcode.com/problems/minimum-genetic-mutation/)               | 🟡 Medium  |  6 / 2   |  🔥 #909  |
|  ⬜  |  5  | [Find a Safe Walk Through a Grid](https://leetcode.com/problems/find-a-safe-walk-through-a-grid/) | 🟡 Medium  |  3 / 2   |  🔥 #924  |
|  ⬜  |  6  | [Path with Maximum Probability](https://leetcode.com/problems/path-with-maximum-probability/)     | 🟡 Medium  |  4 / 1   |  🔥 #982  |
|  ⬜  |  7  | [Word Ladder II](https://leetcode.com/problems/word-ladder-ii/)                                   |  🔴 Hard   |  14 / 2  | 🔥🔥 #308 |
|  ⬜  |  8  | [Bus Routes](https://leetcode.com/problems/bus-routes/)                                           |  🔴 Hard   |  17 / 5  |  🔥 #442  |
|  ⬜  |  9  | [Swim in Rising Water](https://leetcode.com/problems/swim-in-rising-water/)                       |  🔴 Hard   |  8 / 2   |  🔥 #648  |

### Cycle Detection & Bipartite Check

📘 Common · 9 problems

_Some problems are cross-listed from other sections on purpose_

| Done |  #  | Problem                                                                               | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Linked List Cycle](https://leetcode.com/problems/linked-list-cycle/)                 |  🟢 Easy   |  24 / 6  | 🔥🔥🔥 #99 |
|  ⬜  |  2  | [Course Schedule](https://leetcode.com/problems/course-schedule/)                     | 🟡 Medium  | 44 / 12  | 🔥🔥🔥 #24 |
|  ⬜  |  3  | [Linked List Cycle II](https://leetcode.com/problems/linked-list-cycle-ii/)           | 🟡 Medium  |  12 / 4  | 🔥🔥 #239  |
|  ⬜  |  4  | [Redundant Connection](https://leetcode.com/problems/redundant-connection/)           | 🟡 Medium  |  8 / 4   | 🔥🔥 #341  |
|  ⬜  |  5  | [Possible Bipartition](https://leetcode.com/problems/possible-bipartition/)           | 🟡 Medium  |  11 / 2  |  🔥 #526   |
|  ⬜  |  6  | [Is Graph Bipartite?](https://leetcode.com/problems/is-graph-bipartite/)              | 🟡 Medium  |  10 / 3  |  🔥 #540   |
|  ⬜  |  7  | [Circular Array Loop](https://leetcode.com/problems/circular-array-loop/)             | 🟡 Medium  |  4 / 2   |  🔥 #731   |
|  ⬜  |  8  | [Find Eventual Safe States](https://leetcode.com/problems/find-eventual-safe-states/) | 🟡 Medium  |  7 / 2   |  🔥 #785   |
|  ⬜  |  9  | [Longest Cycle in a Graph](https://leetcode.com/problems/longest-cycle-in-a-graph/)   |  🔴 Hard   |  7 / 1   |  🔥 #728   |

### MST, Bridges & Advanced Graph

🧗 Occasional · 2 problems

_Rare outside top-tier interviews; learn Kruskal/Prim and Tarjan only after the core graph sections_

| Done |  #  | Problem                                                                                               | Difficulty | Asked at |   Rank    |
| :--: | :-: | ----------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Reconstruct Itinerary](https://leetcode.com/problems/reconstruct-itinerary/)                         |  🔴 Hard   |  22 / 4  | 🔥🔥 #129 |
|  ⬜  |  2  | [Critical Connections in a Network](https://leetcode.com/problems/critical-connections-in-a-network/) |  🔴 Hard   |  9 / 3   |  🔥 #593  |

---

## Heap / Priority Queue

### Top-K Elements

⭐ Core · 14 problems

_K largest/smallest/most frequent: keep a heap of size K_

| Done |  #  | Problem                                                                                                                   | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Kth Largest Element in a Stream](https://leetcode.com/problems/kth-largest-element-in-a-stream/)                         |  🟢 Easy   |  10 / 3  | 🔥🔥 #280  |
|  ⬜  |  2  | [Last Stone Weight](https://leetcode.com/problems/last-stone-weight/)                                                     |  🟢 Easy   |  12 / 4  | 🔥🔥 #324  |
|  ⬜  |  3  | [Maximum Product of Two Elements in an Array](https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/) |  🟢 Easy   |  8 / 2   |  🔥 #477   |
|  ⬜  |  4  | [Relative Ranks](https://leetcode.com/problems/relative-ranks/)                                                           |  🟢 Easy   |  4 / 1   |  🔥 #938   |
|  ⬜  |  5  | [Kth Largest Element in an Array](https://leetcode.com/problems/kth-largest-element-in-an-array/)                         | 🟡 Medium  | 39 / 10  | 🔥🔥🔥 #31 |
|  ⬜  |  6  | [Task Scheduler](https://leetcode.com/problems/task-scheduler/)                                                           | 🟡 Medium  |  23 / 6  | 🔥🔥 #103  |
|  ⬜  |  7  | [Reorganize String](https://leetcode.com/problems/reorganize-string/)                                                     | 🟡 Medium  |  18 / 5  | 🔥🔥 #113  |
|  ⬜  |  8  | [Top K Frequent Words](https://leetcode.com/problems/top-k-frequent-words/)                                               | 🟡 Medium  |  22 / 4  | 🔥🔥 #120  |
|  ⬜  |  9  | [K Closest Points to Origin](https://leetcode.com/problems/k-closest-points-to-origin/)                                   | 🟡 Medium  |  16 / 4  | 🔥🔥 #155  |
|  ⬜  | 10  | [Sort Characters By Frequency](https://leetcode.com/problems/sort-characters-by-frequency/)                               | 🟡 Medium  |  15 / 5  | 🔥🔥 #191  |
|  ⬜  | 11  | [Find K Pairs with Smallest Sums](https://leetcode.com/problems/find-k-pairs-with-smallest-sums/)                         | 🟡 Medium  |  10 / 3  | 🔥🔥 #269  |
|  ⬜  | 12  | [Ugly Number II](https://leetcode.com/problems/ugly-number-ii/)                                                           | 🟡 Medium  |  8 / 4   | 🔥🔥 #362  |
|  ⬜  | 13  | [Furthest Building You Can Reach](https://leetcode.com/problems/furthest-building-you-can-reach/)                         | 🟡 Medium  |  14 / 2  |  🔥 #475   |
|  ⬜  | 14  | [Minimum Cost to Hire K Workers](https://leetcode.com/problems/minimum-cost-to-hire-k-workers/)                           |  🔴 Hard   |  4 / 2   |  🔥 #778   |

### K-Way Merge

📘 Common · 7 problems

_Merge several sorted sources by always popping the smallest head_

| Done |  #  | Problem                                                                                                                       | Difficulty | Asked at |    Rank    |
| :--: | :-: | ----------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/)                                                       |  🟢 Easy   |  36 / 8  | 🔥🔥🔥 #25 |
|  ⬜  |  2  | [Intersection of Two Arrays II](https://leetcode.com/problems/intersection-of-two-arrays-ii/)                                 |  🟢 Easy   |  9 / 2   | 🔥🔥 #271  |
|  ⬜  |  3  | [Find K Pairs with Smallest Sums](https://leetcode.com/problems/find-k-pairs-with-smallest-sums/)                             | 🟡 Medium  |  10 / 3  | 🔥🔥 #269  |
|  ⬜  |  4  | [Kth Smallest Element in a Sorted Matrix](https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/)             | 🟡 Medium  |  10 / 2  | 🔥🔥 #347  |
|  ⬜  |  5  | [Super Ugly Number](https://leetcode.com/problems/super-ugly-number/)                                                         | 🟡 Medium  |  5 / 0   |  🔥 #819   |
|  ⬜  |  6  | [Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/)                                                   |  🔴 Hard   | 48 / 12  | 🔥🔥🔥 #13 |
|  ⬜  |  7  | [Smallest Range Covering Elements from K Lists](https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/) |  🔴 Hard   |  10 / 2  |  🔥 #572   |

### Two Heaps & Scheduling with Heaps

🧗 Occasional · 10 problems

_Running median, "always pick the best available", resource scheduling_

| Done |  #  | Problem                                                                                                                       | Difficulty | Asked at |   Rank    |
| :--: | :-: | ----------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Maximum Number of Events That Can Be Attended](https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/) | 🟡 Medium  |  16 / 3  | 🔥🔥 #333 |
|  ⬜  |  2  | [Find the Safest Path in a Grid](https://leetcode.com/problems/find-the-safest-path-in-a-grid/)                               | 🟡 Medium  |  8 / 4   |  🔥 #592  |
|  ⬜  |  3  | [Exam Room](https://leetcode.com/problems/exam-room/)                                                                         | 🟡 Medium  |  6 / 2   |  🔥 #693  |
|  ⬜  |  4  | [Avoid Flood in The City](https://leetcode.com/problems/avoid-flood-in-the-city/)                                             | 🟡 Medium  |  6 / 0   |  🔥 #903  |
|  ⬜  |  5  | [Trapping Rain Water II](https://leetcode.com/problems/trapping-rain-water-ii/)                                               |  🔴 Hard   |  13 / 4  | 🔥🔥 #319 |
|  ⬜  |  6  | [Sliding Window Median](https://leetcode.com/problems/sliding-window-median/)                                                 |  🔴 Hard   |  10 / 2  | 🔥🔥 #349 |
|  ⬜  |  7  | [Find Median from Data Stream](https://leetcode.com/problems/find-median-from-data-stream/)                                   |  🔴 Hard   | 40 / 10  | 🔥🔥 #361 |
|  ⬜  |  8  | [Minimum Number of Refueling Stops](https://leetcode.com/problems/minimum-number-of-refueling-stops/)                         |  🔴 Hard   |  8 / 4   |  🔥 #527  |
|  ⬜  |  9  | [IPO](https://leetcode.com/problems/ipo/)                                                                                     |  🔴 Hard   |  8 / 3   |  🔥 #699  |
|  ⬜  | 10  | [Maximum Number of Points From Grid Queries](https://leetcode.com/problems/maximum-number-of-points-from-grid-queries/)       |  🔴 Hard   |  6 / 1   |  🔥 #860  |

---

## Recursion & Backtracking

### Subsets, Permutations & Combinations

⭐ Core · 10 problems

_Enumerate all choices: choose, recurse, un-choose_

| Done |  #  | Problem                                                                                                       | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Generate Parentheses](https://leetcode.com/problems/generate-parentheses/)                                   | 🟡 Medium  |  38 / 7  | 🔥🔥🔥 #33 |
|  ⬜  |  2  | [Letter Combinations of a Phone Number](https://leetcode.com/problems/letter-combinations-of-a-phone-number/) | 🟡 Medium  |  33 / 8  | 🔥🔥🔥 #49 |
|  ⬜  |  3  | [Permutations](https://leetcode.com/problems/permutations/)                                                   | 🟡 Medium  |  24 / 6  | 🔥🔥🔥 #66 |
|  ⬜  |  4  | [Subsets](https://leetcode.com/problems/subsets/)                                                             | 🟡 Medium  |  20 / 7  | 🔥🔥🔥 #71 |
|  ⬜  |  5  | [Combination Sum](https://leetcode.com/problems/combination-sum/)                                             | 🟡 Medium  |  26 / 5  | 🔥🔥 #105  |
|  ⬜  |  6  | [Combination Sum II](https://leetcode.com/problems/combination-sum-ii/)                                       | 🟡 Medium  |  15 / 4  | 🔥🔥 #201  |
|  ⬜  |  7  | [Subsets II](https://leetcode.com/problems/subsets-ii/)                                                       | 🟡 Medium  |  8 / 6   | 🔥🔥 #249  |
|  ⬜  |  8  | [Combinations](https://leetcode.com/problems/combinations/)                                                   | 🟡 Medium  |  8 / 4   | 🔥🔥 #290  |
|  ⬜  |  9  | [Permutations II](https://leetcode.com/problems/permutations-ii/)                                             | 🟡 Medium  |  8 / 2   | 🔥🔥 #356  |
|  ⬜  | 10  | [Permutation Sequence](https://leetcode.com/problems/permutation-sequence/)                                   |  🔴 Hard   |  8 / 4   | 🔥🔥 #346  |

### Constraint Search (Grid & Board Problems)

📘 Common · 8 problems

_Place/choose with pruning: boards, words, grids_

| Done |  #  | Problem                                                                                                                                                         | Difficulty | Asked at |    Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Word Search](https://leetcode.com/problems/word-search/)                                                                                                       | 🟡 Medium  |  34 / 8  | 🔥🔥🔥 #48 |
|  ⬜  |  2  | [Maximum Length of a Concatenated String with Unique Characters](https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/) | 🟡 Medium  |  6 / 1   |  🔥 #871   |
|  ⬜  |  3  | [Matchsticks to Square](https://leetcode.com/problems/matchsticks-to-square/)                                                                                   | 🟡 Medium  |  8 / 0   |   #1406    |
|  ⬜  |  4  | [N-Queens](https://leetcode.com/problems/n-queens/)                                                                                                             |  🔴 Hard   |  16 / 5  | 🔥🔥 #131  |
|  ⬜  |  5  | [Sudoku Solver](https://leetcode.com/problems/sudoku-solver/)                                                                                                   |  🔴 Hard   |  16 / 4  | 🔥🔥 #168  |
|  ⬜  |  6  | [Remove Invalid Parentheses](https://leetcode.com/problems/remove-invalid-parentheses/)                                                                         |  🔴 Hard   |  10 / 3  | 🔥🔥 #345  |
|  ⬜  |  7  | [Unique Paths III](https://leetcode.com/problems/unique-paths-iii/)                                                                                             |  🔴 Hard   |  7 / 2   |  🔥 #473   |
|  ⬜  |  8  | [N-Queens II](https://leetcode.com/problems/n-queens-ii/)                                                                                                       |  🔴 Hard   |  10 / 1  |  🔥 #496   |

### Partition Problems

📘 Common · 7 problems

_Split a string/array into valid pieces, or into k groups_

| Done |  #  | Problem                                                                                             | Difficulty | Asked at |   Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Restore IP Addresses](https://leetcode.com/problems/restore-ip-addresses/)                         | 🟡 Medium  |  20 / 5  | 🔥🔥 #142 |
|  ⬜  |  2  | [Palindrome Partitioning](https://leetcode.com/problems/palindrome-partitioning/)                   | 🟡 Medium  |  10 / 8  | 🔥🔥 #177 |
|  ⬜  |  3  | [Partition to K Equal Sum Subsets](https://leetcode.com/problems/partition-to-k-equal-sum-subsets/) | 🟡 Medium  |  8 / 3   | 🔥🔥 #305 |
|  ⬜  |  4  | [Additive Number](https://leetcode.com/problems/additive-number/)                                   | 🟡 Medium  |  2 / 1   |  🔥 #705  |
|  ⬜  |  5  | [Find Unique Binary String](https://leetcode.com/problems/find-unique-binary-string/)               | 🟡 Medium  |  4 / 1   |   #1081   |
|  ⬜  |  6  | [Word Break II](https://leetcode.com/problems/word-break-ii/)                                       |  🔴 Hard   |  16 / 5  | 🔥🔥 #115 |
|  ⬜  |  7  | [Expression Add Operators](https://leetcode.com/problems/expression-add-operators/)                 |  🔴 Hard   |  6 / 2   |  🔥 #409  |

---

## Dynamic Programming

### 1D DP

⭐ Core · 15 problems

_dp[i] depends on a few earlier states: stairs, rob, decode, break_

| Done |  #  | Problem                                                                                             | Difficulty | Asked at |    Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Climbing Stairs](https://leetcode.com/problems/climbing-stairs/)                                   |  🟢 Easy   |  34 / 9  | 🔥🔥🔥 #43 |
|  ⬜  |  2  | [Pascal's Triangle](https://leetcode.com/problems/pascals-triangle/)                                |  🟢 Easy   |  16 / 6  | 🔥🔥 #116  |
|  ⬜  |  3  | [Fibonacci Number](https://leetcode.com/problems/fibonacci-number/)                                 |  🟢 Easy   |  16 / 6  | 🔥🔥 #126  |
|  ⬜  |  4  | [Min Cost Climbing Stairs](https://leetcode.com/problems/min-cost-climbing-stairs/)                 |  🟢 Easy   |  7 / 4   | 🔥🔥 #360  |
|  ⬜  |  5  | [Counting Bits](https://leetcode.com/problems/counting-bits/)                                       |  🟢 Easy   |  8 / 2   | 🔥🔥 #382  |
|  ⬜  |  6  | [N-th Tribonacci Number](https://leetcode.com/problems/n-th-tribonacci-number/)                     |  🟢 Easy   |  6 / 2   |  🔥 #602   |
|  ⬜  |  7  | [House Robber](https://leetcode.com/problems/house-robber/)                                         | 🟡 Medium  | 37 / 10  | 🔥🔥🔥 #39 |
|  ⬜  |  8  | [Word Break](https://leetcode.com/problems/word-break/)                                             | 🟡 Medium  |  29 / 8  | 🔥🔥🔥 #51 |
|  ⬜  |  9  | [Decode Ways](https://leetcode.com/problems/decode-ways/)                                           | 🟡 Medium  |  21 / 6  | 🔥🔥🔥 #79 |
|  ⬜  | 10  | [House Robber II](https://leetcode.com/problems/house-robber-ii/)                                   | 🟡 Medium  |  18 / 5  | 🔥🔥 #230  |
|  ⬜  | 11  | [Delete and Earn](https://leetcode.com/problems/delete-and-earn/)                                   | 🟡 Medium  |  12 / 4  | 🔥🔥 #247  |
|  ⬜  | 12  | [Integer Break](https://leetcode.com/problems/integer-break/)                                       | 🟡 Medium  |  6 / 3   |  🔥 #417   |
|  ⬜  | 13  | [Minimum Cost For Tickets](https://leetcode.com/problems/minimum-cost-for-tickets/)                 | 🟡 Medium  |  12 / 2  |  🔥 #529   |
|  ⬜  | 14  | [Frog Jump](https://leetcode.com/problems/frog-jump/)                                               |  🔴 Hard   |  11 / 2  | 🔥🔥 #267  |
|  ⬜  | 15  | [Maximum Profit in Job Scheduling](https://leetcode.com/problems/maximum-profit-in-job-scheduling/) |  🔴 Hard   |  22 / 7  |  🔥 #407   |

### Knapsack (0/1, Unbounded, Subset Sum)

📘 Common · 8 problems

_Pick items under a capacity/target; count ways or find min/max_

| Done |  #  | Problem                                                                                 | Difficulty | Asked at |    Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Coin Change](https://leetcode.com/problems/coin-change/)                               | 🟡 Medium  |  38 / 8  | 🔥🔥🔥 #35 |
|  ⬜  |  2  | [Partition Equal Subset Sum](https://leetcode.com/problems/partition-equal-subset-sum/) | 🟡 Medium  |  15 / 4  | 🔥🔥 #182  |
|  ⬜  |  3  | [Perfect Squares](https://leetcode.com/problems/perfect-squares/)                       | 🟡 Medium  |  11 / 4  | 🔥🔥 #200  |
|  ⬜  |  4  | [Target Sum](https://leetcode.com/problems/target-sum/)                                 | 🟡 Medium  |  10 / 4  | 🔥🔥 #264  |
|  ⬜  |  5  | [Combination Sum IV](https://leetcode.com/problems/combination-sum-iv/)                 | 🟡 Medium  |  6 / 2   |  🔥 #501   |
|  ⬜  |  6  | [Coin Change II](https://leetcode.com/problems/coin-change-ii/)                         | 🟡 Medium  |  12 / 4  |  🔥 #825   |
|  ⬜  |  7  | [Ones and Zeroes](https://leetcode.com/problems/ones-and-zeroes/)                       | 🟡 Medium  |  5 / 1   |  🔥 #828   |
|  ⬜  |  8  | [Last Stone Weight II](https://leetcode.com/problems/last-stone-weight-ii/)             | 🟡 Medium  |  5 / 2   |  🔥 #872   |

### Subsequence DP (LIS, LCS, Edit Distance)

⭐ Core · 10 problems

_Two strings/arrays compared index by index, or best subsequence ending at i_

| Done |  #  | Problem                                                                                                             | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Edit Distance](https://leetcode.com/problems/edit-distance/)                                                       | 🟡 Medium  |  23 / 8  | 🔥🔥🔥 #72 |
|  ⬜  |  2  | [Longest Palindromic Subsequence](https://leetcode.com/problems/longest-palindromic-subsequence/)                   | 🟡 Medium  |  14 / 6  | 🔥🔥 #156  |
|  ⬜  |  3  | [Longest Common Subsequence](https://leetcode.com/problems/longest-common-subsequence/)                             | 🟡 Medium  |  19 / 6  | 🔥🔥 #157  |
|  ⬜  |  4  | [Interleaving String](https://leetcode.com/problems/interleaving-string/)                                           | 🟡 Medium  |  9 / 2   | 🔥🔥 #311  |
|  ⬜  |  5  | [Longest Increasing Subsequence](https://leetcode.com/problems/longest-increasing-subsequence/)                     | 🟡 Medium  |  28 / 8  | 🔥🔥 #376  |
|  ⬜  |  6  | [Number of Longest Increasing Subsequence](https://leetcode.com/problems/number-of-longest-increasing-subsequence/) | 🟡 Medium  |  6 / 3   |  🔥 #429   |
|  ⬜  |  7  | [Regular Expression Matching](https://leetcode.com/problems/regular-expression-matching/)                           |  🔴 Hard   |  24 / 6  | 🔥🔥🔥 #76 |
|  ⬜  |  8  | [Wildcard Matching](https://leetcode.com/problems/wildcard-matching/)                                               |  🔴 Hard   |  17 / 4  | 🔥🔥 #185  |
|  ⬜  |  9  | [Distinct Subsequences](https://leetcode.com/problems/distinct-subsequences/)                                       |  🔴 Hard   |  12 / 4  | 🔥🔥 #215  |
|  ⬜  | 10  | [Russian Doll Envelopes](https://leetcode.com/problems/russian-doll-envelopes/)                                     |  🔴 Hard   |  14 / 3  | 🔥🔥 #274  |

### Grid DP

📘 Common · 9 problems

_Paths and best values on a matrix, moving right/down_

| Done |  #  | Problem                                                                                                         | Difficulty | Asked at |   Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Minimum Path Sum](https://leetcode.com/problems/minimum-path-sum/)                                             | 🟡 Medium  |  16 / 7  | 🔥🔥 #101 |
|  ⬜  |  2  | [Unique Paths](https://leetcode.com/problems/unique-paths/)                                                     | 🟡 Medium  |  19 / 6  | 🔥🔥 #102 |
|  ⬜  |  3  | [Maximal Square](https://leetcode.com/problems/maximal-square/)                                                 | 🟡 Medium  |  26 / 4  | 🔥🔥 #148 |
|  ⬜  |  4  | [Unique Paths II](https://leetcode.com/problems/unique-paths-ii/)                                               | 🟡 Medium  |  10 / 5  | 🔥🔥 #193 |
|  ⬜  |  5  | [Triangle](https://leetcode.com/problems/triangle/)                                                             | 🟡 Medium  |  10 / 4  | 🔥🔥 #275 |
|  ⬜  |  6  | [Minimum Falling Path Sum](https://leetcode.com/problems/minimum-falling-path-sum/)                             | 🟡 Medium  |  8 / 2   |  🔥 #425  |
|  ⬜  |  7  | [Count Square Submatrices with All Ones](https://leetcode.com/problems/count-square-submatrices-with-all-ones/) | 🟡 Medium  |  7 / 2   |  🔥 #476  |
|  ⬜  |  8  | [Knight Probability in Chessboard](https://leetcode.com/problems/knight-probability-in-chessboard/)             | 🟡 Medium  |  5 / 0   |  🔥 #647  |
|  ⬜  |  9  | [Dungeon Game](https://leetcode.com/problems/dungeon-game/)                                                     |  🔴 Hard   |  9 / 2   | 🔥🔥 #351 |

### Interval DP

🧗 Occasional · 9 problems

_dp`[l][r]` over a range, splitting at every k inside it_

| Done |  #  | Problem                                                                                           | Difficulty | Asked at |   Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Predict the Winner](https://leetcode.com/problems/predict-the-winner/)                           | 🟡 Medium  |  7 / 4   | 🔥🔥 #350 |
|  ⬜  |  2  | [Stone Game](https://leetcode.com/problems/stone-game/)                                           | 🟡 Medium  |  4 / 2   |  🔥 #651  |
|  ⬜  |  3  | [Guess Number Higher or Lower II](https://leetcode.com/problems/guess-number-higher-or-lower-ii/) | 🟡 Medium  |  3 / 1   |  🔥 #867  |
|  ⬜  |  4  | [Stone Game II](https://leetcode.com/problems/stone-game-ii/)                                     | 🟡 Medium  |  4 / 2   |  🔥 #901  |
|  ⬜  |  5  | [Burst Balloons](https://leetcode.com/problems/burst-balloons/)                                   |  🔴 Hard   |  16 / 4  | 🔥🔥 #194 |
|  ⬜  |  6  | [Remove Boxes](https://leetcode.com/problems/remove-boxes/)                                       |  🔴 Hard   |  11 / 2  | 🔥🔥 #355 |
|  ⬜  |  7  | [Strange Printer](https://leetcode.com/problems/strange-printer/)                                 |  🔴 Hard   |  8 / 2   | 🔥🔥 #377 |
|  ⬜  |  8  | [Palindrome Partitioning II](https://leetcode.com/problems/palindrome-partitioning-ii/)           |  🔴 Hard   |  8 / 4   | 🔥🔥 #383 |
|  ⬜  |  9  | [Minimum Cost to Cut a Stick](https://leetcode.com/problems/minimum-cost-to-cut-a-stick/)         |  🔴 Hard   |  8 / 3   |  🔥 #730  |

### State Machine DP (Stocks & Friends)

📘 Common · 8 problems

_Track a small "mode" (holding, cooldown, k transactions) per index_

| Done |  #  | Problem                                                                                                                                     | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Best Time to Buy and Sell Stock II](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/)                                     | 🟡 Medium  |  27 / 8  | 🔥🔥🔥 #63 |
|  ⬜  |  2  | [Knight Dialer](https://leetcode.com/problems/knight-dialer/)                                                                               | 🟡 Medium  |  6 / 1   | 🔥🔥 #326  |
|  ⬜  |  3  | [Best Time to Buy and Sell Stock with Transaction Fee](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/) | 🟡 Medium  |  8 / 2   | 🔥🔥 #371  |
|  ⬜  |  4  | [Best Time to Buy and Sell Stock with Cooldown](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/)               | 🟡 Medium  |  8 / 3   |  🔥 #606   |
|  ⬜  |  5  | [Best Sightseeing Pair](https://leetcode.com/problems/best-sightseeing-pair/)                                                               | 🟡 Medium  |  6 / 1   |  🔥 #759   |
|  ⬜  |  6  | [Best Time to Buy and Sell Stock III](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/)                                   |  🔴 Hard   |  13 / 3  | 🔥🔥 #246  |
|  ⬜  |  7  | [Best Time to Buy and Sell Stock IV](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/)                                     |  🔴 Hard   |  12 / 2  | 🔥🔥 #313  |
|  ⬜  |  8  | [Stone Game III](https://leetcode.com/problems/stone-game-iii/)                                                                             |  🔴 Hard   |  4 / 2   |   #1147    |

### Bitmask DP

🧗 Occasional · 3 problems

_n is small (about 20): encode the chosen subset as a bitmask_

| Done |  #  | Problem                                                                                                                                                 | Difficulty | Asked at |  Rank   |
| :--: | :-: | ------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :-----: |
|  ⬜  |  1  | [Shopping Offers](https://leetcode.com/problems/shopping-offers/)                                                                                       | 🟡 Medium  |  6 / 1   | 🔥 #516 |
|  ⬜  |  2  | [Minimum Moves to Spread Stones Over Grid](https://leetcode.com/problems/minimum-moves-to-spread-stones-over-grid/)                                     | 🟡 Medium  |  4 / 0   | 🔥 #980 |
|  ⬜  |  3  | [Partition Array Into Two Arrays to Minimize Sum Difference](https://leetcode.com/problems/partition-array-into-two-arrays-to-minimize-sum-difference/) |  🔴 Hard   |  8 / 4   | 🔥 #884 |

### Digit DP

🧗 Occasional · 2 problems

_Count numbers up to N with a digit property; rare, so only 7 here_

| Done |  #  | Problem                                                                   | Difficulty | Asked at |  Rank   |
| :--: | :-: | ------------------------------------------------------------------------- | :--------: | :------: | :-----: |
|  ⬜  |  1  | [Rotated Digits](https://leetcode.com/problems/rotated-digits/)           | 🟡 Medium  |  6 / 2   | 🔥 #507 |
|  ⬜  |  2  | [Number of Digit One](https://leetcode.com/problems/number-of-digit-one/) |  🔴 Hard   |  6 / 3   | 🔥 #461 |

---

## Greedy

### Greedy (Jump, Gas Station, Sort + Scan, Exchange Arguments)

⭐ Core · 15 problems

_A locally best choice is provably globally best; usually sort first_

| Done |  #  | Problem                                                                                                 | Difficulty | Asked at |    Rank    |
| :--: | :-: | ------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Can Place Flowers](https://leetcode.com/problems/can-place-flowers/)                                   |  🟢 Easy   |  15 / 5  | 🔥🔥 #189  |
|  ⬜  |  2  | [Lemonade Change](https://leetcode.com/problems/lemonade-change/)                                       |  🟢 Easy   |  8 / 4   | 🔥🔥 #327  |
|  ⬜  |  3  | [Longest Palindrome](https://leetcode.com/problems/longest-palindrome/)                                 |  🟢 Easy   |  8 / 4   |  🔥 #421   |
|  ⬜  |  4  | [Split a String in Balanced Strings](https://leetcode.com/problems/split-a-string-in-balanced-strings/) |  🟢 Easy   |  4 / 1   |  🔥 #573   |
|  ⬜  |  5  | [Maximum Units on a Truck](https://leetcode.com/problems/maximum-units-on-a-truck/)                     |  🟢 Easy   |  8 / 2   |  🔥 #595   |
|  ⬜  |  6  | [Jump Game](https://leetcode.com/problems/jump-game/)                                                   | 🟡 Medium  |  32 / 7  | 🔥🔥🔥 #60 |
|  ⬜  |  7  | [Jump Game II](https://leetcode.com/problems/jump-game-ii/)                                             | 🟡 Medium  |  27 / 6  | 🔥🔥🔥 #78 |
|  ⬜  |  8  | [Gas Station](https://leetcode.com/problems/gas-station/)                                               | 🟡 Medium  |  28 / 8  | 🔥🔥🔥 #82 |
|  ⬜  |  9  | [Largest Number](https://leetcode.com/problems/largest-number/)                                         | 🟡 Medium  |  20 / 6  | 🔥🔥🔥 #92 |
|  ⬜  | 10  | [Valid Parenthesis String](https://leetcode.com/problems/valid-parenthesis-string/)                     | 🟡 Medium  |  16 / 7  | 🔥🔥 #171  |
|  ⬜  | 11  | [Increasing Triplet Subsequence](https://leetcode.com/problems/increasing-triplet-subsequence/)         | 🟡 Medium  |  6 / 4   | 🔥🔥 #372  |
|  ⬜  | 12  | [Two City Scheduling](https://leetcode.com/problems/two-city-scheduling/)                               | 🟡 Medium  |  6 / 2   |  🔥 #413   |
|  ⬜  | 13  | [Wiggle Sort II](https://leetcode.com/problems/wiggle-sort-ii/)                                         | 🟡 Medium  |  5 / 2   |  🔥 #465   |
|  ⬜  | 14  | [Queue Reconstruction by Height](https://leetcode.com/problems/queue-reconstruction-by-height/)         | 🟡 Medium  |  6 / 1   |  🔥 #628   |
|  ⬜  | 15  | [Candy](https://leetcode.com/problems/candy/)                                                           |  🔴 Hard   |  20 / 5  | 🔥🔥 #255  |

---

## Other Patterns

### Trie (Prefix Tree)

🧗 Occasional · 10 problems

_Prefix search, autocomplete, word dictionaries, XOR tricks_

| Done |  #  | Problem                                                                                                                     | Difficulty | Asked at |   Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Implement Trie (Prefix Tree)](https://leetcode.com/problems/implement-trie-prefix-tree/)                                   | 🟡 Medium  |  20 / 4  | 🔥🔥 #173 |
|  ⬜  |  2  | [Search Suggestions System](https://leetcode.com/problems/search-suggestions-system/)                                       | 🟡 Medium  |  18 / 2  | 🔥🔥 #353 |
|  ⬜  |  3  | [Find the Length of the Longest Common Prefix](https://leetcode.com/problems/find-the-length-of-the-longest-common-prefix/) | 🟡 Medium  |  14 / 5  |  🔥 #480  |
|  ⬜  |  4  | [Maximum XOR of Two Numbers in an Array](https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/)             | 🟡 Medium  |  5 / 3   |  🔥 #498  |
|  ⬜  |  5  | [Lexicographical Numbers](https://leetcode.com/problems/lexicographical-numbers/)                                           | 🟡 Medium  |  6 / 2   |  🔥 #810  |
|  ⬜  |  6  | [Remove Sub-Folders from the Filesystem](https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/)             | 🟡 Medium  |  8 / 1   |  🔥 #850  |
|  ⬜  |  7  | [Implement Magic Dictionary](https://leetcode.com/problems/implement-magic-dictionary/)                                     | 🟡 Medium  |  2 / 1   |  🔥 #894  |
|  ⬜  |  8  | [Word Search II](https://leetcode.com/problems/word-search-ii/)                                                             |  🔴 Hard   |  20 / 4  | 🔥🔥 #118 |
|  ⬜  |  9  | [K-th Smallest in Lexicographical Order](https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/)             |  🔴 Hard   |  6 / 0   |  🔥 #652  |
|  ⬜  | 10  | [Concatenated Words](https://leetcode.com/problems/concatenated-words/)                                                     |  🔴 Hard   |  2 / 1   |  🔥 #820  |

### Bit Manipulation

📘 Common · 10 problems

_XOR cancellation, masks, shifting, power-of-two tricks_

| Done |  #  | Problem                                                                         | Difficulty | Asked at |   Rank    |
| :--: | :-: | ------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Single Number](https://leetcode.com/problems/single-number/)                   |  🟢 Easy   |  16 / 6  | 🔥🔥 #117 |
|  ⬜  |  2  | [Power of Two](https://leetcode.com/problems/power-of-two/)                     |  🟢 Easy   |  10 / 4  | 🔥🔥 #226 |
|  ⬜  |  3  | [Number of 1 Bits](https://leetcode.com/problems/number-of-1-bits/)             |  🟢 Easy   |  11 / 2  | 🔥🔥 #296 |
|  ⬜  |  4  | [Reverse Bits](https://leetcode.com/problems/reverse-bits/)                     |  🟢 Easy   |  10 / 2  | 🔥🔥 #304 |
|  ⬜  |  5  | [Find the Difference](https://leetcode.com/problems/find-the-difference/)       |  🟢 Easy   |  6 / 2   |  🔥 #445  |
|  ⬜  |  6  | [Divide Two Integers](https://leetcode.com/problems/divide-two-integers/)       | 🟡 Medium  |  8 / 4   | 🔥🔥 #240 |
|  ⬜  |  7  | [Sum of Two Integers](https://leetcode.com/problems/sum-of-two-integers/)       | 🟡 Medium  |  8 / 4   | 🔥🔥 #381 |
|  ⬜  |  8  | [Single Number II](https://leetcode.com/problems/single-number-ii/)             | 🟡 Medium  |  5 / 4   |  🔥 #460  |
|  ⬜  |  9  | [Cinema Seat Allocation](https://leetcode.com/problems/cinema-seat-allocation/) | 🟡 Medium  |  6 / 2   |  🔥 #487  |
|  ⬜  | 10  | [Single Number III](https://leetcode.com/problems/single-number-iii/)           | 🟡 Medium  |  7 / 2   |  🔥 #539  |

### Math & Number Theory

⭐ Core · 15 problems

_Digits, primes, gcd, fast exponentiation, overflow care_

| Done |  #  | Problem                                                                                             | Difficulty | Asked at |    Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Palindrome Number](https://leetcode.com/problems/palindrome-number/)                               |  🟢 Easy   | 31 / 10  | 🔥🔥🔥 #52 |
|  ⬜  |  2  | [Fizz Buzz](https://leetcode.com/problems/fizz-buzz/)                                               |  🟢 Easy   |  17 / 6  | 🔥🔥 #151  |
|  ⬜  |  3  | [Plus One](https://leetcode.com/problems/plus-one/)                                                 |  🟢 Easy   |  12 / 5  | 🔥🔥 #181  |
|  ⬜  |  4  | [Maximum Product of Three Numbers](https://leetcode.com/problems/maximum-product-of-three-numbers/) |  🟢 Easy   |  12 / 5  | 🔥🔥 #263  |
|  ⬜  |  5  | [Excel Sheet Column Title](https://leetcode.com/problems/excel-sheet-column-title/)                 |  🟢 Easy   |  10 / 3  | 🔥🔥 #300  |
|  ⬜  |  6  | [Excel Sheet Column Number](https://leetcode.com/problems/excel-sheet-column-number/)               |  🟢 Easy   |  8 / 2   | 🔥🔥 #368  |
|  ⬜  |  7  | [Add Digits](https://leetcode.com/problems/add-digits/)                                             |  🟢 Easy   |  10 / 2  | 🔥🔥 #388  |
|  ⬜  |  8  | [Reverse Integer](https://leetcode.com/problems/reverse-integer/)                                   | 🟡 Medium  |  18 / 7  | 🔥🔥🔥 #69 |
|  ⬜  |  9  | [Pow(x, n)](https://leetcode.com/problems/powx-n/)                                                  | 🟡 Medium  |  22 / 5  | 🔥🔥🔥 #94 |
|  ⬜  | 10  | [Count Primes](https://leetcode.com/problems/count-primes/)                                         | 🟡 Medium  |  16 / 5  | 🔥🔥 #152  |
|  ⬜  | 11  | [Angle Between Hands of a Clock](https://leetcode.com/problems/angle-between-hands-of-a-clock/)     | 🟡 Medium  |  10 / 6  | 🔥🔥 #236  |
|  ⬜  | 12  | [Maximum Swap](https://leetcode.com/problems/maximum-swap/)                                         | 🟡 Medium  |  9 / 2   | 🔥🔥 #261  |
|  ⬜  | 13  | [Fraction to Recurring Decimal](https://leetcode.com/problems/fraction-to-recurring-decimal/)       | 🟡 Medium  |  10 / 2  | 🔥🔥 #278  |
|  ⬜  | 14  | [Bulb Switcher](https://leetcode.com/problems/bulb-switcher/)                                       | 🟡 Medium  |  8 / 4   | 🔥🔥 #292  |
|  ⬜  | 15  | [Integer to English Words](https://leetcode.com/problems/integer-to-english-words/)                 |  🔴 Hard   |  22 / 2  | 🔥🔥 #170  |

### Design Problems

⭐ Core · 15 problems

_Combine data structures to hit target complexities (hash map + list, heap, etc.)_

| Done |  #  | Problem                                                                                       | Difficulty | Asked at |    Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Design HashMap](https://leetcode.com/problems/design-hashmap/)                               |  🟢 Easy   |  12 / 5  | 🔥🔥 #162  |
|  ⬜  |  2  | [LRU Cache](https://leetcode.com/problems/lru-cache/)                                         | 🟡 Medium  | 111 / 29 | 🔥🔥🔥 #1  |
|  ⬜  |  3  | [Time Based Key-Value Store](https://leetcode.com/problems/time-based-key-value-store/)       | 🟡 Medium  | 33 / 12  | 🔥🔥🔥 #50 |
|  ⬜  |  4  | [Flatten Nested List Iterator](https://leetcode.com/problems/flatten-nested-list-iterator/)   | 🟡 Medium  |  18 / 4  | 🔥🔥 #136  |
|  ⬜  |  5  | [Snapshot Array](https://leetcode.com/problems/snapshot-array/)                               | 🟡 Medium  |  18 / 4  | 🔥🔥 #179  |
|  ⬜  |  6  | [Design Twitter](https://leetcode.com/problems/design-twitter/)                               | 🟡 Medium  |  13 / 2  | 🔥🔥 #265  |
|  ⬜  |  7  | [Design Browser History](https://leetcode.com/problems/design-browser-history/)               | 🟡 Medium  |  12 / 4  | 🔥🔥 #306  |
|  ⬜  |  8  | [Design Underground System](https://leetcode.com/problems/design-underground-system/)         | 🟡 Medium  |  6 / 2   |  🔥 #443   |
|  ⬜  |  9  | [Design Memory Allocator](https://leetcode.com/problems/design-memory-allocator/)             | 🟡 Medium  |  18 / 5  |  🔥 #450   |
|  ⬜  | 10  | [Product of the Last K Numbers](https://leetcode.com/problems/product-of-the-last-k-numbers/) | 🟡 Medium  |  10 / 1  |  🔥 #453   |
|  ⬜  | 11  | [Design Spreadsheet](https://leetcode.com/problems/design-spreadsheet/)                       | 🟡 Medium  |  6 / 0   |  🔥 #490   |
|  ⬜  | 12  | [Design Authentication Manager](https://leetcode.com/problems/design-authentication-manager/) | 🟡 Medium  |  12 / 6  |  🔥 #512   |
|  ⬜  | 13  | [LFU Cache](https://leetcode.com/problems/lfu-cache/)                                         |  🔴 Hard   |  30 / 7  | 🔥🔥🔥 #58 |
|  ⬜  | 14  | [All O`one Data Structure](https://leetcode.com/problems/all-oone-data-structure/)            |  🔴 Hard   |  12 / 4  | 🔥🔥 #198  |
|  ⬜  | 15  | [Maximum Frequency Stack](https://leetcode.com/problems/maximum-frequency-stack/)             |  🔴 Hard   |  12 / 2  | 🔥🔥 #302  |

### Randomization & Reservoir Sampling

📘 Common · 5 problems

_Uniform random picks from streams, weighted picks, rejection sampling_

| Done |  #  | Problem                                                                                                                           | Difficulty | Asked at |    Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :--------: |
|  ⬜  |  1  | [Random Pick with Weight](https://leetcode.com/problems/random-pick-with-weight/)                                                 | 🟡 Medium  |  24 / 6  | 🔥🔥🔥 #65 |
|  ⬜  |  2  | [Shuffle an Array](https://leetcode.com/problems/shuffle-an-array/)                                                               | 🟡 Medium  |  7 / 2   |  🔥 #631   |
|  ⬜  |  3  | [Linked List Random Node](https://leetcode.com/problems/linked-list-random-node/)                                                 | 🟡 Medium  |  3 / 1   |  🔥 #802   |
|  ⬜  |  4  | [Implement Rand10() Using Rand7()](https://leetcode.com/problems/implement-rand10-using-rand7/)                                   | 🟡 Medium  |  6 / 0   |  🔥 #965   |
|  ⬜  |  5  | [Insert Delete GetRandom O(1) - Duplicates allowed](https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/) |  🔴 Hard   |  12 / 2  | 🔥🔥 #218  |

### Segment Tree & Fenwick Tree

🧗 Occasional · 5 problems

_Range queries with point/range updates, counting inversions_

| Done |  #  | Problem                                                                                                   | Difficulty | Asked at |   Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Range Sum Query - Mutable](https://leetcode.com/problems/range-sum-query-mutable/)                       | 🟡 Medium  |  4 / 0   |  🔥 #792  |
|  ⬜  |  2  | [Reverse Pairs](https://leetcode.com/problems/reverse-pairs/)                                             |  🔴 Hard   |  8 / 6   | 🔥🔥 #312 |
|  ⬜  |  3  | [Count of Smaller Numbers After Self](https://leetcode.com/problems/count-of-smaller-numbers-after-self/) |  🔴 Hard   |  6 / 2   |  🔥 #410  |
|  ⬜  |  4  | [Range Module](https://leetcode.com/problems/range-module/)                                               |  🔴 Hard   |  6 / 1   |  🔥 #873  |
|  ⬜  |  5  | [Count of Range Sum](https://leetcode.com/problems/count-of-range-sum/)                                   |  🔴 Hard   |  5 / 3   |  🔥 #893  |

### String Algorithms (KMP, Rabin-Karp, Z-Function)

📘 Common · 10 problems

_Fast substring matching, borders, rolling hashes_

| Done |  #  | Problem                                                                                                                                       | Difficulty | Asked at |   Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Rotate String](https://leetcode.com/problems/rotate-string/)                                                                                 |  🟢 Easy   |  12 / 6  | 🔥🔥 #183 |
|  ⬜  |  2  | [Repeated Substring Pattern](https://leetcode.com/problems/repeated-substring-pattern/)                                                       |  🟢 Easy   |  8 / 0   |  🔥 #532  |
|  ⬜  |  3  | [Find the Index of the First Occurrence in a String](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/)       |  🟢 Easy   |  16 / 6  |  🔥 #754  |
|  ⬜  |  4  | [Palindromic Substrings](https://leetcode.com/problems/palindromic-substrings/)                                                               | 🟡 Medium  |  24 / 6  | 🔥🔥 #114 |
|  ⬜  |  5  | [Repeated DNA Sequences](https://leetcode.com/problems/repeated-dna-sequences/)                                                               | 🟡 Medium  |  7 / 3   | 🔥🔥 #379 |
|  ⬜  |  6  | [Repeated String Match](https://leetcode.com/problems/repeated-string-match/)                                                                 | 🟡 Medium  |  6 / 2   |  🔥 #581  |
|  ⬜  |  7  | [Check If a String Contains All Binary Codes of Size K](https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/) | 🟡 Medium  |  5 / 2   |  🔥 #717  |
|  ⬜  |  8  | [Shortest Palindrome](https://leetcode.com/problems/shortest-palindrome/)                                                                     |  🔴 Hard   |  8 / 4   | 🔥🔥 #281 |
|  ⬜  |  9  | [Longest Happy Prefix](https://leetcode.com/problems/longest-happy-prefix/)                                                                   |  🔴 Hard   |  6 / 2   |  🔥 #666  |
|  ⬜  | 10  | [Longest Duplicate Substring](https://leetcode.com/problems/longest-duplicate-substring/)                                                     |  🔴 Hard   |  7 / 0   |  🔥 #895  |

### Geometry

🧗 Occasional · 7 problems

_Rare; mostly arithmetic on points, slopes, and rectangles_

| Done |  #  | Problem                                                                                             | Difficulty | Asked at |   Rank    |
| :--: | :-: | --------------------------------------------------------------------------------------------------- | :--------: | :------: | :-------: |
|  ⬜  |  1  | [Rectangle Overlap](https://leetcode.com/problems/rectangle-overlap/)                               |  🟢 Easy   |  5 / 2   |  🔥 #440  |
|  ⬜  |  2  | [Minimum Time Visiting All Points](https://leetcode.com/problems/minimum-time-visiting-all-points/) |  🟢 Easy   |  6 / 0   |  🔥 #727  |
|  ⬜  |  3  | [Rectangle Area](https://leetcode.com/problems/rectangle-area/)                                     | 🟡 Medium  |  8 / 3   | 🔥🔥 #390 |
|  ⬜  |  4  | [Valid Square](https://leetcode.com/problems/valid-square/)                                         | 🟡 Medium  |  4 / 1   |  🔥 #509  |
|  ⬜  |  5  | [Minimum Area Rectangle](https://leetcode.com/problems/minimum-area-rectangle/)                     | 🟡 Medium  |  8 / 1   |  🔥 #811  |
|  ⬜  |  6  | [Max Points on a Line](https://leetcode.com/problems/max-points-on-a-line/)                         |  🔴 Hard   |  16 / 4  | 🔥🔥 #199 |
|  ⬜  |  7  | [Maximum Number of Visible Points](https://leetcode.com/problems/maximum-number-of-visible-points/) |  🔴 Hard   |  6 / 2   |  🔥 #998  |

---
