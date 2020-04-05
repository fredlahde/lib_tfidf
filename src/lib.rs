use std::io::Read;
trait Document {
    fn get_content(&self) -> Box<dyn Read>;
}

trait TfidfDataSource<'a> {
    fn get_all_documents(&'a self) -> &'a [Box<dyn Document>];
}

#[cfg(test)]
mod test {
    use super::*;
    struct TestDoc {
        s: String,
    }

    impl Document for TestDoc {
        fn get_content(&self) -> Box<dyn Read> {
            use std::io::Cursor;
            let buff = Cursor::new(vec![0xf; 15]);
            return Box::new(buff);
        }
    }
    struct TestSource {
        docs: Vec<Box<dyn Document>>,
    }

    impl<'a> TfidfDataSource<'a> for TestSource {
        fn get_all_documents(&'a self) -> &'a [Box<dyn Document>] {
            &self.docs.as_slice()
        }
    }
    #[test]
    fn test_docs() {
        let d = TestDoc { s: "hello".into() };
        let docs;
        let source = TestSource {
            docs: vec![Box::new(d)],
        };
        docs = source.get_all_documents();
        let mut v = vec![];
        docs[0].get_content().read_to_end(&mut v).unwrap();
        println!("{:?}", v)
    }
}
