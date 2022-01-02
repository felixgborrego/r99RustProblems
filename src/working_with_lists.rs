//P01 (*) Find the last element of a list.
// Example:
//   last(List(1, 1, 2, 3, 5, 8))
//   > 8
pub fn p1_last<T>(list: &[T]) -> Option<&T> {
    list.last()
}

pub fn p1_last_with_index<T>(list: &[T]) -> Option<&T> {
    list.get(list.len() - 1)
}

// P02 (*) Find the last but one element of a list.
// Example:
//   penultimate(List(1, 1, 2, 3, 5, 8))
//   > 5
pub fn p2_penultimate<T>(list: &[T]) -> Option<&T> {
    let len = list.len();
    if len > 1 {
        list.get(list.len() - 2)
    } else {
        None
    }
}

// P03 (*) Find the Kth element of a list.
// By convention, the first element in the list is element 0.
//
// Example:
//  nth(2, List(1, 1, 2, 3, 5, 8))
//  > Int = 2
pub fn p3_nth_index<T>(nth: usize, list: &[T]) -> Option<&T> {
    list.get(nth)
}

pub fn p3_nth_iter<T>(nth: usize, list: &[T]) -> Option<&T> {
    list.get(nth)
}

// P04 (*) Find the number of elements of a list.
// Example:
//  length(List(1, 1, 2, 3, 5, 8))
//  > Int = 6
pub fn p4_length<T>(list: &[T]) -> usize {
    list.len()
}

// P05 (*) Reverse a list.
// Example:
//  reverse(List(1, 1, 2, 3, 5, 8))
//  > List(8, 5, 3, 2, 1, 1)
pub fn p5_reverse<T: Copy>(list: &[T]) -> Vec<T> {
    let mut list = list.to_owned();
    list.reverse();
    list
}

// P06 (*) Find out whether a list is a palindrome.
// Example:
//  isPalindrome(List(1, 2, 3, 2, 1))
//  > true
pub fn p6_is_palindrome<T: PartialEq>(list: &[T]) -> bool {
    let mid = list.len() / 2;
    let (half1, half2) = list.split_at(mid);

    for i in 0..mid {
        if half1.get(i) != half2.get(half2.len() - 1 - i) {
            return false;
        }
    }

    true
}

// P07 (**) Flatten a nested list structure.
// Transform a list, possibly holding lists as elements into a `flat' list
// by replacing each list with its elements (recursively).
// Example:
// * my_flatten(a (b (c d) e)))
// (A B C D E)
// Hint: Use the predefined functions list and append.
pub fn p7_flatten(list: &[Vec<u32>]) -> Vec<u32> {
    let mut buffer = vec![];

    for sub_vec in list {
        for x in sub_vec {
            buffer.push(*x);
        }
    }

    buffer
}

pub fn p7_flatten2(list: &[Vec<u32>]) -> Vec<u32> {
    list.iter().flatten().copied().collect::<Vec<_>>()
}

