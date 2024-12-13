// data structure for terminal display
pub struct Matrix {
    rows: u32,
    cols: u32,
    content: Vec<Vec<Option<char>>>,
}

impl Matrix {
    pub fn new(rows: u32, cols: u32) -> Matrix {
        // Initialize content with rows x cols matrix of None
        let content = vec![vec![None; cols as usize]; rows as usize];
        Matrix {
            rows,
            cols,
            content,
        }
    }
    pub fn populate_from_string(&mut self, input: &str) {
        let mut row_idx = 0; // Track the current row index
        let mut col_idx = 0; // Track the current column index

        for c in input.chars() {
            if row_idx >= self.rows as usize {
                break; // Stop if we've run out of rows
            }

            if c == '\n' {
                // Handle newline: move to the next row
                row_idx += 1;
                col_idx = 0; // Reset column index
            } else if col_idx < self.cols as usize {
                // Populate the current cell
                self.content[row_idx][col_idx] = Some(c);
                col_idx += 1;
            }
        }
    }
}