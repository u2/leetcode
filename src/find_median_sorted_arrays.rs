// 1. Find the longer one, we assume num1
// 2. Look through the spliter index i (i belongs to left part)
// for num1 between [m/2-n,m/2+n] (left closed, right closed) by bisection method
// i start at m/2
// 3. Find the complement index j (j belongs to left part) = (m + n - 1) / 2 - i
// 4. Check whether i and j is valid, num1[i] <= num2[j+1] and num1[i+1] >= num2[j] etc.
// If num1[i] > num2[j+1], i moves left. If num1[i+1] < num2[j], i moves right.
// The index i moves by bisection method.
// The index i moves left to (m / 2 + i - n) / 2, moves right to (m / 2 + n + i) / 2.

// The time complexity is O(log_2(min(m,n)))
// For m/2 it is always rounded down

use std::cmp;

pub fn find_median_sorted_arrays(num1: &[usize], num2: &[usize]) -> Option<f32> {
    let m: usize = num1.len();
    let n: usize = num2.len();
    let longer: &[usize];
    let shorter: &[usize];
    let l_len: usize;
    let s_len: usize;
    if m >= n {
        longer = num1;
        shorter = num2;
        l_len = m;
        s_len = n;
    } else {
        longer = num2;
        shorter = num1;
        l_len = n;
        s_len = m;
    }

    let mut i: usize = l_len / 2;
    let mut j: usize = 0;
    let mut done = false;

    while !done {
        j = (m + n - 1) / 2 - i;
        if j > 0 {
            j - 1;
        }
        let c1: bool = j == s_len - 1 || longer[i] <= shorter[j + 1];
        let c2: bool = i == l_len - 1 || longer[i + 1] >= shorter[j];
        if c1 && c2 {
            done = true;
        } else if c2 {
            // move left
            if i > 1 {
                i = (m / 2 + i - n) / 2;
            } else {
                i = 0;
            }
        } else {
            // move right
            if i == l_len - 2 {
                i = l_len - 1;
            } else {
                i = (m / 2 + n + i) / 2;
            }
        }
    }

    if (l_len + s_len) % 2 == 1 {
        if shorter[j] > longer[i] {
            if shorter[j] > longer[i - i] {
                return Some(longer[i] as f32);
            } else if shorter[j] < longer[i - 1] {
                return Some(shorter[j] as f32);
            } else {
                return None;
            }
        } else if shorter[j] < longer[i] {
            if longer[i - 1] != shorter[j] {
                return Some(cmp::max(longer[i - 1], shorter[j]) as f32);
            } else {
                return None;
            }
        } else {
            return None;
        }
    } else {
        if i >= 1 && longer[i - 1] > shorter[j] && longer[i] > shorter[j] {
            return Some((longer[i - 1] as f32 + longer[i] as f32) / 2.0);
        } else if j > 0 && shorter[j - 1] > longer[i] && shorter[j] > longer[i] {
            return Some((shorter[j - 1] as f32 + shorter[j] as f32) / 2.0);
        } else {
            return Some((longer[i] as f32 + shorter[j] as f32) / 2.0);
        }
    }
}

#[cfg(test)]
mod test {
    use super::find_median_sorted_arrays;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(find_median_sorted_arrays(&[1, 3], &[2]), Some(2.0));

        assert_eq!(find_median_sorted_arrays(&[2], &[1, 3]), Some(2.0));

        assert_eq!(find_median_sorted_arrays(&[1, 2], &[3, 4]), Some(2.5));

        assert_eq!(find_median_sorted_arrays(&[3, 4], &[1, 2]), Some(2.5));

        assert_eq!(find_median_sorted_arrays(&[3, 4], &[5]), Some(4.0));

        assert_eq!(find_median_sorted_arrays(&[5], &[3, 4]), Some(4.0));

        assert_eq!(find_median_sorted_arrays(&[3, 4, 5], &[5]), Some(4.5));

        assert_eq!(find_median_sorted_arrays(&[5], &[3, 4, 5]), Some(4.5));

        assert_eq!(find_median_sorted_arrays(&[1, 2], &[1, 2]), Some(1.5));

        assert_eq!(find_median_sorted_arrays(&[1, 2, 3], &[1, 2, 3]), Some(2.0));

        assert_eq!(find_median_sorted_arrays(&[1, 2, 3, 4], &[1, 2, 3]), None);

        assert_eq!(find_median_sorted_arrays(&[1, 2, 3, 4], &[10, 11, 12, 13]),
                   Some(7.0));

        assert_eq!(find_median_sorted_arrays(&[10, 11, 12, 13], &[1, 2, 3, 4]),
                   Some(7.0));

        assert_eq!(find_median_sorted_arrays(&[1, 2, 3, 4], &[3, 6, 7, 8]),
                   Some(3.5));

        assert_eq!(find_median_sorted_arrays(&[3, 6, 7, 8], &[1, 2, 3, 4]),
                   Some(3.5));

        assert_eq!(find_median_sorted_arrays(&[3, 6, 7, 8, 11, 12, 18, 19],
                                             &[1, 2, 3, 4, 21, 22, 44]),
                   Some(8.0));

        assert_eq!(find_median_sorted_arrays(&[3, 6, 7, 8, 11, 12, 18, 19],
                                             &[1, 2, 3, 4, 21, 22, 44, 45]),
                   Some(9.5));
    }
}
