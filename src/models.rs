use buffer::VecBuffer;
use serde_json::Value;

pub struct DataBlob<'a> {
    pub content: &'a VecBuffer<'a>,
    pub metadata: Value
}

pub enum TreeEntry<'a> {
    Tree(TreeEntry<'a>),
    Blob(DataBlob<'a>)
}

pub struct StorableTarrowTree<'a> {
    pub entries: Vec<(String, Option<TreeEntry<'a>>)>
}

pub struct TarrowTree<'a> {
    pub entries: Vec<(String, TreeEntry<'a>)>
}

struct TarrowRef {
    pub ref_name: String
}

impl TarrowRef {
    fn from(ref_name: String) -> Self {
        Self {
            ref_name
        }
    }
}