// P08 (**) Eliminate consecutive duplicates of list elements.
// If a list contains repeated elements they should be replaced with a single copy of the element.
// The order of the elements should not be changed.
// Example:
// * (compress '(a a a a b c c a a d e e e e))
//  (A B C A D E)
pub fn p8_compress<T: PartialEq + Copy>(list: &[T]) -> Vec<T> {
    let mut previous: Option<&T> = None;

    list.iter()
        .filter_map(|x| {
            if Some(x) != previous {
                previous = Some(x);
                Some(*x)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

pub fn p8_compress2(list: &[u32]) -> Vec<u32> {
    let mut previous: Option<u32> = None;

    list.iter()
        .filter_map(|x| {
            if Some(*x) != previous {
                previous = Some(*x);
                Some(*x)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

pub fn p8_compress3(list: &[u32]) -> Vec<u32> {
    let mut previous = None;
    let mut buffer = vec![];
    for element in list {
        if Some(element) != previous {
            buffer.push(*element);
        }
        previous = Some(element);
    }

    buffer
}

struct Acc<T> {
    current_duplicates: Vec<T>,
    result: Vec<Vec<T>>,
}

// P09 (**) Pack consecutive duplicates of list elements into sublists.
// If a list contains repeated elements they should be placed in separate sublists.
// Example:
// * (pack '(a a a a b c c a a d e e e e))
// ((A A A A) (B) (C C) (A A) (D) (E E E E))
pub fn p9_pack<T: Copy + Eq>(list: &[T]) -> Vec<Vec<T>> {
    let mut acc: Acc<T> = Acc {
        current_duplicates: vec![],
        result: vec![],
    };

    fn accumulate<'a, T: Copy + Eq>(acc: &'a mut Acc<T>, e: &T) -> &'a mut Acc<T> {
        if acc.current_duplicates.is_empty() || acc.current_duplicates.first().unwrap().eq(e) {
            acc.current_duplicates.push(*e);
        } else {
            acc.result.push((*acc.current_duplicates).to_owned());
            acc.current_duplicates = vec![];
            acc.current_duplicates.push(*e);
        }

        acc
    }

    let result = list.iter().fold(&mut acc, accumulate);
    if !result.current_duplicates.is_empty() {
        result.result.push((*result.current_duplicates).to_owned());
    }

    (*result.result).to_owned()
}

// P10 (*) Run-length encoding of a list.
// Use the result of problem P09 to implement the so-called run-length encoding data compression method.
// Consecutive duplicates of elements are encoded as lists (N E) where N is the number of duplicates of the element E.
//
// Example:
// * (encode '(a a a a b c c a a d e e e e))
// ((4 A) (1 B) (2 C) (2 A) (1 D)(4 E))
pub fn p10_run_length<T: Copy + Eq>(list: &[T]) -> Vec<(usize, T)> {
    let packed = p9_pack(list);

    packed
        .into_iter()
        .map(|l| (l.len(), *l.first().unwrap()))
        .collect()
}

// P12 (**) Decode a run-length encoded list.
// Given a run-length code list generated as specified in problem P10. Construct its uncompressed version.
pub fn p12_decode(list: &[(usize, char)]) -> Vec<char> {
    let mut buffer = vec![];
    for (size, c) in list {
        buffer.push(vec![*c; *size])
    }

    buffer.into_iter().flatten().collect()
}


//P13 (**) Run-length encoding of a list (direct solution).
// Implement the so-called run-length encoding data compression method directly. I.e. don't explicitly create the sublists containing the duplicates, as in problem P09, but only count them. As in problem P11, simplify the result list by replacing the singleton lists (1 X) by X.

// Example:
// * (encode-direct '(a a a a b c c a a d e e e e))
// ((4 A) B (2 C) (2 A) D (4 E))
pub fn p13_encode_direct(list: &[char]) -> Vec<(usize, char)> {
    struct AccDirect {
        current_duplicate: Option<char>,
        current_count: usize,
        result: Vec<(usize, char)>,
    }

    let mut acc = AccDirect {
        current_duplicate: None,
        current_count: 0,
        result: vec![],
    };

    fn accumulate<'a>(acc: &'a mut AccDirect, c: &char) -> &'a mut AccDirect {
        if acc.current_duplicate == Some(*c) {
            acc.current_count += 1;
        } else {
            if acc.current_duplicate.is_some() {
                acc.result.push((acc.current_count, acc.current_duplicate.unwrap()));
            }
            acc.current_duplicate = Some(*c);
            acc.current_count = 1;
        }

        acc
    }

    let acc = list.iter().fold(&mut acc, accumulate);
    let mut results = acc.result.to_owned();

    if acc.current_duplicate.is_some() {
        results.push((acc.current_count, acc.current_duplicate.unwrap()));
    }

    results
}

// P14 (*) Duplicate the elements of a list.
// Example:
// p14_duplicate(List('a, 'b, 'c, 'c, 'd))
// Result:  List('a, 'a, 'b, 'b, 'c, 'c, 'c, 'c, 'd, 'd)
pub fn p14_duplicate(list: &[char]) -> Vec<char> {
    list.iter().flat_map(|element| [element, element])
        .copied().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_last() {
        let li = vec![1, 1, 2, 3, 5];
        assert_eq!(p1_last(&li), Some(&5));
    }

    #[test]
    fn test_p1_last_with_index() {
        let li = vec![1, 1, 2, 3, 5];
        assert_eq!(p1_last_with_index(&li), Some(&5));
    }

    #[test]
    fn test_p1_empty() {
        let li: Vec<i32> = vec![];
        assert_eq!(p1_last(&li), None);
    }

    #[test]
    fn test_p2_penultimate() {
        let li = vec![1, 1, 2, 3, 5];
        assert_eq!(p2_penultimate(&li), Some(&3));

        let li: Vec<i32> = vec![];
        assert_eq!(p2_penultimate(&li), None);
    }

    #[test]
    fn test_p3_nth_index() {
        let li = vec![1, 1, 2, 3, 5];
        assert_eq!(p3_nth_index(4, &li), Some(&5));

        let li: Vec<i32> = vec![];
        assert_eq!(p3_nth_index(5, &li), None);
    }

    #[test]
    fn test_p3_nth_iter() {
        let li = vec![1, 1, 2, 3, 5];
        assert_eq!(p3_nth_iter(4, &li), Some(&5));

        let li: Vec<i32> = vec![];
        assert_eq!(p3_nth_iter(5, &li), None);
    }

    #[test]
    fn test_p4_length() {
        let li = vec![1, 1, 2, 3, 5];
        assert_eq!(p4_length(&li), 5);
    }

    #[test]
    fn test_p5_reverse() {
        let li = vec![1, 1, 2, 3, 5];
        assert_eq!(p5_reverse(&li), vec![5, 3, 2, 1, 1]);
    }

    #[test]
    fn test_p6_is_palindrome() {
        let li = vec![1, 1, 2, 3, 5];
        assert!(!p6_is_palindrome(&li));

        let li = vec![1, 2, 3, 2, 1];
        assert!(p6_is_palindrome(&li));

        let li = vec![1, 2, 3, 3, 2, 1];
        assert!(p6_is_palindrome(&li));
    }

    #[test]
    fn test_p7() {
        let li = vec![vec![1, 2], vec![3, 4, 5]];
        assert_eq!(p7_flatten(&li), vec![1, 2, 3, 4, 5]);

        assert_eq!(p7_flatten2(&li), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_p8_compress() {
        let li = vec![1, 1, 1, 2, 2, 3, 3];
        assert_eq!(p8_compress(&li), vec![1, 2, 3]);
        assert_eq!(p8_compress2(&li), vec![1, 2, 3]);
        assert_eq!(p8_compress3(&li), vec![1, 2, 3]);
    }

    #[test]
    fn test_p9_pack() {
        // * (pack '(a a a a b c c a a d e e e e))
        // ((A A A A) (B) (C C) (A A) (D) (E E E E))
        let li = vec![
            'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
        ];
        assert_eq!(
            p9_pack(&li),
            vec![
                vec!['a', 'a', 'a', 'a'],
                vec!['b'],
                vec!['c', 'c'],
                vec!['a', 'a'],
                vec!['d'],
                vec!['e', 'e', 'e', 'e'],
            ]
        );
    }

    #[test]
    fn test_p10_and_p12() {
        // * (encode '(a a a a b c c a a d e e e e))
        // ((4 A) (1 B) (2 C) (2 A) (1 D)(4 E))
        let li = vec![
            'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
        ];
        let encoded = p10_run_length(&li);
        assert_eq!(
            encoded,
            vec![(4, 'a'), (1, 'b'), (2, 'c'), (2, 'a'), (1, 'd'), (4, 'e')]
        );

        let decoded = p12_decode(&encoded);
        assert_eq!(decoded, li);
    }

    #[test]
    fn test_p13_encode_direct() {
        let li = vec![
            'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
        ];

        let encoded = p13_encode_direct(&li);
        assert_eq!(
            encoded,
            vec![(4, 'a'), (1, 'b'), (2, 'c'), (2, 'a'), (1, 'd'), (4, 'e')]
        );
    }

    #[test]
    fn test_p14_duplicate() {
        let li = vec!['a', 'b', 'c', 'c', 'd', ];
        let duplicated = p14_duplicate(&li);
        assert_eq!(duplicated, ['a', 'a', 'b','b', 'c', 'c','c','c', 'd', 'd'])
    }
}
