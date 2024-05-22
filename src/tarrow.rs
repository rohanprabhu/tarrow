use crate::models::StorableTarrowTree;
use crate::object_db::object_db::ObjectDb;
use crate::tree_db::tree_db::TreeDb;

struct Tarrow {
    object_db: ObjectDb,
    tree_db: TreeDb
}

impl Tarrow {
    fn store(
        &mut self,
        tree: StorableTarrowTree
    ) {
    }
}
