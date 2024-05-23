import requests
import json

github_repo = "https://github.com/vedic-lang/vedic"


def get_tag_number() -> str:
    url = "https://api.github.com/repos/vedic-lang/vedic/releases/latest"
    response = requests.get(url)
    data = json.loads(response.text)
    return data["tag_name"].replace("v", "")


tag = get_tag_number()


def get_sha_values() -> dict:
    url = f"{github_repo}/releases/download/v{tag}/sha256sums.txt"
    response = requests.get(url)
    sha_values = {
        each.split("/")[-1].strip(): each.split(" ")[0].strip()
        for each in str(response.text).split("\n")
    }
    print(url, response, sha_values)
    return sha_values


sha_values = get_sha_values()


# DEFUALT
version = "{version}"
bin = "{bin}"
homepage = "{homepage}"

tap_code = f"""class vedic < Formula
  desc "vedic-lang is a Sanskrit programming language"
  homepage "{github_repo}"
  version "{tag}"
  license "MIT"
  head "{github_repo}.git", branch: "main"

  if OS.mac?
    if Hardware::CPU.arm?
      url "{github_repo}/releases/download/v#{version}/vedic-darwin-aarch64.tar.gz"
      sha256 "{sha_values.get('vedic-darwin-aarch64.tar.gz','')}"
    elsif Hardware::CPU.intel?
      url "{github_repo}/releases/download/v#{version}/vedic-darwin-x86_64.tar.gz"
      sha256 "{sha_values.get('vedic-darwin-aarch64.tar.gz','')}"
    else
      # Unsupported platform
      odie "This platform is not supported."
    end
  elsif OS.linux?
    if Hardware::CPU.arm? && Hardware::CPU.is_64_bit?
      url "{github_repo}/releases/download/v#{version}/vedic-linux-gnu-aarch64.tar.xz"
      sha256 "{sha_values.get('vedic-darwin-aarch64.tar.gz','')}"
    elsif Hardware::CPU.intel? && Hardware::CPU.is_64_bit?
      url "{github_repo}/releases/download/v#{version}/vedic-linux-gnu-x86_64.tar.xz"
      sha256 "{sha_values.get('vedic-darwin-aarch64.tar.gz','')}"
    elsif Hardware::CPU.intel? && Hardware::CPU.is_32_bit?
      url "{github_repo}/releases/download/v#{version}/vedic-linux-gnu-i686.tar.xz"
      sha256 "{sha_values.get('vedic-darwin-aarch64.tar.gz','')}"
    elsif Hardware::CPU.arm? && Hardware::CPU.is_32_bit?
      url "{github_repo}/releases/download/v#{version}/vedic-linux-gnueabihf-armv7.tar.xz"
      sha256 "{sha_values.get('vedic-darwin-aarch64.tar.gz','')}"
    elsif Hardware::CPU.intel? && Hardware::CPU.is_64_bit? && OS::Linux::Musl.is_detected?
      url "{github_repo}/releases/download/v#{version}/vedic-linux-musl-x86_64.tar.xz"
      sha256 "{sha_values.get('vedic-darwin-aarch64.tar.gz','')}"
    else
      # Unsupported platform
      odie "This platform is not supported."
    end
  else
    # Unsupported platform
    odie "This platform is not supported."
  end

  def install
    bin.install "vedic"  # Install the binary
  end

  def caveats
    <<~EOS
      To get started, run:
        vedic --help
    EOS
  end

  test do
    # Simple test to verify the binary is installed
    assert_match "Usage:", shell_output("#{bin}/vedic --help")
  end
end"""


with open("pyspark-schema-generator/vedic.rb", "w", encoding="utf-8") as f:
    f.write(tap_code)
