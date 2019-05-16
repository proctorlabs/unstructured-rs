use crate::*;
use pest::Parser;
use pest_derive::*;
use std::collections::BTreeMap;

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Serialize)]
    struct TestStruct {
        val: String,
        vals: Vec<usize>,
    }

    #[test]
    fn test_selector() {
        let mut doc = Document::new(TestStruct {
            val: "some_val".to_string(),
            vals: vec![1, 2, 3],
        })
        .unwrap();

        assert_eq!(doc.select("/vals/1").unwrap().clone(), Document::U64(2));

        assert_eq!(
            doc.select("/vals/1").unwrap().clone(),
            doc.select_mut("/vals/1").unwrap().clone()
        );

        assert_eq!(
            doc.select(".[\"vals\"].[1]").unwrap(),
            doc.select("/vals/1").unwrap()
        );
    }

    #[test]
    fn test_filter() -> Result<(), String> {
        let docs = vec![
            Document::new(TestStruct {
                val: "some_val".to_string(),
                vals: vec![1, 2, 3],
            })
            .map_err(|e| format!("{:?}", e))?,
            Document::new(TestStruct {
                val: "Another".to_string(),
                vals: vec![4, 5, 6],
            })
            .map_err(|e| format!("{:?}", e))?,
        ];

        let result = Document::filter(&docs, "[1].val | [0].vals.[1:3]")?;
        println!("{}", result);
        Ok(())
    }
}

macro_rules! parse_array_index {
    ($pair:ident, $name:ident) => {
        $name[$pair
            .as_str()
            .parse::<usize>()
            .map_err(|e| format!("Parse failure: {}!", e))?]
    };
}

macro_rules! parse_char_string {
    ($pair: ident) => {
        String::from($pair.as_str())
    };
}

macro_rules! parse_char {
    ($pair:ident, $name:ident) => {
        $name[$pair.as_str()]
    };
}

macro_rules! parse_ident_string {
    ($pair: ident) => {
        String::from($pair.as_str())
    };
}

macro_rules! parse_ident {
    ($pair:ident, $name:ident) => {
        $name[$pair.as_str()]
    };
}

macro_rules! parse_range {
    ($pair:ident, $name:ident) => {
        match $name {
            Document::Seq(s) => {
                let mut range: Vec<usize> = $pair
                    .as_str()
                    .split(":")
                    .map(|v| v.parse::<usize>().unwrap_or(0))
                    .collect();
                if range[1] > s.len() || range[1] == 0 {
                    range[1] = s.len();
                }
                if range[0] >= s.len() {
                    Document::Seq(vec![])
                } else {
                    let res = Vec::from(&s[range[0]..range[1]]);
                    Document::Seq(res)
                }
            }
            _ => Err(format!("Cannot take range on non-sequence value!"))?,
        }
    };
}

macro_rules! parse_doc_index {
    ($pair:ident) => {
        $pair
            .as_str()
            .parse::<usize>()
            .map_err(|e| format!("Parse failure: {}!", e))?
    };
}

#[derive(Parser)]
#[grammar = "selector/grammar/selector.pest"]
struct SelectorParser;

impl Document {
    pub fn select<'a>(&'a self, sel: &str) -> Result<&'a Document, String> {
        let selection = SelectorParser::parse(Rule::selector, sel).map_err(|e| e.to_string())?;
        let mut result = self;
        for selector in selection {
            match selector.as_rule() {
                Rule::index => result = &parse_array_index!(selector, result),
                Rule::chars => result = &parse_char!(selector, result),
                Rule::ident => result = &parse_ident!(selector, result),
                Rule::EOI => return Ok(result),
                _ => return Err(format!("Invalid selector {}", selector)),
            };
        }
        Ok(result)
    }

    pub fn select_mut<'a>(&'a mut self, sel: &str) -> Result<&'a mut Document, String> {
        let selection = SelectorParser::parse(Rule::selector, sel).map_err(|e| e.to_string())?;
        let mut result = self;
        for selector in selection {
            match selector.as_rule() {
                Rule::index => result = &mut parse_array_index!(selector, result),
                Rule::chars => result = &mut parse_char!(selector, result),
                Rule::ident => result = &mut parse_ident!(selector, result),
                Rule::EOI => return Ok(result),
                _ => return Err(format!("Invalid selector {}", selector)),
            };
        }
        Ok(result)
    }

    pub fn filter(docs: &[Document], sel: &str) -> Result<Document, String> {
        let mut result = Document::Map(BTreeMap::new());
        if !docs.is_empty() {
            let mut current_owned = None;
            let mut current = &docs[0];
            let mut key_path = vec![];
            let selection =
                SelectorParser::parse(Rule::selector_filter, sel).map_err(|e| e.to_string())?;
            for selector in selection {
                match selector.as_rule() {
                    Rule::doc_index => {
                        let index = parse_doc_index!(selector);
                        if index >= docs.len() {
                            Err(format!("Document index of {} is out of bounds", index))?;
                        } else {
                            current = &docs[index];
                        }
                    }
                    Rule::doc_wildcard => {
                        for doc in docs.iter() {
                            result = result + doc.clone();
                        }
                    }
                    Rule::index => current = &parse_array_index!(selector, current),
                    Rule::chars => {
                        current = &parse_char!(selector, current);
                        if current != &Document::Unit {
                            key_path.push(parse_char_string!(selector));
                        }
                    }
                    Rule::ident => {
                        current = &parse_ident!(selector, current);
                        if current != &Document::Unit {
                            key_path.push(parse_ident_string!(selector));
                        }
                    }
                    Rule::range => current_owned = Some(parse_range!(selector, current)),
                    Rule::EOI | Rule::pipe => {
                        if !key_path.is_empty() {
                            let mut tree = Document::Map(BTreeMap::default());
                            let mut pos = &mut tree;
                            for (i, path) in key_path.iter().enumerate() {
                                let mut new_doc = Document::Map(BTreeMap::default());
                                if i == key_path.len() - 1 {
                                    new_doc = new_doc
                                        + match current_owned {
                                            Some(s) => s,
                                            None => current.clone(),
                                        };
                                    current_owned = None;
                                    current = &docs[0];
                                }
                                pos[&path] = new_doc;
                                pos = &mut pos[&path];
                            }
                            if tree != Document::Unit {
                                result = result + tree;
                            }
                            key_path.clear();
                        } else {
                            let temp = match current_owned {
                                Some(s) => s,
                                None => current.clone(),
                            };
                            if temp != Document::Unit {
                                result = result + temp;
                            }
                            current_owned = None;
                            current = &docs[0];
                        }
                    }
                    _ => return Err(format!("Invalid selector {}", selector)),
                }
            }
        }
        Ok(result)
    }
}
