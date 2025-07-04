import re
import pathlib

# get path of Cargo.toml
this_dir = pathlib.Path(__file__).parents[1]
cargo_path = this_dir.joinpath("Cargo.toml")
cargo_path = cargo_path.resolve()


# read Cargo.toml and return version
def get_version():
    with open(cargo_path) as cargo_toml:
        cargo_toml = cargo_toml.read()
        version = re.search(r"version = \"(.+)\"", cargo_toml)
        if version:
            return version.group(1)
        else:
            return None


# update version in Cargo.toml
def update_version(currunt, updated):
    with open(cargo_path) as cargo_toml:
        toml_data = cargo_toml.read()
        toml_data = toml_data.replace(currunt, updated)
    with open(cargo_path, "w") as cargo_toml:
        cargo_toml.write(toml_data)


# get new version
def new_version(version):
    version = version.split(".")
    # update minor version till 9 else update major version and reset minor version
    if int(version[-1]) < 99:
        version[-1] = str(int(version[-1]) + 1)
    else:
        version[-1] = "0"
        version[-2] = str(int(version[-2]) + 1)
    return ".".join(version)


def main():
    currunt = get_version()
    if currunt:
        print(f"Current version: {currunt}")
        updated = new_version(currunt)
        update_version(currunt, updated)
        print(f"Version updated to {updated}")
    else:
        print("Version not found")


main()
