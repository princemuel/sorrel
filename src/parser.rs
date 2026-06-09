use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{fs, io};

pub(crate) struct Parser;
impl Parser {
    pub(crate) fn read_by_ext<P>(path: P) -> io::Result<String>
    where
        P: AsRef<Path>,
    {
        use io::Error;

        let path = path.as_ref();
        let ext = path.extension().ok_or_else(|| {
            Error::other(format!(
                "cant detect the file tpe of this path '{path}' w/o the extension",
                path = path.display()
            ))
        })?;

        let content = match ext.to_str() {
            Some("xml" | "xhtml") => Self::read_xml(path)?,
            Some("txt" | "md") => Self::read_text(path)?,
            _ => {
                return Err(Error::other(format!(
                    "unsupported extension {ext}",
                    ext = ext.display()
                )));
            }
        };

        Ok(content)
    }

    pub(crate) fn read_text<P>(path: P) -> io::Result<String>
    where
        P: AsRef<Path>,
    {
        fs::read_to_string(path)
    }

    pub(crate) fn read_xml<P>(path: P) -> io::Result<String>
    where
        P: AsRef<Path>,
    {
        use xml::EventReader;
        use xml::common::{Position as _, TextPosition};
        use xml::reader::XmlEvent;

        let path = path.as_ref();
        let file = File::open(path)?;
        let reader = EventReader::new(BufReader::new(file));

        let mut buffer = String::new();
        for event in reader {
            let event = event.map_err(|err| {
                let TextPosition { row, column } = err.position();
                eprintln!("{path}:{row}:{column}: ERROR: {err}", path = path.display());
                io::Error::other(err)
            })?;

            if let XmlEvent::Characters(ref text) = event {
                buffer.push_str(text);
                buffer.push(' ');
            }
        }

        Ok(buffer)
    }
}
