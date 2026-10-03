// 生成 LocalGloss 自有图标的全部分辨率；只写入传入的构建目录。
import AppKit
import Foundation

let destination = URL(fileURLWithPath: CommandLine.arguments[1], isDirectory: true)
try FileManager.default.createDirectory(at: destination, withIntermediateDirectories: true)
let variants = [(16, 1), (16, 2), (32, 1), (32, 2), (128, 1), (128, 2), (256, 1), (256, 2), (512, 1), (512, 2)]
for (points, scale) in variants {
    let pixels = points * scale
    let bitmap = NSBitmapImageRep(bitmapDataPlanes: nil, pixelsWide: pixels, pixelsHigh: pixels,
                                  bitsPerSample: 8, samplesPerPixel: 4, hasAlpha: true,
                                  isPlanar: false, colorSpaceName: .deviceRGB, bytesPerRow: 0, bitsPerPixel: 0)!
    let context = NSGraphicsContext(bitmapImageRep: bitmap)!
    NSGraphicsContext.saveGraphicsState()
    NSGraphicsContext.current = context
    context.cgContext.scaleBy(x: CGFloat(pixels) / 1024, y: CGFloat(pixels) / 1024)
    let background = NSBezierPath(roundedRect: NSRect(x: 80, y: 80, width: 864, height: 864), xRadius: 194, yRadius: 194)
    NSGradient(starting: NSColor(calibratedRed: 0.10, green: 0.22, blue: 0.32, alpha: 1),
               ending: NSColor(calibratedRed: 0.04, green: 0.10, blue: 0.17, alpha: 1))!.draw(in: background, angle: -90)
    NSColor.white.setFill()
    NSBezierPath(roundedRect: NSRect(x: 200, y: 350, width: 500, height: 430), xRadius: 98, yRadius: 98).fill()
    let tail = NSBezierPath()
    tail.move(to: NSPoint(x: 260, y: 385))
    tail.line(to: NSPoint(x: 260, y: 285))
    tail.line(to: NSPoint(x: 390, y: 385))
    tail.close()
    tail.fill()
    let chineseStyle: [NSAttributedString.Key: Any] = [
        .font: NSFont.systemFont(ofSize: 280, weight: .semibold),
        .foregroundColor: NSColor(calibratedRed: 0.07, green: 0.16, blue: 0.23, alpha: 1)
    ]
    let chinese: NSString = "文"
    let chineseSize = chinese.size(withAttributes: chineseStyle)
    chinese.draw(at: NSPoint(x: 450 - chineseSize.width / 2, y: 565 - chineseSize.height / 2), withAttributes: chineseStyle)
    NSColor(calibratedRed: 0.30, green: 0.88, blue: 0.70, alpha: 1).setFill()
    NSBezierPath(roundedRect: NSRect(x: 520, y: 220, width: 310, height: 310), xRadius: 85, yRadius: 85).fill()
    let englishStyle: [NSAttributedString.Key: Any] = [
        .font: NSFont.systemFont(ofSize: 235, weight: .bold),
        .foregroundColor: NSColor(calibratedRed: 0.04, green: 0.17, blue: 0.20, alpha: 1)
    ]
    let english: NSString = "A"
    let englishSize = english.size(withAttributes: englishStyle)
    english.draw(at: NSPoint(x: 675 - englishSize.width / 2, y: 375 - englishSize.height / 2), withAttributes: englishStyle)
    NSGraphicsContext.restoreGraphicsState()
    let suffix = scale == 2 ? "@2x" : ""
    try bitmap.representation(using: .png, properties: [:])!.write(to: destination.appendingPathComponent("icon_\(points)x\(points)\(suffix).png"))
}
