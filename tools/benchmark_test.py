#!/bin/python3
import os
import sys
import json
import pathlib
import subprocess
from datetime import datetime


def vedic_path():
    this_dir = pathlib.Path(__file__).parents[1]
    # path to binary
    VEDIC = this_dir.joinpath("target/release/vedic")
    if os.name == "nt":
        VEDIC = VEDIC.with_suffix(".exe")
    return VEDIC.resolve()


# path to compilers
COMPILE_PATH = {
    "nodejs": "node",
    "python3": sys.executable,
    "vedic": vedic_path(),
}


# Create test cases from tests folder
def get_test_cases(lang):
    tests_path = pathlib.Path(f"benchmarks/{lang}")
    tests = sorted(pathlib.Path(tests_path).glob("*"))
    return tests


# run test cases
def run_test(lang, REPETITION):
    binary = COMPILE_PATH[lang]
    print(f"\nRunning {lang} tests...")
    tests = get_test_cases(lang)
    result = []
    for benchmark in tests:
        times = []
        print(f"\n{benchmark} =>", end=" ")
        for i in range(REPETITION):
            # Start the stopwatch in ms
            t_start = datetime.timestamp(datetime.now()) * 1000
            # run
            test_job = subprocess.run(
                [str(binary), str(benchmark.resolve())],
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )

            # Stop the stopwatch
            t_stop = datetime.timestamp(datetime.now()) * 1000
            if test_job.returncode == 0:
                times.append(t_stop - t_start)
                print("✔", end=" ")
            else:
                print("✖", end=" ")
                break
        if len(times) != 0:
            avg_time = sum(times) / len(times)
        else:
            avg_time = None
        result.append(
            {"lang": lang, "test": benchmark.name.split(".")[0], "time": avg_time}
        )
    return result


def get_data():
    compiler_names = ["vedic", "python3", "nodejs"]
    result = []
    for lang in compiler_names:
        test_result = run_test(lang, 3)
        result += test_result
    return result


def run():
    t_start = datetime.timestamp(datetime.now())
    data = get_data()
    print(data)
    t_stop = datetime.timestamp(datetime.now())
    print("\ntotal", t_stop - t_start)
    json.dump(data, open("benchmarks/data.json", "w"), indent=4, allow_nan=True)


if __name__ == "__main__":
    run()
