/// Searches for a target in a slice using linear search.
/// Returns Option<usize> containing the index of the target if found, otherwise None.
pub fn linear_search<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    // Iterate through the array with both index and element.
    for (index, value) in arr.iter().enumerate() {
        // If the current element matches the item we are looking for...
        if value == item {
            // ...return its index wrapped in Some.
            return Some(index);
        }
    }
    // If the loop completes without finding the item, return None.
    None
}