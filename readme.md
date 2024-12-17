# Cargo Rust project dioxus base

This is template structure for cargo projects using Dioxus framework with workspaces

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```sh
cargo generate --git https://github.com/Grapple228/rust-dioxus-base.git --name my-project
cd my-project
```

### Install `dioxus-cli`

Follow [this](https://dioxuslabs.com/learn/0.6/getting_started/) article to install dx on your os.

### Run application

[Learn more about `dioxus` here.](https://github.com/dioxuslabs/dioxus)

#### 1: Web

```sh
cd crates/apps/web
dx serve
```

#### 2: Desktop

```sh
cd crates/apps/desktop
dx serve
```

#### 3: Mobile (virtual)

```sh
emulator -avd Pixel_6a_API_35 -gpu host -netdelay none -netspeed full
cd crates/apps/mobile
dx serve --platform android
```

#### 4: Mobile (native)

```sh
# Turn on USB-debugging

# -- Connect usb to wsl (Optional)
usbipd bind --busid 2-2
usbipd attach --wsl --busid 2-2

# -- Build apk
cd crates/apps/mobile
dx build --platform android

# -- Install apk on device
adb install target/dx/mobile/debug/android/app/app/build/outputs/apk/debug/app-debug.apk
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [LINK](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE](LICENSE) or [LINK](http://opensource.org/licenses/MIT))

at your option.
