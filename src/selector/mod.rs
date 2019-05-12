use crate::*;

#[cfg(feature = "selector")]
mod parser;

impl Document {
    pub fn pointer<'a>(&'a self, pointer: &str) -> Option<&'a Document> {
        if pointer == "" {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        let tokens = pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"));
        let mut target = self;

        for token in tokens {
            let target_opt = match *target {
                Document::Map(ref map) => map.get(&token.into()),
                Document::Seq(ref list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => return None,
            };
            if let Some(t) = target_opt {
                target = t;
            } else {
                return None;
            }
        }
        Some(target)
    }

    pub fn pointer_mut<'a>(&'a mut self, pointer: &str) -> Option<&'a mut Document> {
        if pointer == "" {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        let tokens = pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"));
        let mut target = self;

        for token in tokens {
            // borrow checker gets confused about `target` being mutably borrowed too many times because of the loop
            // this once-per-loop binding makes the scope clearer and circumvents the error
            let target_once = target;
            let target_opt = match *target_once {
                Document::Map(ref mut map) => map.get_mut(&token.into()),
                Document::Seq(ref mut list) => {
                    parse_index(&token).and_then(move |x| list.get_mut(x))
                }
                _ => return None,
            };
            if let Some(t) = target_opt {
                target = t;
            } else {
                return None;
            }
        }
        Some(target)
    }
}

fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}
