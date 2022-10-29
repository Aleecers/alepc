from dataclasses import dataclass
import re


def remove_cotetion(string: str) -> str:
    """Remove the quotation marks from the string."""
    return string.strip('"').strip("'")


def divide_text(string: str) -> list:
    """divide text into 3 lines to each element."""
    lines = string.splitlines()
    return [lines[i : i + 3] for i in range(0, len(lines), 3)]


def get_type_from_string(string: str) -> str:
    """Return the type of the string content."""
    if string.isdigit():
        return "Integer"
    elif string.replace(".", "", 1).isdigit():
        return "Float"
    elif string.lower() in ("true", "false"):
        return "Boolean"
    else:
        if len(string) == 1:
            return "Char"
        else:
            return "String"


@dataclass
class Configration:
    name: str
    f_type: type
    default: str
    docs: str

    def write(self) -> str:
        """Write the configuration line."""
        return f"|`{self.name}`| {get_type_from_string(self.default)} | {self.docs} | `{self.default}` |"


def parse_config(docs: str, default: str, name_and_type: str) -> Configration:
    """Parse config elements.
    # Arguments
    docs: /// {docs}
    default: #[educe(Default = {default})]
    name_and_type: pub {name}: {f_type},
    """
    name = re.search(r"pub (.*):", name_and_type).group(1)
    f_type = re.search(r"pub .*: (.*),", name_and_type).group(1)
    default = re.search(r"Default = (.*)\)", default).group(1)
    docs = re.search(r"/// (.*)", docs).group(1)
    return Configration(name, f_type, remove_cotetion(default), docs)


# Get it from `config_utils.rs`
STR_CONFIG = """/// The question of select action
#[educe(Default = "What do you want to do ❓")]
pub select_action_message: String,
/// Create a new post choice in select
#[educe(Default = "Create a new post ✍")]
pub new_post_choice: String,
/// Update an existing post choice
#[educe(Default = "Update existing post 🖌️")]
pub update_existing_post: String,
/// Version choice
#[educe(Default = "Alepc Version ⚙")]
pub version_choice: String,"""


def main():
    configs = divide_text(STR_CONFIG)
    for config in configs:
        print(parse_config(*config).write())


if __name__ == "__main__":
    main()
