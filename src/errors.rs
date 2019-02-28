error_chain! {
        errors {
            BadParse(nonterminal: String, desc: String, _text: String) {
                description("a parse error"),
                display("Parse error: {:1} {:0}", desc, nonterminal),
            }
        }

        foreign_links{
            Io(::std::io::Error);
            ParseFloat(::std::num::ParseFloatError);
        }
    }
