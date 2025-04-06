use regex::Regex;

pub trait Clear {
    fn clear(_: &mut String);
}
pub trait ExtractRegexStr {
    fn extract_regex(&self, regex: &Regex) -> Option<String>;
}
pub trait ExtractRegexArray {
    fn extract_regex(&self, regex: &Regex) -> Option<Vec<String>>;
}
pub trait DeleteDuplicate {
    fn delete(self) -> Vec<String>;
}
impl DeleteDuplicate for Vec<String> {
    fn delete(self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::with_capacity(self.len());
        for str in self {
            if !vec.contains(&str) {
                vec.push(str);
            }
        }
        vec
    }
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
