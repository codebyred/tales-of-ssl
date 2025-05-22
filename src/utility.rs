pub fn get_iso_country_name_from(str: &str) -> String {

    let mut iso_country_name = String::new();

    for word in str.split_whitespace() {
        if let Some(c) = word.chars().next() {
            if c.is_ascii_uppercase() {
                iso_country_name.push(c);
            }
        }
    }

    iso_country_name

}