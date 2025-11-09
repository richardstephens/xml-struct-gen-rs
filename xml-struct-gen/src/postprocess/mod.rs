use crate::common::elem_props::ElemProps;
use anyhow::bail;
use indexmap::IndexMap;
use xml::name::OwnedName;

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

pub fn validate_text_invariant(
    structs: &IndexMap<Vec<OwnedName>, ElemProps>,
) -> anyhow::Result<()> {
    for (k, v) in structs.iter() {
        if v.has_text && !v.child_stacks.is_empty() {
            bail!(
                "Path:{:?} has both text and child elements, this is unsupported",
                k
            );
        }
    }

    Ok(())
}
