cask "localgloss" do
  version "0.3.0"
  sha256 "5295153de4f20e09dc602cc3b3feda4c143be39d8ca4f28144c6a911a53e64d8"

  url "https://github.com/songsongsongggg/LocalGloss/releases/download/v#{version}/LocalGloss-v#{version}-macOS-arm64.zip"
  name "LocalGloss"
  desc "Offline Chinese pinyin input method with English glosses"
  homepage "https://github.com/songsongsongggg/LocalGloss"

  conflicts_with cask: "localgloss@alpha"
  depends_on arch: :arm64
  depends_on macos: :ventura

  input_method "LocalGloss-v#{version}-macOS-arm64/LocalGloss.app"

  caveats <<~EOS
    Add 本地译词 in System Settings > Keyboard > Text Input after installation.
    This build is ad-hoc signed and not notarized; keep Gatekeeper enabled.
    Before upgrading, save your work, switch to a system input source,
    back up LocalGloss.app and quit the old input method yourself.
    Manual installations must be moved out of the target directory first.
    Personal settings are preserved when uninstalling this cask.
  EOS
end
