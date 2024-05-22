class Vedic < Formula
  desc "vedic-lang is a Sanskrit programming language"
  homepage "https://github.com/vedic-lang/vedic"
  version "2.0.6"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/vedic-lang/vedic/releases/download/v#{version}/vedic-darwin-aarch64.tar.gz"
      sha256 "69c2ec15825502f85478f21ce86ff64eade278a26ab70d4cf2a5299945d6e1a3"
    elsif Hardware::CPU.intel?
      url "https://github.com/vedic-lang/vedic/releases/download/v#{version}/vedic-darwin-x86_64.tar.gz"
      sha256 "f2622649af902dbc6909d1a1d2836a31cfd576daa50d0a5b1895cbc579962779"
    else
      # Unsupported platform
      odie "This platform is not supported."
    end
  elsif OS.linux?
    if Hardware::CPU.arm? && Hardware::CPU.is_64_bit?
      url "https://github.com/vedic-lang/vedic/releases/download/v#{version}/vedic-linux-gnu-aarch64.tar.xz"
      sha256 "0ed48b0af33f87421d0a60c5cffe831c8a1de388a103f42b666894a738fc0fff"
    elsif Hardware::CPU.intel? && Hardware::CPU.is_64_bit?
      url "https://github.com/vedic-lang/vedic/releases/download/v#{version}/vedic-linux-gnu-x86_64.tar.xz"
      sha256 "33b643c211121f5679636809e845354e672031814c85bb10527f4e5af4a7f18d"
    elsif Hardware::CPU.intel? && Hardware::CPU.is_32_bit?
      url "https://github.com/vedic-lang/vedic/releases/download/v#{version}/vedic-linux-gnu-i686.tar.xz"
      sha256 "affc9a338a50021b62505076ed9544df569f2430c207bf2f82c5595403c4c34c"
    elsif Hardware::CPU.arm? && Hardware::CPU.is_32_bit?
      url "https://github.com/vedic-lang/vedic/releases/download/v#{version}/vedic-linux-gnueabihf-armv7.tar.xz"
      sha256 "e4c3d44dcb75b63411e3fe844eb3c8f4e70c727a888bc2b27a6edf361b00eec4"
    elsif Hardware::CPU.intel? && Hardware::CPU.is_64_bit? && OS::Linux::Musl.is_detected?
      url "https://github.com/vedic-lang/vedic/releases/download/v#{version}/vedic-linux-musl-x86_64.tar.xz"
      sha256 "3b8fd6f14b7da520d06190430fbf5870418e6d93f1afc6b87f54cc1d7a4fb30b"
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
end
