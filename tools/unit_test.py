import os
import pathlib
import subprocess
import re
from time import process_time

# from rich import print

# global variables
result = []
current_result = {}
this_dir = pathlib.Path(__file__).parents[1]
# path to binary
VEDIC = this_dir.joinpath("target/release/vedic")
if os.name == "nt":
    VEDIC = VEDIC.with_suffix(".exe")
VEDIC = VEDIC.resolve()


# custom log function to store output
def logger(*logs):
    for log in logs:
        current_result["memory"].append(log)


# custom print result function
def flush_result():
    p, f = 0, 0
    for res in result:
        p += res["P"]
        f += res["F"]
        if res["F"] > 0:
            for log in res["memory"]:
                print(log)
    print(f"\nTotal Result : Passed: {p} Failed: {f}")
    return f


# assert equal test cases
def assert_equal(expected, actual, msg=None):
    if expected == actual:
        current_result["P"] += 1
    else:
        logger(f"\n\tResone : {msg}")
        if isinstance(expected, list):
            logger("\tExpected:")
            for i in expected:
                logger(f"\t>>>\t{i}")
            logger("\tGot:")
            for i in actual:
                logger(f"\t>>>\t{i}")
        else:
            logger(f"\tExpected: {expected}\n\tgot: {actual if actual else 'None'}")
        current_result["F"] += 1


# assert unimplemented test cases
def assert_unimplemented(case=False, msg=None):
    if case:
        current_result["P"] += 1
    else:
        logger(f"\tscenario : {msg}\n\n")
        current_result["F"] += 1


# Create test cases from tests folder
def get_test_cases():
    tests_path = pathlib.Path("tests")
    tests = sorted(pathlib.Path(tests_path).glob("*/*.ved"))
    return tests


# parse comments from test files
def parse_comments(path):
    expect = {
        "in": [],
        "out": [],
        "compiletime_err": [],
        "runtime_err": [],
    }
    with open(path, "r") as content_file:
        content = content_file.read()
        for line in content.splitlines():
            m = re.search(r"# input: ?(.*)", line)
            if m:
                s = m.group(1)
                expect["in"].append(s)

            m = re.search(r"# expect: ?(.*)", line)
            if m:
                s = m.group(1)
                expect["out"].append(s)

            m = re.search(r"# CTE: (.+)", line)
            if m:
                msg = m.group(1)
                expect["compiletime_err"].append(msg)

            m = re.search(r"# RTE: (.+)", line)
            if m:
                msg = m.group(1)
                expect["runtime_err"].append(msg)
    if os.name == "nt":
        # replace each compile error forward slash with back slash in paths
        expect["compiletime_err"] = [
            x.replace("/", "\\") for x in expect["compiletime_err"]
        ]
        expect["runtime_err"] = [x.replace("/", "\\") for x in expect["runtime_err"]]
    return expect


# run test cases for a file
def run_test(file):
    # details = file.relative_to(this_dir)
    current_result["T"] = file
    current_result["P"] = 0
    current_result["F"] = 0
    current_result["memory"] = []
    print(f"Testing {str(file).ljust(50)} :", end="")
    logger(f"\nDebug {file} :")

    # parse comments in test file
    expected = parse_comments(file)

    # test cases
    have_runtime_err = len(expected["runtime_err"]) > 0
    have_compiletime_err = len(expected["compiletime_err"]) > 0
    have_input = len(expected["in"]) > 0

    if have_input:
        input = "\n".join(expected["in"]).encode("utf-8")
    else:
        input = None

    output = subprocess.run(
        [VEDIC, file],
        input=input,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    out = [x.decode("utf-8").strip() for x in output.stdout.split(b"\n")[:-1]]
    err = [x.decode("utf-8").strip() for x in output.stderr.split(b"\n")[:-1]]

    if not have_runtime_err and not have_compiletime_err:
        assert_unimplemented(
            output.returncode == 0, "Program exited with failure, expected success"
        )

    if have_runtime_err:
        assert_equal(
            70, output.returncode, "Program exited with success, expected failure"
        )

    if have_runtime_err:
        assert_equal(
            expected["runtime_err"],
            err,
            "Runtime error should match",
        )

    if have_compiletime_err:
        assert_equal(
            65, output.returncode, "Program exited with success, expected failure"
        )
        assert_equal(expected["compiletime_err"], err, "Compile error should match")

    for i, e in enumerate(expected["out"]):
        assert_equal(e, (out[i:] or [""])[0], "Output should match")

    print(f" Passed: {str(current_result['P']).ljust(2)} Failed: {current_result['F']}")
    result.append(current_result.copy())


def run_tests():
    # get test cases
    testcase = get_test_cases()

    # Start the stopwatch / counter
    t1_start = process_time()

    # run test cases
    for test in testcase:
        run_test(test)

    # Stop the stopwatch / counter
    t1_stop = process_time()

    # print result
    failed_count = flush_result()
    print("Elapsed time:", t1_stop - t1_start, "seconds")
    return failed_count


if __name__ == "__main__":
    run_tests()
