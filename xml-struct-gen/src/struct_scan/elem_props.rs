use linked_hash_set::LinkedHashSet;
use xml::name::OwnedName;

#[derive(Debug, Clone, Default)]
pub struct ElemProps {
    pub attributes: Vec<OwnedName>,
    pub child_stacks: LinkedHashSet<Vec<OwnedName>>,
    pub has_text: bool,
}
