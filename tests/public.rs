// tests/gradescope.rs
//
// 10-point Gradescope tester.
// Each #[test] here is worth 1 point.
// If any assertion in a test fails, the student loses that point.

use std::collections::HashMap;

use homework4::*;

/// Point 1/10: fib – basic and larger cases.
#[test]
fn point1_fib() {
    assert_eq!(fib(0), vec![]);
    assert_eq!(fib(1), vec![0]);
    assert_eq!(fib(2), vec![0, 1]);
    assert_eq!(fib(3), vec![0, 1, 1]);
    assert_eq!(
        fib(10),
        vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
    );

    // Quick sanity: length matches n
    let n = 15;
    let res = fib(n);
    assert_eq!(res.len(), n as usize);
}

/// Point 2/10: is_palindrome – true and false, small and larger.
#[test]
fn point2_is_palindrome() {
    // Trivial palindromes
    assert!(is_palindrome(0));
    assert!(is_palindrome(1));
    assert!(is_palindrome(9));

    // Non-palindromes
    assert!(!is_palindrome(10));
    assert!(!is_palindrome(120210));

    // Mixed cases
    assert!(is_palindrome(11));
    assert!(is_palindrome(121));
    assert!(is_palindrome(10101));
    assert!(!is_palindrome(123431));
}

/// Point 3/10: nthmax – basic, duplicates, and edge cases.
#[test]
fn point3_nthmax() {
    // Basic examples from spec
    assert_eq!(nthmax(0, &[1, 2, 3, 0]), Some(3));
    assert_eq!(nthmax(1, &[3, 2, 1, 0]), Some(2));
    assert_eq!(nthmax(2, &[7, 3, 4, 5]), Some(4));
    assert_eq!(nthmax(5, &[1, 2, 3]), None);

    // Duplicates: treated as separate positions in the sorted list.
    // Sorted descending: [5, 5, 2, 1]
    let arr = [5, 1, 5, 2];
    assert_eq!(nthmax(0, &arr), Some(5));
    assert_eq!(nthmax(1, &arr), Some(5));
    assert_eq!(nthmax(2, &arr), Some(2));
    assert_eq!(nthmax(3, &arr), Some(1));
    assert_eq!(nthmax(4, &arr), None);

    // Empty
    let empty: [i32; 0] = [];
    assert_eq!(nthmax(0, &empty), None);
}

/// Point 4/10: freq – empty, given examples, and some extra cases.
#[test]
fn point4_freq() {
    // Empty string
    assert_eq!(freq(""), "");

    // Examples from spec
    assert_eq!(freq("aaabb"), "a");
    assert_eq!(freq("bbaaa"), "a");
    assert_eq!(freq("ssabcd"), "s");
    assert_eq!(freq("a12xxxxxyyyxyxyxy"), "x");

    // Extra: single char, clear winner
    assert_eq!(freq("z"), "z");
    assert_eq!(freq("abbcccdddde"), "d"); // d appears 4 times, others less
}

/// Point 5/10: zip_hash – lengths, contents, and mismatch behavior.
#[test]
fn point5_zip_hash() {
    // Both empty
    let v1: Vec<String> = vec![];
    let v2: Vec<String> = vec![];
    let m = zip_hash(&v1, &v2).unwrap();
    assert!(m.is_empty());

    // Single pair
    let v1 = vec!["1".to_string()];
    let v2 = vec!["2".to_string()];
    let m = zip_hash(&v1, &v2).unwrap();
    assert_eq!(m.get("1"), Some(&"2".to_string()));

    // Multiple pairs
    let v1 = vec!["1".into(), "5".into()];
    let v2 = vec!["2".into(), "4".into()];
    let m = zip_hash(&v1, &v2).unwrap();
    assert_eq!(m.len(), 2);
    assert_eq!(m.get("1"), Some(&"2".to_string()));
    assert_eq!(m.get("5"), Some(&"4".to_string()));

    // Mismatched lengths
    let v1 = vec!["1".into()];
    let v2 = vec!["2".into(), "3".into()];
    assert!(zip_hash(&v1, &v2).is_none());

    let v3: Vec<String> = vec![];
    let v4 = vec!["x".into()];
    assert!(zip_hash(&v3, &v4).is_none());

    // Names example
    let a1 = vec!["Umar".into(), "Justin".into(), "Yuhong".into()];
    let a2 = vec!["prof".into(), "TA".into(), "TA".into()];
    let m = zip_hash(&a1, &a2).unwrap();

    assert_eq!(m.get("Umar"), Some(&"prof".to_string()));
    assert_eq!(m.get("Justin"), Some(&"TA".to_string()));
    assert_eq!(m.get("Yuhong"), Some(&"TA".to_string()));
}

/// Point 6/10: hash_to_array – empty, single, multiple (sorted by key).
#[test]
fn point6_hash_to_array() {
    // Empty map
    let m: HashMap<String, String> = HashMap::new();
    let v = hash_to_array(&m);
    assert!(v.is_empty());

    // Single key/value
    let mut m1 = HashMap::new();
    m1.insert("a".to_string(), "b".to_string());
    let v1 = hash_to_array(&m1);
    assert_eq!(v1, vec![("a".to_string(), "b".to_string())]);

    // Multiple keys, expecting sorted-by-key ordering as defined in implementation.
    let mut m2 = HashMap::new();
    m2.insert("x".to_string(), "v".to_string());
    m2.insert("z".to_string(), "u".to_string());
    m2.insert("y".to_string(), "w".to_string());

    let v2 = hash_to_array(&m2);
    assert_eq!(
        v2,
        vec![
            ("x".to_string(), "v".to_string()),
            ("y".to_string(), "w".to_string()),
            ("z".to_string(), "u".to_string()),
        ]
    );
}

