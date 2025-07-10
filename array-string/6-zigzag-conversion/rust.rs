impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || num_rows as usize >= s.len() {
            return s;
        }

        let num_rows = num_rows as usize;
        let mut rows = vec![Vec::new(); num_rows];
        let mut curr_row = 0;
        let mut step = 1; // 1 for down, -1 for up

        for c in s.chars() {
            rows[curr_row].push(c);
            if curr_row == 0 {
                step = 1; // Move down
            } else if curr_row == num_rows - 1 {
                step = -1; // Move up
            }
            curr_row = (curr_row as i32 + step) as usize;
        }

        rows.into_iter().flat_map(|row| row).collect()
    }
}