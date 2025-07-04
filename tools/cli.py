import os
import sys
import shutil
import argparse
import subprocess

import unit_test
import benchmark_test

ENV_VERBOSE = False


def clear():
    print("Clearing screen...")
    clear_command = "cls" if os.name == "nt" else "clear"
    output = subprocess.run(clear_command, shell=True)
    return output.returncode


def check_and_prompt_rust_tools():
    required_tools = {
        "cargo": (
            "Install Rust (includes `cargo` and `rustc`):\n"
            "  Website: https://www.rust-lang.org/tools/install\n"
            "  Shell command:\n"
            "    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        ),
        "wasm-pack": (
            "Install wasm-pack:\n"
            "  Website: https://rustwasm.github.io/wasm-pack/installer/\n"
            "  Shell command:\n"
            "    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
        ),
        "pre-commit": (
            "Install pre-commit (Python Git hook manager):\n"
            "  Website: https://pre-commit.com/\n"
            "  pip install command:\n"
            "    pip install pre-commit"
        ),
    }

    missing = []

    for tool, install_hint in required_tools.items():
        if shutil.which(tool) is None:
            print(f"'{tool}' is not installed.\n{install_hint}\n")
            missing.append(tool)
        else:
            try:
                version = subprocess.check_output(
                    [tool, "--version"], text=True
                ).strip()
                print(f"Found {tool}: {version}")
            except Exception as e:
                print(f"Could not verify version of '{tool}': {e}")

    if missing:
        print("\nPlease install the missing tools listed above and re-run the script.")
        sys.exit(1)
    else:
        print("All required Rust tools are installed.\n")


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
    output_fmt = subprocess.run(["cargo", "+nightly", "fmt"])
    output_precommit = subprocess.run(["pre-commit", "run", "--all-files"])
    return output_fmt.returncode or output_precommit.returncode


def validate_return_code(returncode):
    if returncode != 0:
        print("Command failed.")
        sys.exit(1)


def start_server(port):
    print(f"Starting web server on port {port}...")
    output = subprocess.run(
        [sys.executable, "-m", "http.server", "--bind", "localhost", str(port)]
    )
    return output.returncode


def run():
    # Check rust tools at start
    check_and_prompt_rust_tools()

    # Get arguments
    parser = argparse.ArgumentParser()
    parser.add_argument("-c", "--clear", action="store_true", help="clear screen")
    parser.add_argument(
        "-f", "--format", action="store_true", help="format code and run pre-commit"
    )
    parser.add_argument("-b", "--build", action="store_true", help="build executable")
    parser.add_argument("-w", "--wasm", action="store_true", help="build Wasm package")
    parser.add_argument("-t", "--test", action="store_true", help="run test cases")
    parser.add_argument("-B", "--bench", action="store_true", help="run benchmarks")
    parser.add_argument(
        "-s", "--serve", type=int, metavar="PORT", help="start web server for Wasm"
    )
    parser.add_argument("--v", "--verbose", action="store_true", help="verbose mode")
    args = parser.parse_args()

    global ENV_VERBOSE
    if args.v:
        print("Verbose mode enabled.")
        print(args)
        ENV_VERBOSE = True

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
        parser.print_help()
        sys.exit(1)


if __name__ == "__main__":
    run()
