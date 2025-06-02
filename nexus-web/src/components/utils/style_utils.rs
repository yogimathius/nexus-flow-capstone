/// Combines multiple CSS classes into a single string
///
/// This utility function takes a slice of class strings and joins them together
/// with proper spacing, removing any empty classes.
pub fn combine_classes(classes: &[&str]) -> String {
    classes
        .iter()
        .filter(|class| !class.is_empty())
        .cloned()
        .collect::<Vec<&str>>()
        .join(" ")
}
