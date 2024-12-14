#![allow(non_camel_case_types)]

mod Matrix;
mod TestVars;
mod TermDisplay;
mod VTerm;


fn main() -> Result<(), String> {
    let mut term_display = TermDisplay::TermDisplay::new()?;
    term_display.test_render()?;
    term_display.run()?;
    Ok(())
}