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
    for far in 1..v.len() {
        if v[far] < v[pivot] {
            // move pivot forward and put this element before it
            let near = pivot + 1;
            v.swap(near, far);
            v.swap(near, pivot);
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
    // pivot is the first member of b and is already sorted
    // also i think if we didn't exclude the pivot, then it could move around
    // again but its already in its proper place. might even overflow the stack?
    quick_sort(&mut b[1..]);
}

fn quick_sort2<T: PartialOrd>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot2(v);
    let (a, b) = v.split_at_mut(p);
    quick_sort2(a);
    quick_sort2(&mut b[1..]);
}

// this version of pivot picks a random element from the array to act as the
// initial pivot. picking a random pivot minimizes the chance of encountering
// worst case scenario O(n^2). picking the first element as pivot in a nearly
// sorted array can yield this worst case scenario.
fn pivot2<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);
    p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            // move pivot forward and put this element before it
            v.swap(p + 1, i);
            v.swap(p + 1, p);
            p += 1;
        }
    }
    p
}
struct RawSend<T>(*mut [T]);

unsafe impl<T> Send for RawSend<T> {}

// this will try to spawn an obscene number of threads for a big vec
fn threaded_quick_sort<T: 'static + PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot2(v);
    let (a, b) = v.split_at_mut(p);

    let raw_a: *mut [T] = a as *mut [T];
    let raw_s = RawSend(raw_a);
    unsafe {
        let handle = std::thread::spawn(move || {
            // explicitly move raw_s into the closure otherwise we just move the
            // contents of the struct when we access below
            let raw_s = raw_s;
            threaded_quick_sort(&mut *raw_s.0);
        });
        threaded_quick_sort(&mut b[1..]);

        handle.join().ok();
    }
}

fn quick_sort_rayon<T:Send + PartialOrd>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot2(v);
    let (a,b) = v.split_at_mut(p);

    // put 2nd func on a queue then start func1
    // if another thread is ready then it will execute func2
    // will work recursively
    rayon::join(||quick_sort_rayon(a), ||quick_sort_rayon(&mut b[1..]));
}

// random fibonacci facts:

// fib(0) = 1 is the "combinatorial" definition (f_n)
// fib(0) = 0 is the "classical" definition (F_n)

// Program with fib(0) = 1; spits out fib(4) = 5
// 1 1 2 3 5

// Program with fib(0) = 0; spits out fib(4) = 3
// 0 1 1 2 3


// this is a "combinatorial" impl
fn fibonacci(n:i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    fibonacci(n -1) + fibonacci(n -2)
}

// this is also a "combinatorial" definition because the base case of n=0
// returns (1,0) with 1 being the actual answer and 0 being n-1 and really just
// used as storage for one of the arms in the fibonacci tree of calls so that it
// doesnt need to be recalculated again and again
fn fibonacci_dynamic(n:i32) -> (i32, i32) {
    if n == 0 {
        return (1, 0);
    }
    // a is fib(n-1) and b is fib(n-2)
    let (a, b) = fibonacci_dynamic(n - 1);

    // return the answer of (fib(n), fib(n-1)) so that fib(n-1) doesn't need to
    // be recalculated like it would in the previous fib impl
    (a+b, a)
}


#[cfg(test)]
mod tests {
    use crate::{bubble_sort, bubble_sort_optimized, fibonacci_dynamic, merge_sort, pivot, quick_sort, quick_sort2, quick_sort_rayon, threaded_quick_sort};

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

    #[test]
    fn test_quick_sort2() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 2, 2, 101, -1000333];
        let sorted = vec![-1000333, 1, 2, 2, 4, 6, 8, 11, 13, 101];
        quick_sort2(&mut v);
        assert_eq!(v, sorted)
    }

    #[test]
    fn test_threaded_quick_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 2, 2, 101, -1000333];
        let sorted = vec![-1000333, 1, 2, 2, 4, 6, 8, 11, 13, 101];
        threaded_quick_sort(&mut v);
        assert_eq!(v, sorted)
    }


    #[test]
    fn test_rayon_quick_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 2, 2, 101, -1000333];
        let sorted = vec![-1000333, 1, 2, 2, 4, 6, 8, 11, 13, 101];
        quick_sort_rayon(&mut v);
        assert_eq!(v, sorted)
    }

    #[test]
    fn test_fib() {
        let result = fibonacci_dynamic(2);
        assert_eq!(result, (9, 8));
    }
}


