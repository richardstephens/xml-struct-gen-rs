pub(crate) fn structname_for(p0: &str) -> String {
    let mut name = "".to_string();
    let (s, r) = p0.split_at(1);
    name.push_str(s.to_ascii_uppercase().as_str());
    let mut next_upper = false;
    for c in r.chars() {
        if c == '_' {
            next_upper = true;
        } else if next_upper {
            name.push(c.to_ascii_uppercase());
            next_upper = false;
        } else {
            name.push(c);
        }
    }
    name
}
