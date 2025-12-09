use std::collections::HashMap;

/// Returns the first `n` Fibonacci numbers.
pub fn fib(_n: u32) -> Vec<u32> {
    let mut x = Vec::new();
    if _n == 0 {
        return x;
    }
    x.push(0);
    if _n == 1 {
        return x;
    }
    x.push(1);
    for _ in 2.._n {
        let len = x.len();
        let next = x[len-1] + x[len-2];
        x.push(next);
    }
    x
}

/// Returns true if `n` is a palindrome, false otherwise.
pub fn is_palindrome(n: u32) -> bool {
    let s = n.to_string(); s == s.chars().rev().collect::<String>()
}

/// Returns the nth largest element in `a`, or None if it does not exist.
pub fn nthmax(_n: usize, _a: &[i32]) -> Option<i32> {
   if _a.is_empty() || _n >= _a.len() {
    return None;
   }
   let mut sorted = _a.to_vec();
   sorted.sort_by(|x,y| y.cmp(x));
   Some(sorted[_n])
}

/// Returns a one-character String containing the most frequent character in `s`.
pub fn freq(s: &str) -> String {
    if s.is_empty() {return String::new();}
    let mut map = HashMap::new();
    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    let (max, _) = map.iter().max_by_key(|&(_, count)| count).unwrap();
    max.to_string()
}

/// Zips two slices into a HashMap, mapping arr1[i] -> arr2[i].
pub fn zip_hash(
    _arr1: &[String],
    _arr2: &[String],
) -> Option<HashMap<String, String>> {
    if _arr1.len() != _arr2.len() {
        return None;
    }
    let mut map = HashMap::new();
    for (key, value) in _arr1.iter().zip(_arr2.iter()) {
        map.insert(key.clone(), value.clone());
    }
    Some(map)
}

/// Converts a HashMap into a Vec of (key, value) pairs.
pub fn hash_to_array(
    _map: &HashMap<String, String>,
) -> Vec<(String, String)> {
    // TODO: implement
    let mut result: Vec<(String, String)> = _map
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

// ========================
// Part 2: PhoneBook
// ========================

/// A single phone book entry.
#[derive(Debug, Clone)]
pub struct PhoneEntry {
    pub name: String,
    pub number: String,
    pub is_listed: bool,
}

/// PhoneBook holds name/number pairs and whether each is listed.
#[derive(Debug, Default)]
pub struct PhoneBook {
    // You are free to change this internal representation if you want.
    pub entries: Vec<PhoneEntry>,
}

impl PhoneBook {
    /// Constructor: create an empty PhoneBook.
    pub fn new() -> Self {
        // You may also use `Self::default()`
        PhoneBook {
            entries: Vec::new(),
        }
    }

    /// Attempts to add a new entry.
    ///
    /// Rules:
    /// 1. If the name already exists, return false.
    /// 2. If the number is not in the format NNN-NNN-NNNN, return false.
    /// 3. A number can be unlisted any number of times, but listed at most once.
    ///    - If the number already exists as listed, adding another listed entry
    ///      with the same number must return false.
    ///
    /// Returns true if the entry was successfully added.

    fn is_valid_number(number: &str) -> bool {
        if number.len() != 12 {
            return false;
        }
        let parts: Vec<&str> = number.split('-').collect();
        if parts.len() != 3 {return false;}
        if parts[0].len() != 3 || parts[1].len() != 3 || parts[2].len() != 4 {return false;}
        parts.iter().all(|part| part.chars().all(|c| c.is_ascii_digit()))
    }
    
    pub fn add(
        &mut self,
        name: String,
        number: String,
        is_listed: bool,
    ) -> bool {
        if self.entries.iter().any(|e| e.name == name) {return false;}
        if !Self::is_valid_number(&number) {return false;}
        if is_listed {
            if self.entries.iter().any(|e| e.number == number && e.is_listed) {
                return false;
            }
        }
        self.entries.push(PhoneEntry {
            name,
            number,
            is_listed,
        });
        
        true
    }

    /// Looks up `name` and returns the number ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup(&self, name: &str) -> Option<String> {
        self.entries
            .iter()
            .find(|e| e.name == name && e.is_listed)
            .map(|e| e.number.clone())
    }

    /// Looks up `num` and returns the associated name ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup_by_num(&self, num: &str) -> Option<String> {
        self.entries
            .iter()
            .find(|e| e.number == num && e.is_listed)
            .map(|e| e.name.clone())
    }

    /// Returns all names (listed and unlisted) whose numbers begin with `areacode`.
    pub fn names_by_ac(&self, areacode: &str) -> Vec<String> {
        self.entries
            .iter()
            .filter(|e| e.number.starts_with(areacode))
            .map(|e| e.name.clone())
            .collect()
    }
}