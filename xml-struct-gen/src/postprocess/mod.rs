
use anyhow::bail;
use indexmap::IndexMap;
use xml::name::OwnedName;
use crate::common::elem_props::ElemProps;

pub fn mark_root(structs: &mut IndexMap<Vec<OwnedName>, ElemProps>) -> anyhow::Result<()> {
    let root_elemprops = structs.get(&Vec::new()).unwrap();

    if root_elemprops.child_stacks.len() != 1 {
        bail!(
            "Expected exactly one stack in root, found len={}",
            root_elemprops.child_stacks.len()
        );
    }
    let root_stack = root_elemprops.child_stacks.iter().cloned().next().unwrap();

    let ep_to_mark = structs.get_mut(&root_stack).unwrap();
    ep_to_mark.is_root = true;

    Ok(())
}
