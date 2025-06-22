impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        
        let n = needle.len();
        let m = haystack.len();

        if m == 0 || n > m {
            return -1;
        }
        
        for i in 0..=m.saturating_sub(n) {
            if &haystack[i..i+n] == needle {
                return i as i32;
            }
        }
        
        -1
       
    }
}