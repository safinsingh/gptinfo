s = """
const KNOWN_GUIDS: &[(&[u8], &str)] = &[
"""

with open("./guid/guids.txt", "r") as f:
    content = f.read()

    guids = content.split("\n")
    for pair in guids:
        name = " ".join(pair.split(" ")[:-1])
        guid = "".join([chr for chr in pair.split(" ")[-1] if chr != '-'])

        s += """    (&[{}], "{}"),
""".format(", ".join(['0x' + str(guid[i:i + 2]) for i in range(0, len(guid), 2)]), name)

s += """];

pub fn get_str_from_ty_uuid(uuid: &[u8]) -> Option<&'static str> {
  if uuid == &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] {
    return None;
  }

  for (candidate, name) in KNOWN_GUIDS {
    if uuid == candidate {
      return Some(name);
    }
  }

  None
}
"""

print(s)
