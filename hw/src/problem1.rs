/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut sum: i32 = 0;

    for el in slice {
        sum += el;
    }

    return sum;
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];

    for el in vs {
      if !res.contains(el) {
        res.push(*el);
      }
    }

    return res;
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
  let mut res: Vec<i32> = vec![];

  for el in vs {
    if pred(*el) {
      res.push(*el);
    }
  }

  return res;
}
