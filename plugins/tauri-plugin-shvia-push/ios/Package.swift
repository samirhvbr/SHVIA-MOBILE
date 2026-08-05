// swift-tools-version:5.5
import PackageDescription

let package = Package(
  name: "tauri-plugin-shvia-push",
  platforms: [
    .iOS(.v15)
  ],
  products: [
    .library(
      name: "tauri-plugin-shvia-push",
      type: .static,
      targets: ["tauri-plugin-shvia-push"])
  ],
  dependencies: [
    .package(name: "Tauri", path: "../.tauri/tauri-api")
  ],
  targets: [
    .target(
      name: "tauri-plugin-shvia-push",
      dependencies: [
        .byName(name: "Tauri")
      ],
      path: "Sources")
  ]
)
