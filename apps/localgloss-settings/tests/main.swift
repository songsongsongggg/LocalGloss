//! 在隔离临时目录验证用户主动保存路径和原子替换，绝不读写真实偏好。
import Foundation
import Darwin
umask(0o077)
let directory = FileManager.default.temporaryDirectory.appendingPathComponent("localgloss-settings-test-" + UUID().uuidString)
var model = SettingsModel()
model.terms = [UserTerm(code: "kf", text: "开发", gloss: "develop")]
try model.save(to: directory)
let file = directory.appendingPathComponent("settings.json")
let first = try Data(contentsOf: file)
let restored = try SettingsModel.load(from: file)
assert(restored.terms.first?.gloss == "develop")
let mode = try FileManager.default.attributesOfItem(atPath: file.path)[.posixPermissions] as! NSNumber
assert(mode.intValue == 0o600)
model.font_size = 22
try model.save(to: directory)
let updated = try JSONDecoder().decode(SettingsModel.self, from: Data(contentsOf: file))
assert(updated.font_size == 22)
let good = try Data(contentsOf: file)
model.terms.append(model.terms[0])
do { try model.save(to: directory); fatalError("duplicate code accepted") } catch {}
let afterFailure = try Data(contentsOf: file)
assert(afterFailure == good)
let symlinkDirectory = directory.appendingPathComponent("symlink-target")
try FileManager.default.createSymbolicLink(at: symlinkDirectory, withDestinationURL: directory)
model.terms.removeLast()
do { try model.save(to: symlinkDirectory); fatalError("symlink accepted") } catch {}
let futureFile = directory.appendingPathComponent("future.json")
var object = try JSONSerialization.jsonObject(with: good) as! [String: Any]
object["future_setting"] = true
try JSONSerialization.data(withJSONObject: object).write(to: futureFile)
do { _ = try SettingsModel.load(from: futureFile); fatalError("unknown field accepted") } catch {}
print("settings tests passed: round trip, 0600 permissions, replacement, invalid-save preservation, symlink rejection, unknown-field rejection")