/// Point 7/10: PhoneBook::add – name uniqueness, number format, listed/unlisted rules.
#[test]
fn point7_phonebook_add() {
    let mut pb = PhoneBook::new();

    // Basic additions
    assert_eq!(pb.add("John".into(), "110-192-1862".into(), false), true);
    assert_eq!(pb.add("Jane".into(), "220-134-1312".into(), false), true);

    // Duplicate name not allowed
    assert_eq!(pb.add("John".into(), "000-000-0000".into(), false), false);

    // Invalid formats
    assert_eq!(pb.add("Ravi".into(), "110".into(), true), false);
    assert_eq!(pb.add("Bob".into(), "11-000-0000".into(), true), false);
    assert_eq!(pb.add("Alice".into(), "1111-111-1111".into(), true), false);
    assert_eq!(pb.add("Eve".into(), "111-1111-111".into(), true), false);

    // Listed/unlisted rules
    let mut pb2 = PhoneBook::new();
    assert_eq!(pb2.add("Alice".into(), "012-345-6789".into(), false), true);
    assert_eq!(pb2.add("Bob".into(), "012-345-6789".into(), false), true);
    assert_eq!(pb2.add("Eve".into(), "012-345-6789".into(), true), true);
    // Another listed with same number -> should fail
    assert_eq!(pb2.add("Rob".into(), "012-345-6789".into(), true), false);
    // But unlisted with same number still allowed
    assert_eq!(
        pb2.add("Johnny B. Good".into(), "012-345-6789".into(), false),
        true
    );
}

/// Point 8/10: PhoneBook::lookup – listed-only behavior.
#[test]
fn point8_phonebook_lookup() {
    let mut pb = PhoneBook::new();
    assert_eq!(pb.add("John".into(), "110-192-1862".into(), false), true);
    assert_eq!(pb.add("Jane".into(), "220-134-1312".into(), true), true);
    assert_eq!(pb.add("Jack".into(), "114-192-1862".into(), false), true);
    assert_eq!(pb.add("Jessie".into(), "410-124-1131".into(), true), true);
    assert_eq!(pb.add("Ravi".into(), "110".into(), true), false);

    // Unlisted entries should return None
    assert_eq!(pb.lookup("John"), None);
    assert_eq!(pb.lookup("Jack"), None);

    // Listed entries should return their numbers
    assert_eq!(pb.lookup("Jane"), Some("220-134-1312".into()));
    assert_eq!(pb.lookup("Jessie"), Some("410-124-1131".into()));

    // Non-existent
    assert_eq!(pb.lookup("Ravi"), None);
    assert_eq!(pb.lookup("DoesNotExist"), None);
}

/// Point 9/10: PhoneBook::lookup_by_num – listed-only reverse lookup.
#[test]
fn point9_phonebook_lookup_by_num() {
    let mut pb = PhoneBook::new();
    assert_eq!(pb.add("John".into(), "110-192-1862".into(), false), true);
    assert_eq!(pb.add("Jane".into(), "220-134-1312".into(), true), true);
    assert_eq!(pb.add("Jack".into(), "114-192-1862".into(), false), true);
    assert_eq!(pb.add("Jessie".into(), "410-124-1131".into(), true), true);

    // Unlisted -> None
    assert_eq!(pb.lookup_by_num("110-192-1862"), None);
    assert_eq!(pb.lookup_by_num("114-192-1862"), None);

    // Listed -> Some(name)
    assert_eq!(pb.lookup_by_num("220-134-1312"), Some("Jane".into()));
    assert_eq!(pb.lookup_by_num("410-124-1131"), Some("Jessie".into()));

    // Non-existent
    assert_eq!(pb.lookup_by_num("999-999-9999"), None);
}

/// Point 10/10: PhoneBook::names_by_ac – includes unlisted, filters by area code.
#[test]
fn point10_phonebook_names_by_ac() {
    let mut pb = PhoneBook::new();
    assert_eq!(pb.add("John".into(), "110-192-1862".into(), false), true);
    assert_eq!(pb.add("Jane".into(), "220-134-1312".into(), true), true);
    assert_eq!(pb.add("Jack".into(), "114-192-1862".into(), false), true);
    assert_eq!(pb.add("Jessie".into(), "110-124-1131".into(), true), true);

    let mut n110 = pb.names_by_ac("110");
    n110.sort();
    assert_eq!(n110, vec!["Jessie".to_string(), "John".to_string()]);

    let mut n114 = pb.names_by_ac("114");
    n114.sort();
    assert_eq!(n114, vec!["Jack".to_string()]);

    let n111 = pb.names_by_ac("111");
    assert!(n111.is_empty());

    // Also check empty phonebook case
    let pb2 = PhoneBook::new();
    let n = pb2.names_by_ac("123");
    assert!(n.is_empty());
}
