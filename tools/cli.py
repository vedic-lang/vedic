import os
import sys
import subprocess
import argparse
import unit_test
import benchmark_test

ENV_VARBOSE = False


def clear():
    print("Clearing screen...")
    clear_command = "cls" if os.name == "nt" else "clear"
    output = subprocess.run([clear_command])
    return output.returncode


def build_vedic_cli():
    print("Building vedic...")
    output = subprocess.run(["cargo", "build", "--package", "vedic", "--release"])
    return output.returncode


def build_vedic_wasm():
    print("Building Wasm...")
    output = subprocess.run(["wasm-pack", "build", "wasm", "--target", "no-modules"])
    return output.returncode


def format_code():
    print("Formatting code...")
    output = subprocess.run(["cargo", "+nightly", "fmt"])
    return output.returncode


def validate_return_code(returncode):
    if returncode != 0:
        print("\nFailed")
        sys.exit(1)


def start_server(port):
    print("Starting web server...")
    output = subprocess.run(
        [sys.executable, "-m", "http.server", "--bind", "localhost", str(port)]
    )
    return output.returncode


def run():
    # get arguments
    parser = argparse.ArgumentParser()
    parser.add_argument("-c", "--clear", action="store_true", help="clear screen")
    parser.add_argument("-f", "--format", action="store_true", help="format code")
    parser.add_argument("-b", "--build", action="store_true", help="build Executable")
    parser.add_argument("-w", "--wasm", action="store_true", help="build Wasm Package")
    parser.add_argument("-t", "--test", action="store_true", help="run test cases")
    parser.add_argument("--bench", action="store_true", help="run benchmarks")
    parser.add_argument(
        "--serve",
        help="Start Web Server for Wasm",
        type=int,
        metavar="PORT",
    )
    parser.add_argument("--varbose", action="store_true", help="varbose mode")
    args = parser.parse_args()

    if args.varbose:
        print(args)
        ENV_VARBOSE = True

    if args.format:
        validate_return_code(format_code())

    if args.clear:
        validate_return_code(clear())

    if args.build:
        validate_return_code(build_vedic_cli())

    if args.wasm:
        validate_return_code(build_vedic_wasm())

    if args.test:
        validate_return_code(unit_test.run_tests())

    if args.bench:
        validate_return_code(benchmark_test.run())

    if args.serve:
        validate_return_code(start_server(args.serve))

    if not any(vars(args).values()):
        validate_return_code(parser.print_help())


if __name__ == "__main__":
    run()
