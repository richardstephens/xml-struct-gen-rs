use indexmap::IndexMap;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::common::elem_props::ElemProps;
use xml::EventReader;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

pub struct StructScanner {
    elem_map: IndexMap<Vec<OwnedName>, ElemProps>,
    pub namespaces: BTreeMap<String, String>,
}

impl StructScanner {
    pub fn new() -> Self {
        let mut elem_map: IndexMap<Vec<OwnedName>, ElemProps> = IndexMap::new();
        elem_map.insert(vec![], ElemProps::default());

        Self {
            elem_map,
            namespaces: BTreeMap::new(),
        }
    }

    pub fn scan_structs(
        &mut self,
        path: &Path,
    ) -> anyhow::Result<IndexMap<Vec<OwnedName>, ElemProps>> {
        let file = BufReader::new(File::open(path)?);
        let parser = EventReader::new(file);

        let mut elem_stack: Vec<OwnedName> = Vec::new();
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => {
                    self.append_namespaces(namespace);
                    let parent_stack = elem_stack.clone();
                    elem_stack.push(name.clone());

                    self.elem_map
                        .get_mut(&parent_stack)
                        .unwrap()
                        .child_stacks
                        .insert(elem_stack.clone());

                    if !self.elem_map.contains_key(&elem_stack) {
                        self.elem_map
                            .insert(elem_stack.clone(), ElemProps::default());
                    }
                    let elem_props = self.elem_map.get_mut(&elem_stack).unwrap();

                    for attr in attributes {
                        if !elem_props.attributes.contains(&attr.name) {
                            elem_props.attributes.push(attr.name.clone());
                        }
                    }
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    elem_stack.pop();
                }
                Err(e) => {
                    panic!("XML Parse error: {:?}", e)
                }
                Ok(XmlEvent::Characters(_)) => {
                    self.elem_map.get_mut(&elem_stack).unwrap().has_text = true;
                }
                _ => {}
            }
        }
        Ok(self.elem_map.clone())
    }

    fn append_namespaces(&mut self, ns: Namespace) {
        for (pfx, url) in ns.0 {
            self.namespaces.insert(pfx, url);
        }
    }
}
