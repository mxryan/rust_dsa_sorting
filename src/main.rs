mod b_rand;

fn main() {
    println!("Hello, world!");
}

// n^2 time complexity
fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    for _ in 0..v.len() {
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
            }
        }
    }
}

// still n^2 complexity despite the shrinking size of the array wtih successive
// passes
fn bubble_sort_optimized<T: PartialOrd>(v: &mut [T]) {
    for p in 0..v.len() {
        // if we dont swap during a whole pass, then the array must be sorted
        let mut sorted = true;
        // every pass of bubble sort will 'catch' the largest unsorted num and
        // bring it to its proper position towards the end of the array. so with
        // every pass we know that 1 more element at the end of the array is
        // sorted properly, so we can stop traversing the array at that point.
        // the point at which stop traversing the array inches 1 spot forward
        // in the array with every pass.
        for i in 0..v.len() - 1 - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false
            }
        }
        if sorted {
            return;
        }
    }
}

// O(n + log(n))
fn merge_sort<T: PartialOrd>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }
    let mut res = Vec::with_capacity(v.len());
    let second_half = v.split_off(v.len() / 2); // v now is the first half of arr
    let a = merge_sort(v);
    let b = merge_sort(second_half);
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();
    loop {
        match a_peek {
            Some(ref a_val) => {
                match b_peek {
                    // we still have values in a and b and therefore need to
                    // compare them in order to determine which is lowest and
                    // gets put into our our result array next
                    Some(ref b_val) => {
                        if b_val < a_val {
                            res.push(b_peek.take().unwrap());
                            b_peek = b_it.next();
                        } else {
                            res.push(a_peek.take().unwrap());
                            a_peek = a_it.next();
                        }
                    }
                    // we have at least 1 value in A still but none in B so
                    // we can take whatever is left in A (plus our 'peek') and
                    // slap it into the result array
                    None => {
                        res.push(a_peek.take().unwrap());
                        res.extend(a_it);
                        return res;
                    }
                }
            }
            None => {
                if let Some(b_val) = b_peek {
                    // nothing in A but we do have stuff in B so we put the rest
                    // of B into result and return it
                    res.push(b_val);
                    res.extend(b_it);
                    return res;
                }
                // i dont think this line is actually reachable but just in case
                // to prevent an infinite loop we might as well return
                return res;
            }
        }
    }
}

// O(n + log(n))
// fn merge_sort_my_merge_impl<T: PartialOrd>(mut v: Vec<T>) -> Vec<T> {
//     if v.len() <= 1 {
//         return v;
//     }
//     let mut res = Vec::with_capacity(v.len());
//     let second_half = v.split_off(v.len() / 2); // v now is the first half of arr
//     let a = merge_sort(v);
//     let b = merge_sort(second_half);
//
//     let mut a_it = a.into_iter();
//     let mut b_it = b.into_iter();
//     let mut a_peek = a_it.next();
//     let mut b_peek = b_it.next();
//
//     loop {
//         match (a_peek, b_peek) {
//             (Some(ref a_val), Some(ref b_val)) => {
//                 if b_val < a_val {
//                     res.push(b_peek.take().unwrap());
//                     b_peek = b_it.next();
//                 } else {
//                     res.push(a_peek.take().unwrap());
//                     a_peek = a_it.next();
//                 }
//             },
//             (Some(ref a_val), None) => {
//                 res.push(a_peek.take().unwrap());
//                 res.extend(a_it);
//                 return res;
//             },
//             (None, Some(ref b_val)) => {
//                 res.push(b_peek.take().unwrap());
//                 res.extend(b_it);
//                 return res;
//             },
//             (None, None) => {
//                 return res;
//             }
//         }
//     }
// }

// pivot is part of quicksort
// moves the first element to the correct place and puts everything less
// than that element before it and everything greater than that element after
// it.
// then returns the location
fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut pivot = 0;
    // i is always in front of the pivot
    // [4,2,3,6,1,9,10]
    //  p i                         i and p+1 swap does nothing here but the p,p+1 swap moves the 2 and 4
    // [2,4,3,6,1,9,10]
    //    p i                       i and p+1 swap does nothing here but the p,p+1 swap moves the
    // [2,3,4,6,1,9,10]
    //      p i                     no swapping here b/c the pivot is < i (also means no pivot increment)
    // [2,3,4,6,1,9,10]
    //      p   i                   first swap 6 and 1, second swap 4 and 1 (the 1 is now in the 6 spot from prior swap)
    // [2,3,1,4,6,9,10]
    //        p   i
    for i in 1..v.len() {
        if v[i] < v[pivot] {
            // move pivot forward and put this element before it
            v.swap(pivot + 1, i);
            v.swap(pivot + 1, pivot);
            pivot += 1;
        }
    }
    pivot
}

// n log n? except i think worse case is still n^2
fn quick_sort<T: PartialOrd>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]); // pivot is the first member of b and is already sorted
}

#[cfg(test)]
mod tests {
    use crate::{bubble_sort, bubble_sort_optimized, merge_sort, pivot, quick_sort};

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 2, 2];
        let sorted = vec![1, 2, 2, 4, 6, 8, 11, 13];
        bubble_sort(&mut v);
        assert_eq!(v, sorted)
    }

    #[test]
    fn test_bubble_sort_optimized() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 2, 2];
        let sorted = vec![1, 2, 2, 4, 6, 8, 11, 13];
        bubble_sort_optimized(&mut v);
        assert_eq!(v, sorted)
    }

    #[test]
    fn test_merge_sort() {
        let v = vec![2222, 2333, 4, 6, 1, 8, 11, 1017, 13, 3, 104, 106, 33, 99, 0, -1, -9000, 9000];
        let v = merge_sort(v);
        assert_eq!(v, vec![-9000, -1, 0, 1, 3, 4, 6, 8, 11, 13, 33, 99, 104, 106, 1017, 2222, 2333, 9000])
    }

    #[test]
    fn test_pivot() {
        let mut v = vec![4, 9, 1, 3, 2, 3, 9, 94, 1, 3, 9, 8, 8, 7];
        let p = pivot(&mut v);
        assert_eq!(v, vec![1, 3, 2, 3, 1, 3, 4, 94, 9, 9, 9, 8, 8, 7]);
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 2, 2, 101, -1000333];
        let sorted = vec![-1000333, 1, 2, 2, 4, 6, 8, 11, 13, 101];
        quick_sort(&mut v);
        assert_eq!(v, sorted)
    }
}
