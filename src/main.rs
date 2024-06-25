mod parser;
mod textbox;
mod tree;
mod utils;

fn main() -> std::io::Result<()> {
    let srcfile = "./testdata/mock/test.txt";
    parser::parse_and_render(&srcfile)?;
    Ok(())
}
