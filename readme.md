# Homework 4: Rust Homework

This is an individual assignment. You must work on this homework alone.

---

## Introduction

This homework introduces you to fundamental **Rust** programming concepts, including:

- basic data types (`u32`, `i32`, `String`, `bool`)
- vectors (`Vec<T>`)
- hash maps (`HashMap<K, V>`)
- defining structs and methods using `impl`
- Rust control flow: loops, branching, pattern matching
- writing and running Rust tests

You will complete two components:

1. A set of warm-up functions (Part 1)
2. A `PhoneBook` data structure (Part 2)

All code should be placed in `src/lib.rs` file.

## Testing

Public tests are provided in: `tests/public.rs`

Run them with:
`cargo test --test public`


You may add additional tests in:
`tests/student.rs`


and run them using:
`cargo test --test student`


## Part 1 — Warm-up Functions

### 1. `fib(n)`
Return a vector containing the first n Fibonacci numbers.

Rust signature:
``` rust
pub fn fib(n: u32) -> Vec<u32>
```

Examples:
``` rust
fib(0) → []
fib(1) → [0]
fib(2) → [0, 1]
fib(3) → [0, 1, 1]
fib(10) → [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
```

Assume n ≥ 0.

### 2. `is_palindrome(n)`

Goal:
Return true if the number reads the same forward and backward; otherwise return false.

Rust signature:

``` rust
pub fn is_palindrome(n: u32) -> bool
```

You may convert the integer to a string for convenience.

Assume:
- `n` is non-negative
- `n has no leading zeros

### 3. `nthmax(n, a)`

Goal:
Return the n-th largest value in the array a, or None if it does not exist.
The largest value corresponds to n = 0.

Rust signature:
```rust
pub fn nthmax(n: usize, a: &[i32]) -> Option<i32>
```
Examples:
```
nthmax(0, [1,2,3,0]) → Some(3)
nthmax(1, [3,2,1,0]) → Some(2)
nthmax(2, [7,3,4,5]) → Some(4)
nthmax(5, [1,2,3])   → None
```

Assume `n` ≥ 0.

### 4. `freq(s)`

Goal:
Return the character that appears most frequently in the string s.
If s is empty, return the empty string.

Rust signature:
```rust
pub fn freq(s: &str) -> String
```

Assume there are no ties for most frequent character.

### 5. `zip_hash(arr1, arr2)`

Goal:
Return a hash map where arr1[i] is mapped to arr2[i].
If the vectors differ in length, return None.

Rust signature:
```rust
pub fn zip_hash(
    keys: &[String],
    values: &[String]
) -> Option<std::collections::HashMap<String, String>>
```

Examples:
```rust
zip_hash([], []) → Some({})
zip_hash(["a"], ["b"]) → Some({"a" -> "b"})
zip_hash(["a"], ["b", "c"]) → None
```
### 6. `hash_to_array(map)`

Goal:
Return a vector of (key, value) pairs from the hash map.
Pairs must be ordered by sorted key order (or the order required by tests).

Rust signature:
```rust
pub fn hash_to_array(
    map: &std::collections::HashMap<String, String>
) -> Vec<(String, String)>
```
## Part 2 - PhoneBook Struct

You will implement a PhoneBook type using a Rust struct and impl block.

A phone book records:
- a person's name
- their phone number (NNN-NNN-NNNN)
- whether the entry is listed or unlisted

You may choose any internal representation (e.g., one or multiple hash maps).

### `add(name, number, is_listed)`

Goal:
Add an entry to the phone book.

Rust signature:
``` rust
pub fn add(&mut self, name: String, number: String, is_listed: bool) -> bool
```

Rules:
- If the name already exists, the add fails.
- If the phone number is not in NNN-NNN-NNNN format, the add fails.
- A number can be added as unlisted any number of times.
- A number can be added as listed only once.
- If a number already exists as listed, it may only be added again as unlisted.

Return `true` if added successfully; otherwise `false`.

### `lookup(name)`

Goal:
Return the phone number for name only if the entry is listed; otherwise return `None`.

Signature:
```rust
pub fn lookup(&self, name: &str) -> Option<String>
```

### `lookup_by_num(number)`

Goal:
Return the name associated with the phone number only if the entry is listed.

Signature:
```rust
pub fn lookup_by_num(&self, number: &str) -> Option<String>
```
### `names_by_ac(areacode)`
Goal:
Return all names whose phone numbers begin with areacode.
This includes listed and unlisted entries.

Signature:
``` rust
pub fn names_by_ac(&self, areacode: &str) -> Vec<String>
```

The returned vector does not need a specific sort order unless required by tests.

Good luck, and enjoy learning Rust!
