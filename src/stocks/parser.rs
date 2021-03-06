// Expected format of the tree:
// 'Stocks'
//   <Stock>*
//     'Lots'
//       <Lot>
//
// Stock   := Symbol '-' [ Name '-' ]? Num ['-' Tags]?
// Symbol  := /[A-Z.]*/
// Name    := StringWithSpaces
// Num     := A floating point number (no scientific notation)
// Tags    := Tag*
// Tag     := /[@#][A-Za-z0-9]*/
//
// Lot     := ???????
//
// All whitespace is ignored except in StringWithSpaces.

use crate::errors::*;
use std::collections::hash_set::HashSet;

fn parse_tag(str: &str) -> Result<&str> {
    let result = str.trim();
    if !result.starts_with("@") && !result.starts_with("#") {
        bail!(ErrorKind::BadParse(
            "TAG",
            "must begin with '@' or '#'.",
            "".to_string()
        ));
    }

    // TODO: check for valid characters as described above in BNF
    if result.contains(char::is_whitespace) {
        bail!("Tag cannot contain white space.");
    }

    Ok(str.trim())
}

fn parse_tags(str: &str) -> Result<HashSet<&str>> {
    let mut result = HashSet::new();
    for tag in str.split_whitespace() {
        result.insert(parse_tag(tag)?);
    }
    Ok(result)
}

fn parse_num(str: &str) -> Result<f32> {
    Ok(str.trim().parse()?)
}

fn parse_name(str: &str) -> Result<&str> {
    Ok(str.trim())
}

fn parse_symbol(str: &str) -> Result<&str> {
    let result = str.trim();

    // Test for _invalid_ characters.
    let test = result.contains(|c: char| !c.is_ascii_uppercase() && c != '.');

    if test {
        bail!(ErrorKind::BadParse(
            "SYMBOL",
            "must contain only A-Z and '.'",
            str.to_string()
        ));
    }

    Ok(result)
}

