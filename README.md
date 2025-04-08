[![Crate](https://img.shields.io/crates/v/text_utils_s.svg)](https://crates.io/crates/text_utils_s)

# Extension for default type. You are can extract regular expression from array.

````rust
let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
let string = vec!["d-2010-03-14".to_string(), "d-2010-03-25".to_string()].extract_regex(&re).unwrap();
let ok_result = vec!["2010-03-14", "2010-03-25"];

let string = String::new("Hello, World!);
string.replace_range(0..string.len(), "")

````
## You can also remove duplicates from an array.
### There are three ways:
### Returned sorted iterator. Use if you are needed sorted result
### Time: O(n log n)
### Memory: O(n)
````rust
let vector = vec!["1", "8", "1", "3", "4"].clone();
let result = unique_sorted(vector.clone())
  .into_iter()
  .collect::<Vec<_>>();
assert_eq!(result,
            vec![
                "1".to_string(),
                "3".to_string(),
                "4".to_string(),
                "8".to_string(),
            ]
);
````
## Use if you aren't needed sorted result. Elements will be in a chaotic order.
### Time: O(n) 
### Memory: O(n)
````rust
let vector = vec!["1", "8", "1", "3", "4"].clone();
let result = unique_ch(vector.clone()).into_iter().collect::<Vec<_>>();
for _ in 0..result.len() - 1 {
  assert_eq!(result.contains(&"4"), true);
  assert_eq!(result.contains(&"8"), true);
  assert_eq!(result.contains(&"3"), true);
  assert_eq!(result.contains(&"1"), true);
}
````
##  Use if are you needed saved order. Use if you are having few elements 
### Time: O(n²)
### Memory: O(n)
````rust
let vector = vec!["1", "8", "1", "3", "4"];
let result = unique(vector.clone()).into_iter().collect::<Vec<_>>();
assert_eq!(result, vec!["1", "8", "3", "4",]);
````
