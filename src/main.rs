#![allow(dead_code, special_module_name, unstable_name_collisions)]

use std::time::Instant;
use prettytable::{format, Table};

use crate::lib::grammar::{Grammar};
use crate::lib::parsers::{GrammarParserLALR1, GrammarParserLL1, GrammarParserLR, GrammarParserSLR1};

mod lib;

fn main() {
    let grammar: Grammar = Grammar::from_json_file("grammars/lalr1_example.json").unwrap();
    println!("\n{}", grammar);
    println!("\n{}\n", grammar.to_jsmachine_string());

    let parser_ll1 = GrammarParserLL1::from_grammar(&grammar);
    println!("LL(1) - {}", parser_ll1.is_ll1());

    let mut last = Instant::now();
    let parser_lr0 = GrammarParserSLR1::from_grammar(&grammar);
    println!("Parser generated ({}ms)", last.elapsed().as_millis());

    last = Instant::now();
    let parser_lalr1 = GrammarParserLALR1::from_grammar(&grammar);
    println!("Parser generated ({}ms)", last.elapsed().as_millis());

    let mut pt = Table::new();
    pt.set_format(*format::consts::FORMAT_BOX_CHARS);
    parser_lr0.get_parse_table().to_prettytable(&mut pt);
    pt.printstd();

    // let mut trace = Table::new();
    // trace.set_format(*format::consts::FORMAT_BOX_CHARS);
    // let res = parser_lr0.parse_from_string_trace(&grammar, "A -> 'a';", Some(&mut trace));
    // if let Err(e) = res {
    //     println!("{}", e);
    // } else if let Ok(d) = res {
    //     d.print_std();
    // }
    // trace.printstd();

    // let parser_slr1 = GrammarParserSLR1::from_grammar(&grammar);
    // trace = Table::new();
    // trace.set_format(*format::consts::FORMAT_BOX_CHARS);
    // trace.set_titles(row![cFyb => "Step", "Stack", "Lookahead", "Action"]);
    // res = parser_slr1.parse_from_string_trace(&grammar, "2 + 2 * 2", Some(&mut trace));
    // if let Err(e) = res {
    //     println!("{}", e);
    // }
    // trace.printstd();
}
