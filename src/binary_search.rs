/// Searches for a target in a sorted slice using binary search.
/// Returns Option<usize> containing the index of the target if found, otherwise None.
pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    // 'low' is the starting index of the search space.
    let mut low = 0;
    // 'high' is the ending index of the search space.
    let mut high = arr.len();

    // Loop as long as the search space is valid (low < high).
    while low < high {
        // Calculate the middle index to avoid potential overflow.
        let mid = low + (high - low) / 2;

        match arr[mid].cmp(item) {
            // The middle element is less than the item, so discard the left half.
            std::cmp::Ordering::Less => {
                low = mid + 1;
            }
            // The middle element is greater than the item, so discard the right half.
            std::cmp::Ordering::Greater => {
                high = mid;
            }
            // The middle element is equal to the item, we found it!
            std::cmp::Ordering::Equal => {
                return Some(mid);
            }
        }
    }
    // If the loop finishes, the item was not found.
    None
}