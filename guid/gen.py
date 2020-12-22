s = """#[allow(unreachable_patterns)]

// Get partition type given 16-byte GUID
pub(crate) fn get_str_from_ty_uuid(uuid: &[u8]) -> Option<&'static str> {
    match uuid {
"""

with open("./guid/guids.txt", "r") as f:
    content = f.read()

    guids = content.split("\n")
    for pair in guids:
        name = " ".join(pair.split(" ")[:-1])
        guid = "".join([chr for chr in pair.split(" ")[-1] if chr != '-'])

        s += """        &[{}] => Some("{}"),
""".format(", ".join(['0x' + str(guid[i:i + 2]) for i in range(0, len(guid), 2)]), name)

s += """        _ => Some("Unknown"),
    }
}
"""

print(s)