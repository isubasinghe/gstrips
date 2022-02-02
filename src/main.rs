use lrlex::lrlex_mod;
use lrpar::lrpar_mod;
use std::env;
mod ast;
lrlex_mod!("gstrips.l");
lrpar_mod!("gstrips.y");

fn main() {
    let lexerdef = gstrips_l::lexerdef();

    println!("Hello, world!");
}
