<div align="center">

# Alepc | Aleecers post cli
CLI to create/modify post template for [aleecers blog]

<a href="https://www.gnu.org/licenses/">
  <img src="https://img.shields.io/badge/license-GPLv3-orange.svg" alt="License">
</a>
<a href="https://rust-lang.org/">
  <img src="https://img.shields.io/badge/Made%20with-Rust-orange.svg" alt="Rust">
</a>
<br>
<a href="https://crates.io/crates/alepc">
    <img src="https://img.shields.io/crates/v/alepc.svg">
  </a>
<br>
<a href="https://github.com/aleecers/alepc/actions/workflows/ci.yml">
  <img src="https://github.com/aleecers/alepc/actions/workflows/ci.yml/badge.svg" alt="Continuous Integration">
</a>
<br>
<a href="https://github.com/aleecers/alepc/actions/workflows/release.yml">
  <img src="https://github.com/aleecers/alepc/actions/workflows/release.yml/badge.svg" alt="Release">
</a>

</div>

## Features
- Add post to posts directory
- Copy image to images directory
- Rename image to "{post-slug}-header.{extension}"
- Easy to use
- Configuration file ( [RON] )
- Save configuration file in config system directory [`#6`], [`#2`], [`#9`]
- The ability to show the [logs](#Logging) [`#5`], [`#11`]
- Modify [`#24`]
  - Title
  - Description
  - Slug (Will rename the post file, images folder, link, header. in one click 🦀)
  - Post header (Just enter the new image and will copy it to images directory and rename it 💪)
  - Last modified date (in one click)
  - Draft status (in one click)


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

## Logging
To show logs run Alepc with [`RUST_LOG`] environment variable with [`trace`] value

## Configuration
Alepc will create empty configuration file in config system directory, to know where its see [`#2`].
You can change the configuration file to your needs.
Below is the table of configuration you can used in configuration file, the examples is whoe you can use it in configuration file.

### Global configuration
| Key | Type | Description | Default |
| --- | --- | --- | --- |
|`posts_path`| String | Path of posts |`../Aleecers.github.io/src/pages/blog/`|
|`images_path`| String | Path to images directory |`../Aleecers.github.io/public/images/`|
|`blog_site_path`| String | Path of blog in the site |`/blog/`|
|`images_site_path`| String | Path of images in the site |`/images/`|
|`posts_layout`| String | Layout path of posts ( path start from `posts_path` ) |`../../layouts/blog.astro`|
|`repository_url`| String | Repository url |`https://github.com/aleecers/alepc`|
|`date_format`| String | Date format |`%Y/%m/%d`|

#### Example
> Note: The first parentheses belong to the file itself, if you want to use the above configuration in a file that already has configurations added, delete the first parentheses
```ron
(
    posts_path: "../Aleecers.github.io/src/pages/blog/",
    images_path: "../Aleecers.github.io/public/images/",
    blog_site_path: "/blog/",
)
```

### `select_action` configuration
| Key | Type | Description | Default |
| --- | --- | --- | --- |
|`select_action_message`| String | The question of select action |`What do you want to do ❓`|
|`new_post_choice`| String | Create a new post choice in select |`Create a new post ✍`|
|`update_existing_post`| String | Update an existing post choice |`Update existing post 🖌️`|
|`version_choice`| String | Version choice |`Alepc Version ⚙`|

#### Example
> Note: The first parentheses belong to the file itself, if you want to use the above configuration in a file that already has configurations added, delete the first parentheses
```ron
(
  select_action: (
      select_action_message: "What do you want to do ❓",
      new_post_choice: "Create a new post ✍",
  )
)
```

### `create_post_settings` configuration
| Key | Type | Description | Default |
| --- | --- | --- | --- |
|`minimum_title_length`| Integer | Minimum length of post title|`7`|
|`maximum_title_length`| Integer | Maximum length of post title|`30`|
|`title_message`| String | Ask for post title message |`Title of post 📝`|
|`minimum_description_length`| Integer | Minimum length of post description|`10`|
|`maximum_description_length`| Integer | Maximum length of post description|`255`|
|`description_message`| String | Ask for post description message |`Description of post 📝`|
|`minimum_tags_count`| Integer | Minimum tags on post|`1`|
|`maximum_tags_count`| Integer | Maximum tags on post|`3`|
|`tags_message`| String | Ask for post tags message |`Tags of post (separated by comma)`|
|`separated_tags_by`| Char | separated tags by | `,` |
|`minimum_single_tag_length`| Integer | Minimum single tag length|`3`|
|`maximum_single_tag_length`| Integer | Maximum single tag length|`8`|
|`slug_message`| String | Ask for post slug message |`Slug of post`|
|`minimum_slug_length`| Integer | Minimum length of post slug|`5`|
|`maximum_slug_length`| Integer | Maximum length of post slug|`20`|
|`image_message`| String | Ask for post image message |`Image of post`|

#### Example
> Note: The first parentheses belong to the file itself, if you want to use the above configuration in a file that already has configurations added, delete the first parentheses
```ron
(
  create_post_settings: (
      minimum_title_length: 7,
      maximum_title_length: 30,
      title_message: "Title of post 📝",
  )
)
```

### `modify_post_settings` configuration
| Key | Type | Description | Default |
| --- | --- | --- | --- |
|`post_name_question`| String | The question of post name |`What's the post you want to modify it (Write the slug)`|
|`choice_action`| String | Choice modify action |`What do you want to update?`|
|`update_the_date_question`| String | Update date question |`Update modified date`|
|`update_draft_status_question`| String | Update draft status question ( Will add the currently status in the end) |`Update draft status`|
|`show_all_question`| String | Show all fields to update it question |`Show all`|
|`new_post_slug`| String | New post slug question (Wheen show_all) |`New post slug`|
|`new_post_title`| String | New post title question (Wheen show_all) |`New post title`|
|`new_post_descrioption`| String | New post descripation question (Wheen show_all) |`New post description`|
|`new_post_image`| String | New post image question (Wheen show_all) |`New post image`|
|`new_post_tags`| String | New post tags question (Wheen show_all) |`New post tags`|
|`new_post_draft`| String | New post draft status question (Wheen show_all) |`Do you want to change draft status?`|
|`keep_old_value_message`| String | Message to keep old value |`Press enter to keep it 🤏`|

#### Example
> Note: The first parentheses belong to the file itself, if you want to use the above configuration in a file that already has configurations added, delete the first parentheses
```ron
(
  modify_post_settings: (
      post_name_question: "What's the post you want to modify it (Write the slug)",
      choice_action: "What do you want to update?",
      update_the_date_question: "Update modified date",
  )
)
```

## Images

| Home                                      | Creation                                     |
|-------------------------------------------|--------------------------------------------|
| ![home-screen](https://i.suar.me/05Xmr/l) | ![creation-image](https://i.suar.me/G5KA9/s) |

| Modification                              |Modify all                                     |
|-------------------------------------------|--------------------------------------------|
| ![modification-screen](https://i.suar.me/XovgB/l)| ![creation-image](https://i.suar.me/55P52/l) |


## License
GNU General Public License version 3 of the license for more see <https://www.gnu.org/licenses/>


[aleecers blog]: https://github.com/aleecers/Aleecers.github.io
[RON]: https://github.com/ron-rs/ron
[release page]: https://github.com/aleecers/alepc/releases/latest
[Cargo]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[`trace`]: https://docs.rs/log/latest/log/enum.Level.html#variant.Trace
[`RUST_LOG`]: https://docs.rs/env_logger/0.9.0/env_logger/#enabling-logging
[`#2`]: https://github.com/Aleecers/alepc/issues/2
[`#6`]: https://github.com/Aleecers/alepc/issues/6
[`#9`]: https://github.com/Aleecers/alepc/pull/9
[`#5`]: https://github.com/Aleecers/alepc/issues/5
[`#11`]: https://github.com/Aleecers/alepc/pull/11
[`#24`]: https://github.com/Aleecers/alepc/pull/24
