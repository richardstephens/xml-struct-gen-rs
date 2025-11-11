use crate::codegen::stubs::gen_el_struct;

use crate::codegen::AssignedTypeMap;
use crate::codegen::stubs_ns::generate_namespace_const;
use crate::common::elem_props::ElemProps;
use bimap::BiMap;
use indexmap::IndexMap;
use std::collections::{BTreeMap, HashSet};
use xml::name::OwnedName;

pub fn generate_code(
    ns: BTreeMap<String, String>,
    mut e: IndexMap<Vec<OwnedName>, ElemProps>,
) -> anyhow::Result<String> {
    let assigned_names =
        assign_struct_names(e.keys().filter(|k| !k.is_empty()).cloned().collect())?;

    let mut s = "use serde::{Deserialize, Serialize};\n".to_string();
    s.push_str("use std::collections::HashMap;\n");
    s.push_str("pub use xml_struct_types::v1::*;");

    let doc_struct_root_props = e.shift_remove(&vec![]).unwrap();

    s.push_str(&generate_namespace_const(ns).to_string());

    for (k, v) in e.into_iter() {
        let rp = if v.is_root {
            Some(doc_struct_root_props.clone())
        } else {
            None
        };
        let el_struct = gen_el_struct(&k, &v, &assigned_names, rp);
        s.push_str(&el_struct.to_string());
    }
    Ok(s)
}

fn assign_struct_names(stacks: Vec<Vec<OwnedName>>) -> anyhow::Result<AssignedTypeMap> {
    let mut result = BiMap::new();

    let mut seen_names = HashSet::new();
    let mut dup_names = HashSet::new();
    for stack in stacks.iter() {
        if !seen_names.insert(stack.last().unwrap().local_name.clone()) {
            dup_names.insert(stack.last().unwrap().local_name.clone());
        }
    }

    for stack in stacks {
        let proposed_name = if dup_names.contains(&stack.last().unwrap().local_name) {
            fully_qualified_name(&stack)
        } else {
            format!(
                "{}",
                heck::AsUpperCamelCase(&stack.last().unwrap().local_name)
            )
        };

        result.insert(
            stack,
            unused_name_maybe_with_suffix(&proposed_name, |n| result.contains_right(n)),
        );
    }

    Ok(result)
}

fn fully_qualified_name(stack: &Vec<OwnedName>) -> String {
    let mut parts = vec![];
    for item in stack.iter() {
        if let Some(pfx) = item.prefix.as_ref()
            && !pfx.trim().is_empty()
        {
            parts.push(pfx.to_string());
        }
        parts.push(item.local_name.clone());
    }
    format!("{}", heck::AsUpperCamelCase(&parts.join(" ")))
}

fn unused_name_maybe_with_suffix(base: &str, is_in_use: impl Fn(&str) -> bool) -> String {
    if !is_in_use(base) {
        base.to_string()
    } else {
        let mut index = 2;
        loop {
            let proposed_name = format!("{base}{index}");
            if !is_in_use(&proposed_name) {
                return proposed_name;
            } else {
                index += 1;
            }
        }
    }
}
