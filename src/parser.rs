#![expect(dead_code)]
use std::io;
use std::path::Path;

pub(crate) struct Parser;
impl Parser {
    pub(crate) fn read_xml<P>(_path: P) -> io::Result<String>
    where
        P: AsRef<Path>,
    {
        todo!()
    }
}
