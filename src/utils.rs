use regex::Regex;
use std::collections::{BTreeSet, HashSet};

pub trait Clear {
    fn clear(_: &mut String);
}
pub trait ExtractRegexStr {
    fn extract_regex(&self, regex: &Regex) -> Option<String>;
}
pub trait ExtractRegexArray {
    fn extract_regex(&self, regex: &Regex) -> Option<Vec<String>>;
}

impl ExtractRegexArray for [String] {
    ///
    ///
    /// # Arguments
    ///
    /// * `regex`:
    ///   allocate new Vec for returned result
    ///   returns: Option<Vec<String, Global>>
    ///
    /// # Examples
    ///
    /// ```
    ///  use regex::Regex;
    ///  use text_utils_s::utils::ExtractRegexArray;
    ///  let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    ///  let string = vec!["d-2010-03-14".to_string(), "d-2010-03-25".to_string()].extract_regex(&re).unwrap();
    ///  let ok_result = vec!["2010-03-14", "2010-03-25"];
    /// ```
    fn extract_regex(&self, regex: &Regex) -> Option<Vec<String>> {
        let mut result: Vec<String> = Vec::new();
        for i in self {
            match regex.find(i) {
                Some(fin) => {
                    result.push(fin.as_str().to_string());
                }
                _ => continue,
            }
        }
        if result.is_empty() {
            return None;
        }
        Some(result)
    }
}
impl ExtractRegexStr for str {
    ///
    ///
    /// # Arguments
    ///
    /// * `regex`:
    ///
    /// returns: Option<String>
    ///
    /// # Examples
    ///
    /// ```
    /// use regex::Regex;
    /// use text_utils_s::utils::ExtractRegexStr;
    /// let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    /// let string = "d-2010-03-14".extract_regex(&re).unwrap();
    /// let ok_result = "2010-03-14";
    /// ```
    fn extract_regex(&self, regex: &Regex) -> Option<String> {
        let result = regex.find(self).map(|m| m.as_str().to_string());
        result
    }
}
impl Clear for String {
    ///
    ///
    /// # Arguments
    ///
    /// * `string`: for clear
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn clear(string: &mut String) {
        string.replace_range(0..string.len(), "")
    }
}

/// Returned sorted iterator
/// Time: O(n log n)
/// Memory: O(n)
/// Use if you are needed sorted result
/// # Arguments
///
/// * `array`:  impl IntoIterator,
///
/// returns:  impl IntoIterator + Sized
///
/// # Examples
///
/// ```
///  use text_utils_s::utils::unique_sorted;
///  let vector = vec!["1", "8", "1", "3", "4"].clone();
///  let result = unique_sorted(vector.clone())
///     .into_iter()
///     .collect::<Vec<_>>();
///  assert_eq!(
///     result,
///     vec![
///         "1".to_string(),
///         "3".to_string(),
///         "4".to_string(),
///         "8".to_string(),
///     ]
///  );
///
/// ```
pub fn unique_sorted<T: Ord>(array: impl IntoIterator<Item = T>) -> impl IntoIterator<Item = T> {
    let set = BTreeSet::from_iter(array);
    set.into_iter()
}
/// Use if you aren't needed sorted result. Elements will be in a chaotic order.
/// Time: O(n)
/// Memory: O(n)
/// # Arguments
///
/// * `array`: impl IntoIterator
///
/// returns: impl IntoIterator + Sized
///
/// # Examples
/// ```
///  use text_utils_s::utils::unique_ch;
///  let vector = vec!["1", "8", "1", "3", "4"].clone();
///  let result = unique_ch(vector.clone()).into_iter().collect::<Vec<_>>();
///  for _ in 0..result.len() - 1 {
///     assert_eq!(result.contains(&"4"), true);
///     assert_eq!(result.contains(&"8"), true);
///     assert_eq!(result.contains(&"3"), true);
///     assert_eq!(result.contains(&"1"), true);
///     }
///  }
///
/// ```
pub fn unique_ch<T: Ord + std::hash::Hash>(
    array: impl IntoIterator<Item = T>,
) -> impl IntoIterator<Item = T> {
    let mut set = HashSet::new();
    for value in array {
        set.insert(value);
    }
    set.into_iter()
}
///
/// Use if are you needed saved order. Use if you are having few elements
/// Time: O(n²)
/// Memory: O(n)
/// # Arguments
///
/// * `array`: impl IntoIterator
///
/// returns: impl IntoIterator + Sized
///
/// # Examples
///
/// ```
///  use text_utils_s::utils::unique;
///
///  fn delete_duplicates_vector() {
///     let vector = vec!["1", "8", "1", "3", "4"].clone();
///     let result = unique(vector.clone()).into_iter().collect::<Vec<_>>();
///     assert_eq!(result, vec!["1", "8", "3", "4",]);
///  }
///
/// ```
pub fn unique<T: PartialEq>(array: impl IntoIterator<Item = T>) -> impl IntoIterator<Item = T> {
    let mut vec = Vec::new();
    for value in array {
        if !vec.contains(&value) {
            vec.push(value)
        }
    }
    vec.into_iter()
}
