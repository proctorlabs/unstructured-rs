use crate::*;
use pest::Parser;
use pest_derive::*;

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
                Rule::index => result = &result[selector.as_str().parse::<usize>().unwrap()],
                Rule::chars | Rule::ident => result = &result[selector.as_str()],
                _ => (),
            };
        }
        Ok(result)
    }

    pub fn select_mut<'a>(&'a mut self, sel: &str) -> Result<&'a mut Document, String> {
        let selection = SelectorParser::parse(Rule::selector, sel).map_err(|e| e.to_string())?;
        let mut result = self;
        for selector in selection {
            match selector.as_rule() {
                Rule::index => result = &mut result[selector.as_str().parse::<usize>().unwrap()],
                Rule::chars | Rule::ident => result = &mut result[selector.as_str()],
                _ => (),
            };
        }
        Ok(result)
    }
}
