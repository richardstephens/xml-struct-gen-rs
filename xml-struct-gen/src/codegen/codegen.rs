use crate::codegen::codegen_stubs::gen_el_struct;

use crate::codegen::util::structname_for;
use crate::common::elem_props::ElemProps;
use bimap::BiMap;
use indexmap::IndexMap;
use std::collections::HashSet;
use xml::name::OwnedName;

pub fn generate_code(e: IndexMap<Vec<OwnedName>, ElemProps>) -> String {
    let assigned_names = assign_struct_names(e.keys().filter(|k| !k.is_empty()).cloned().collect());

    let mut s = "use serde::{Deserialize, Serialize};\n".to_string();
    for (k, v) in e.into_iter() {
        let el_struct = gen_el_struct(&k, &v, &assigned_names);
        s.push_str(&el_struct.to_string());
    }
    s
}

fn assign_struct_names(stacks: Vec<Vec<OwnedName>>) -> BiMap<Vec<OwnedName>, String> {
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
            structname_for(&stack.last().unwrap().local_name)
        };

        result.insert(
            stack,
            unused_name_maybe_with_suffix(&proposed_name, |n| result.contains_right(n)),
        );
    }

    result
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
