pub fn serialize_name(string: String) -> String {
    string.replace("-", "$")
}
