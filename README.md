# Alepc | Aleecers post cli

CLI to create post template for [aleecers blog]

## Features
- Add post to posts directory
- Copy image to images directory
- Rename image to "{post-slug}-header.{extension}"
- Easy to use
- Configuration file ( [RON] )

## Installation
### Assets

You can install Alepc from [release page]

### Cargo
You can install Alepc by [Cargo] with following command
```bash
cargo install alepc
```

### From source code
To compile Alepc from source code, you need [Cargo]
```bash
git clone https://github.com/aleecers/alepc
cd alepc
cargo build --release
```
after the build you can find binary file in `./target/release/alepc`

## Images

| Home                                      | Create                                     |
|-------------------------------------------|--------------------------------------------|
| ![home-screen](https://i.suar.me/05Xmr/l) | ![create-image](https://i.suar.me/G5KA9/s) |


## License
GNU General Public License version 3 of the license for more see <https://www.gnu.org/licenses/>


[aleecers blog]: https://github.com/aleecers/Aleecers.github.io
[RON]: https://github.com/ron-rs/ron
[release page]: https://github.com/aleecers/alepc/releases/latest
[Cargo]: https://doc.rust-lang.org/cargo/getting-started/installation.html