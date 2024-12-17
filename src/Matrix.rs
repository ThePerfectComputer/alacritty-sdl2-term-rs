use crate::aterm::ATerm;
// data structure for terminal display
pub struct Matrix {
    rows: u32,
    cols: u32,
    pub content: Vec<Vec<Option<char>>>,
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

    pub fn populate_from_aterm(&mut self, aterm: &ATerm) {
        let term = aterm.term.lock();
        let grid = term.grid().clone();

        // Reset the matrix content to None
        for row in self.content.iter_mut() {
            for col in row.iter_mut() {
                *col = None;
            }
        }

        // Populate from grid
        for indexed in grid.display_iter() {
            let x = indexed.point.column.0 as usize;
            let y = indexed.point.line.0 as usize;
            if y < self.rows as usize && x < self.cols as usize {
                self.content[y][x] = Some(indexed.c);
            }
        }
    }
}
