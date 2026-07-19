class Snag < Formula
  desc "A metrics/data collection tool for enterprises (multiple organizations, tools and repositories"
  homepage "https://github.com/grahambrooks/snag"
  version "2026.7.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/grahambrooks/snag/releases/download/v2026.7.0/snag-v2026.7.0-aarch64-apple-darwin.tar.gz"
      sha256 "e24ed636cc4bd8d23579b0359bc3593fc10111ea2950c4db8f1ac0389772bdfc"
    end
    on_intel do
      odie "Intel Mac binaries are not provided. Run `cargo install --git https://github.com/grahambrooks/snag --locked` to build from source."
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/grahambrooks/snag/releases/download/v2026.7.0/snag-v2026.7.0-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "dababb1c8d378621b8cd76887d609a64c3ad2ae31865942b08cc46e2e7424b0c"
    end
    on_intel do
      url "https://github.com/grahambrooks/snag/releases/download/v2026.7.0/snag-v2026.7.0-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "35de0e627eeecaf45d669d68269271f6f81413b88f43ad0fab79faeddcb1a83d"
    end
  end

  def install
    bin.install "snag"
  end

  test do
    assert_path_exists bin/"snag"
  end
end
