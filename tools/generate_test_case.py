import argparse
import os
import pathlib
import re
import subprocess
from time import process_time

this_dir = pathlib.Path(__file__).parents[1]
# path to binary
VEDIC = this_dir.joinpath("target/release/vedic")
if os.name == "nt":
    VEDIC = VEDIC.with_suffix(".exe")
VEDIC = VEDIC.resolve()


# Create test cases from tests folder
def get_test_cases_file():
    tests_path = pathlib.Path("tests")
    tests = sorted(pathlib.Path(tests_path).glob("*/*.ved"))
    return tests


# Clear existing test cases
def clear(file, clear_test=False):
    last = True
    with open(file, "r") as f:
        lines = f.readlines()
    with open(file, "w") as f:
        for line in lines:
            if clear_test:
                if line.startswith("# CTE:"):
                    continue
                if line.startswith("# RTE:"):
                    continue
            if len(line.strip()) == 0:
                if last:
                    f.write("\n")
                    last = False
                    continue
                else:
                    last = True
                    continue
            f.write(line)
            last = True


# Generate test cases for file
def generate_test(file):
    output = subprocess.run(
        [VEDIC, file],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )

    # out = [x.decode("utf-8").strip() for x in output.stdout.split(b"\n")[:-1]]
    err = [x.decode("utf-8").strip() for x in output.stderr.split(b"\n")[:-1]]

    with open(file, "a") as f:
        if output.returncode > 0 and len(err) > 0:
            for x in err:
                if output.returncode == 65:
                    f.write("\n# CTE: " + x)
                elif output.returncode == 70:
                    f.write("\n# RTE: " + x)
            f.write("\n")
        # else:
        #     with open(file, "a") as f:
        #         for x in out:
        #             f.write("\n# expect: "+out)


def main():
    # clear screen
    os.system("cls" if os.name == "nt" else "clear")

    # get test cases files
    test_files = get_test_cases_file()
    # generate test cases for each file
    for test_file in test_files:
        print("Generating test cases for: " + str(test_file))
        clear(test_file, clear_test=True)
        generate_test(test_file)
        clear(test_file)


if __name__ == "__main__":
    main()
