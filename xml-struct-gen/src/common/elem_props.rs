use heck::ToSnakeCase;
use linked_hash_set::LinkedHashSet;
use xml::name::OwnedName;

#[derive(Debug, Clone, Default)]
pub struct ElemProps {
    pub attributes: Vec<OwnedName>,
    pub child_stacks: LinkedHashSet<Vec<OwnedName>>,
    pub has_text: bool,
    pub is_root: bool,
}

impl ElemProps {
    pub fn get_attr_fields(&self) -> Vec<AttrField> {
        self.attributes
            .iter()
            .map(|x| AttrField {
                xml_name: x.clone(),
                sanitized_name: sanitize_field_name(&x.local_name),
            })
            .collect()
    }
}

fn sanitize_field_name(name: &str) -> String {
    format!("r#{}", name.to_snake_case())
}

pub struct AttrField {
    pub xml_name: OwnedName,
    pub sanitized_name: String,
}
