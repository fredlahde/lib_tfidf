use std::collections::HashMap;
use std::io::{Read, Result};
trait Document<ID>: Send + Sync
where
    ID: Into<String>,
{
    fn get_id(&self) -> Box<ID>;
    fn get_content(&self) -> Box<dyn Read>;
}

trait TfidfDataSource<'a, T>: Send + Sync
where
    T: Into<String>,
{
    fn get_all_documents(&'a self) -> &'a [Box<dyn Document<T>>];
    fn get_document(&'a self, idx: T) -> Option<&'a dyn Document<T>>;
}

#[derive(Hash, Eq, Debug)]
struct Token {
    term: String,
    position: usize,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.term == other.term
    }
}

struct Tfidf<'a, ID: Into<String>> {
    source: &'a dyn TfidfDataSource<'a, ID>,
    cache: HashMap<Token, usize>,
}

impl<'a, ID: Into<String>> Tfidf<'a, ID> {
    fn new(src: &'a dyn TfidfDataSource<'a, ID>) -> Self {
        Tfidf {
            source: src,
            cache: HashMap::new(),
        }
    }

    fn fit_transform(&mut self) -> Result<()> {
        for doc in self.source.get_all_documents() {
            let mut content = String::new();
            doc.get_content().read_to_string(&mut content)?;

            // TODO proper tokenizer
            // maybe as argument with trait
            for (position, term) in content.split(' ').enumerate() {
                let term = term.to_owned();
                let token = Token { term, position };
                let entry = self.cache.entry(token).or_insert(0);
                *entry += 1;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    struct TestDoc {
        path: &'static str,
    }

    impl Document<String> for TestDoc {
        fn get_content(&self) -> Box<dyn Read> {
            Box::new(std::fs::File::open(self.path).unwrap())
        }

        fn get_id(&self) -> Box<String> {
            Box::new(self.path.to_owned())
        }
    }
    struct TestSource<ID>
    where
        ID: Into<String>,
    {
        docs: Vec<Box<dyn Document<ID>>>,
    }

    impl<'a> TfidfDataSource<'a, String> for TestSource<String> {
        fn get_all_documents(&'a self) -> &'a [Box<dyn Document<String>>] {
            &self.docs.as_slice()
        }

        fn get_document(&'a self, idx: String) -> Option<&'a dyn Document<String>> {
            let idx: String = idx.into();
            for d in &self.docs {
                let didx: String = d.get_id().as_ref().to_owned();
                if didx == idx {
                    return Some(d.as_ref());
                }
            }
            return None;
        }
    }

    #[test]
    fn test_docs() {
        let d = TestDoc { path: "test.doc" };
        let docs;
        let source = TestSource {
            docs: vec![Box::new(d)],
        };
        docs = source.get_all_documents();
        let mut v = String::new();
        docs[0].get_content().read_to_string(&mut v).unwrap();
    }

    #[test]
    fn test_tfidf_construction() {
        let test_doc_fn = || TestDoc { path: "test.doc" };
        let source = TestSource {
            docs: vec![Box::new(test_doc_fn()), Box::new(test_doc_fn())],
        };
        let tfidf = Tfidf::new(&source);
    }
    #[test]
    fn test_tfidf_fit_transform() {
        let test_doc_fn = || TestDoc { path: "test.doc" };
        let source = TestSource {
            docs: vec![Box::new(test_doc_fn()), Box::new(test_doc_fn())],
        };
        let mut tfidf = Tfidf::new(&source);
        tfidf.fit_transform().unwrap();
        println!("{:?}", tfidf.cache);
    }
}

//  LocalWords:  tokenizer
