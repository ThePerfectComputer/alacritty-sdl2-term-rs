use crate::ATerm::ATerm;
use crate::TestVars::{CONTENT1, CONTENT2};
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

    pub fn populate_from_string(&mut self, input: &str) {
        let mut chars = input.chars().peekable();

        for row in self.content.iter_mut() {
            let mut hit_newline = false;
            for col in row.iter_mut() {
                if hit_newline {
                    *col = None;
                } else {
                    match chars.peek() {
                        Some(&'\n') => {
                            *col = None;
                            chars.next();
                            hit_newline = true;
                        }
                        Some(&c) => {
                            *col = Some(c);
                            chars.next();
                        }
                        None => {
                            *col = None;
                        }
                    }
                }
            }
        }
    }

    pub fn set_to_content1(&mut self) {
        self.populate_from_string(CONTENT1);
    }

    pub fn set_to_content2(&mut self) {
        self.populate_from_string(CONTENT2);
    }
}
