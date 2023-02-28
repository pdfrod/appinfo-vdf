use std::ffi::CString;

pub mod printer;
pub mod reader;
pub mod updater;
pub mod writer;

#[derive(Debug)]
pub struct VDF {
    header: VDFHeader,
    sections: Vec<VDFAppSection>,
}

#[derive(Clone, Debug)]
struct VDFHeader {
    magic: u32,
    version: u32,
}

#[derive(Clone, Debug)]
struct VDFAppSection {
    app_id: u32,
    data_size: u32,
    info_state: u32,
    last_updated: u32,
    pics_token: u64,
    sha1: [u8; 20],
    change_number: u32,
    binary_hash: [u8; 20],
    nodes: Vec<VDFAppNode>,
}

#[derive(Clone, Debug)]
enum VDFAppNode {
    Simple {
        name: CString,
        children: Vec<VDFAppNode>,
    },
    Str {
        name: CString,
        value: CString,
    },
    Int {
        name: CString,
        value: u32,
    },
}

enum VDFAppNodeKind {
    Simple = 0,
    Str = 1,
    Int = 2,
    End = 8,
}