pub fn parse_stock(str: &str) -> Result<super::Stock> {
    // Stock   := Symbol '-' [ Name '-' ]? Num ['-' Tags]?

    let pieces: Vec<&str> = str.split(" - ").collect();

    let (symbol, name, num, tags) = match pieces.len() {
        4 => {
            let symbol = parse_symbol(pieces[0])?.into();
            let name = Some(parse_name(pieces[1])?.into());
            let num = parse_num(pieces[2])?;
            let tags = parse_tags(pieces[3])?
                .iter()
                .map(|s| s.to_string())
                .collect();
            Ok((symbol, name, num, tags))
        }
        3 => {
            // Can be either symbol/name/num or symbol/num/tags.
            // Try to tell them apart by parsing the num field.
            let symbol = parse_symbol(pieces[0])?.into();
            if let Ok(num) = parse_num(pieces[2]) {
                let name = parse_name(pieces[1])?.into();
                Ok((symbol, Some(name), num, HashSet::new()))
            } else {
                let num = parse_num(pieces[1])?;
                let tags = parse_tags(pieces[2])?
                    .iter()
                    .map(|s| s.to_string())
                    .collect();
                Ok((symbol, None, num, tags))
            }
        }
        2 => {
            // Only two components, it must be symbol and num.
            let symbol = parse_symbol(pieces[0])?.into();
            let num = parse_num(pieces[1])?;
            Ok((symbol, None, num, HashSet::new()))
        }
        i if i > 4 => Err(ErrorKind::BadParse(
            "Stock",
            "Extra components",
            str.to_string(),
        )),
        _ => Err(ErrorKind::BadParse(
            "Stock",
            "Missing components",
            str.to_string(),
        )),
    }?;

    Ok(super::Stock {
        symbol,
        name,
        num,
        tags,
        lots: vec![],
        last_price: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stocks::Stock;
    use crate::type_tools::VecTools;
    use std::iter::FromIterator;

    #[test]
    fn test_bad_stock() {
        assert_eq!(
            Stock {
                symbol: "CL".into(),
                name: None,
                num: -33.0,
                tags: HashSet::from_iter(
                    vec!["@etrade", "@longshort", "@short"]
                        .to_strings()
                        .into_iter()
                ),
                lots: vec![],
            },
            dbg!(parse_stock("CL - -33 - @etrade @longshort @short").unwrap())
        );
    }

    #[test]
    fn test_stock() {
        assert_eq!(
            Stock {
                symbol: "AAPL".into(),
                name: Some("Apple Computer".into()),
                num: 3.0,
                tags: HashSet::from_iter(vec!["@foo", "#bar"].to_strings().into_iter()),
                lots: vec![],
            },
            parse_stock("AAPL - Apple Computer - 3 - @foo #bar").unwrap()
        );

        // Test present, but empty, tags.

        // Test missing name
        assert_eq!(
            Stock {
                symbol: "AAPL".to_owned(),
                name: None,
                num: 3.0,
                tags: HashSet::from_iter(vec!["@foo".to_string()].into_iter()),
                lots: vec![],
            },
            parse_stock("AAPL - 3 - @foo").unwrap()
        );

        // Test missing tags
        assert_eq!(
            Stock {
                symbol: "AAPL".to_owned(),
                name: Some("Apple Computer".to_owned()),
                num: 3.0,
                tags: HashSet::new(),
                lots: vec![],
            },
            parse_stock("AAPL - Apple Computer - 3").unwrap()
        );

        // Test missing both name and tags
        assert_eq!(
            Stock {
                symbol: "AAPL".to_owned(),
                name: None,
                num: 3.0,
                tags: HashSet::new(),
                lots: vec![],
            },
            parse_stock("AAPL - 3").unwrap()
        );

        assert!(parse_stock("").is_err());
        assert!(parse_stock("FOO").is_err());
        assert!(parse_stock("FOO - BAR").is_err());
        assert!(parse_stock("FOO - BAR - BAZ - QUUX - BAM").is_err());
    }

    #[test]
    fn test_symbol() {
        assert_eq!("AAPL", parse_symbol("AAPL").unwrap());
        assert_eq!("AAPL", parse_symbol("   AAPL").unwrap());
        assert_eq!("AAPL", parse_symbol("AAPL   ").unwrap());
        assert_eq!("AAPL", parse_symbol("  AAPL    ").unwrap());

        assert_eq!("BRK.B", parse_symbol("  BRK.B  ").unwrap());

        assert!(parse_symbol("NO SPACES").is_err());
        assert!(parse_symbol("nolower").is_err());
        assert!(parse_symbol("nopunct!").is_err());
    }

    #[test]
    fn test_name() {
        assert_eq!("quux", parse_name("quux").unwrap());
        assert_eq!("quux bar", parse_name("quux bar").unwrap());
        assert_eq!("quux bar", parse_name("   quux bar").unwrap());
        assert_eq!("quux bar", parse_name("quux bar   ").unwrap());
        assert_eq!("quux bar", parse_name("    quux bar   ").unwrap());

        assert_eq!("!#@# \t\n!!@@", parse_name("  !#@# \t\n!!@@     ").unwrap());
    }

    #[test]
    fn test_num() {
        assert_eq!(3.14, parse_num("3.14").unwrap());
        assert_eq!(-3.14, parse_num("-3.14").unwrap());
        assert_eq!(3.14, parse_num("  3.14").unwrap());
        assert_eq!(3.14, parse_num("3.14  ").unwrap());
        assert_eq!(3.14, parse_num("   3.14  ").unwrap());

        assert_eq!(3.0, parse_num("3").unwrap());
        assert_eq!(-3.0, parse_num("-3").unwrap());

        assert_eq!(0.0, parse_num("0").unwrap());
        assert_eq!(0.0, parse_num("000").unwrap());
        assert_eq!(0.0, parse_num("-000").unwrap());

        assert!(parse_num("xxx").is_err());
    }

    #[test]
    fn test_tags() {
        assert_eq!(HashSet::<&'static str>::new(), parse_tags("").unwrap());
        assert_eq!(
            HashSet::from_iter(vec!["@one"].into_iter()),
            parse_tags("@one").unwrap()
        );
        assert_eq!(
            HashSet::from_iter(vec!["@one", "#two", "@three"].into_iter()),
            parse_tags("@one #two @three").unwrap()
        );
        assert_eq!(
            HashSet::from_iter(vec!["@one", "#two", "@three"].into_iter()),
            parse_tags("  @one #two @three  ").unwrap()
        );
        assert_eq!(
            HashSet::from_iter(vec!["@one", "#two", "@three"].into_iter()),
            parse_tags("@one    #two   @three").unwrap()
        );
    }

    #[test]
    fn test_tag() {
        assert_eq!("@foo", parse_tag("@foo").unwrap());
        assert_eq!("@foo", parse_tag("   @foo").unwrap());
        assert_eq!("@foo", parse_tag("@foo   ").unwrap());
        assert_eq!("@foo", parse_tag("   @foo   ").unwrap());

        assert_eq!("#quux", parse_tag("  #quux  ").unwrap());

        // TODO: check the errors for what I expect.
        assert!(parse_tag("xxx").is_err());
        assert!(parse_tag("@ foo").is_err());
    }

}
