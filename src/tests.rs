#[cfg(test)]
mod regex {
    use crate::utils::{ExtractRegexArray, ExtractRegexStr};
    use regex::Regex;
    #[test]
    fn delete_duplicates() {
        let vector = vec!["1".to_string(), "1".to_string(), "3".to_string(), "4".to_string()].clone();
        assert_eq!(vector, vec!["1".to_string(), "1".to_string(), "3".to_string(), "4".to_string()]);
        let vector = ["1", "1", "3", "4"];
        assert_eq!(vector, ["1", "1", "3", "4"]);
    }
    #[test]
    fn get_regex_result() {
        let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let string = "d-2010-03-14".extract_regex(&re).unwrap();
        let ok_result = "2010-03-14";
        assert_eq!(string, ok_result);
    }
    #[test]
    fn get_regex_result_array() {
        let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let string = vec!["d-2010-03-14".to_string(), "d-2010-03-25".to_string()].extract_regex(&re).unwrap();
        let ok_result = vec!["2010-03-14", "2010-03-25"];
        assert_eq!(string, ok_result);
    }
}
#[test]
fn clear() {
    let mut result_with_capacity = String::with_capacity(100);
    result_with_capacity.push_str("hello");
    let mut result_without_capacity = String::new();
    result_without_capacity.push_str("hello");
    result_with_capacity.clear();
    result_without_capacity.clear();
    assert_eq!(result_with_capacity, "");
    assert_eq!(result_without_capacity, "");
    assert_eq!(result_with_capacity.len(), 0);
    assert_eq!(result_without_capacity.len(), 0);
}