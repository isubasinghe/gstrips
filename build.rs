use cfgrammar::yacc::YaccKind;
use lrlex::LexerBuilder;
use lrpar::CTParserBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lex_rule_id_maps = CTParserBuilder::new()
        .yacckind(YaccKind::Grmtools)
        .process_file_in_src("gstrips.y")?;
    LexerBuilder::new()
        .rule_ids_map(lex_rule_id_maps)
        .process_file_in_src("gstrips.l")?;
    Ok(())
}
