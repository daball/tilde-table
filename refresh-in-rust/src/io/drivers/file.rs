use std::fs::File;
use std::{io, io::{Bytes, Read, Seek, SeekFrom}};
use std::path::PathBuf;
use super::{Driver, DriverOption, DriverOptionParam, ReadDriver, Reader, ReaderContext};

pub struct FileReaderContext {
    pub file: Option<File>,
    pub path: PathBuf,
}

impl ReaderContext<std::fs::File> for FileReaderContext {
    fn is_open(&self) -> bool {
        match &self.file {
            Some(_file) => true,
            None => false,
        }
    }
    fn is_closed(&self) -> bool {
        match &self.file {
            Some(_file) => false,
            None => true,
        }
    }
    fn can_seek(&self) -> bool {
        match &self.file {
            Some(_file) => true,
            None => false,
        }
    }
    fn bytes(&mut self) -> io::Result<Bytes<&mut std::fs::File>> {
        match &mut self.file {
            Some(file) => Ok(file.bytes()),
            None => Err(io::Error::new(io::ErrorKind::Other, "No file provided for read.")),
        }
    }
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match &mut self.file {
            Some(file) => file.seek(pos),
            None => Err(io::Error::new(io::ErrorKind::Other, "No file provided for seek.")),
        }
    }
    fn close(&mut self) {
        match &self.file {
            Some(_file) => self.file = None,
            None => {},
        }
    }
}

pub struct FileReader {}

impl Reader<FileReaderContext, File> for FileReader {
    fn open(&self, params: Vec<DriverOptionParam>) -> io::Result<FileReaderContext> {
        let mut path: Option<PathBuf> = None;
        for param in &params {
            if param.name.to_lowercase() == "path" {
                path = Some(PathBuf::from(&param.value));
            }
        }
        match path {
            Some(path) => {
                let file = File::open(path.as_path())?;
                Ok(FileReaderContext{
                    path: path,
                    file: Some(file),
                })
            },
            None => Err(io::Error::new(io::ErrorKind::Other, "No path provided.")),            
        }
    }
}

pub struct FileDriver {}

impl Driver for FileDriver {
    fn options(&self) -> Vec<DriverOption> {
        vec![
            DriverOption {
                name: String::from("path"),
                flag_name: String::from("path"),
                description: String::from("path to a file"),
                is_required: true,
            }        
        ]
    }
}

impl ReadDriver<FileReader, FileReaderContext, File> for FileDriver {
    fn reader(&self) -> FileReader {
        FileReader {}
    }
}
