pub mod file;
use std::{io, io::{Bytes, SeekFrom}};

pub struct DriverOption {
    pub name: String,
    pub flag_name: String,
    pub description: String,
    pub is_required: bool,
}

pub struct DriverOptionParam {
    pub name: String,
    pub value: String,
}

pub trait ReaderContext<U> {
    fn is_open(&self) -> bool;
    fn is_closed(&self) -> bool;
    fn can_seek(&self) -> bool;
    fn bytes(&mut self) -> io::Result<Bytes<&mut U>>;
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64>;
    fn close(&mut self);
}

pub trait Reader<RC: ReaderContext<U>, U> {
    fn open(&self, params: Vec<DriverOptionParam>) -> io::Result<RC>;
}

pub trait Driver
{
    fn options(&self) -> Vec<DriverOption>;
}

pub trait ReadDriver<R: Reader<RC, U>, RC: ReaderContext<U>, U>
{
    fn reader(&self) -> R;
}
