use std::collections::{HashMap, HashSet};
use std::io::Result;

pub trait Token {
    fn get_term(&self) -> String;
    fn get_offset_begin(&self) -> usize;
    fn get_offset_end(&self) -> usize {
        self.get_term().len() + self.get_offset_begin()
    }
    fn get_pos(&self) -> Option<String>;
}

pub trait Document<ID, TOK>: Send + Sync
where
    ID: Into<String>,
    TOK: Token,
{
    fn get_id(&self) -> Box<ID>;
    fn get_content(&self) -> Vec<Box<TOK>>;
}

pub struct Tfidf<'a, ID: Into<String>, TOK: Token> {
    docs: &'a [Box<dyn Document<ID, TOK>>],
    cache: HashMap<String, usize>,
}

impl<'a, ID: Into<String>, TOK: Token> Tfidf<'a, ID, TOK> {
    pub fn new(src: &'a [Box<dyn Document<ID, TOK>>]) -> Self {
        Tfidf {
            docs: src,
            cache: HashMap::new(),
        }
    }

    pub fn fit_transform(&mut self) -> Result<()> {
        for doc in self.docs {
            let mut seen = HashSet::new();
            for token in doc.get_content() {
                let term = token.get_term();
                if !seen.contains(&term) {
                    seen.insert(term.clone());
                    let entry = self.cache.entry(term.clone()).or_insert(1);
                    *entry += 1;
                }
            }
        }
        Ok(())
    }

    pub fn tfidf_doc(&self, doc: &dyn Document<ID, TOK>, term: &dyn Token) -> Result<f64> {
        self.tfidf(doc.get_content().as_slice(), term)
    }

    pub fn tfidf(&self, doc: &[Box<TOK>], term: &dyn Token) -> Result<f64> {
        let tf = self.tf(doc, term);
        let idf = self.idf(term);
        if tf * idf < 0f64 {
            panic!("oooo {} {}", tf, idf);
        }

        Ok(tf * idf)
    }

    pub fn rank_tokens(&self, doc: &[Box<TOK>]) -> Result<HashMap<String, f64>> {
        let mut acc = HashMap::new();
        for t in doc {
            acc.insert(t.get_term(), self.tfidf(&doc, t.as_ref())?);
        }
        Ok(acc)
    }

    fn tf(&self, doc: &[Box<TOK>], term: &dyn Token) -> f64 {
        doc.iter()
            .filter(|s| s.get_term() == term.get_term())
            .fold(0f64, |acc, _| acc + 1f64)
    }

    fn idf(&self, term: &dyn Token) -> f64 {
        let f = self.cache.get(term.get_term().as_str()).unwrap_or(&0);
        let f = *f as f64 + 1f64;
        let n = self.docs.len() as f64;

        let div = n / f;
        div.log10()
    }
}

//  LocalWords:  tokenizer
