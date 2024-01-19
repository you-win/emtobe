# Emtobe

Markdown to BB Code utility for Godot. Converts markdown into Godot-compatible BB code.

# Translation List

List taken from [markdownguide.org](https://www.markdownguide.org/).

- Headings: `# text` -> `[font_size=N]text[/font_size]`
- Paragraphs: Nothing, these are just text!
- Line breaks: `text  ` -> `text\n`
- Emphasis
  - Bold: `**text**` or `__text__` -> `[b]text[/b]`
  - Italic: `*text*` or `_text_` -> `[i]text[/i]`
- Blockquotes: Not supported! Could theoretically be supported by inserting `CheckBox` controls.
- Lists:
  - Ordered: `1. text` -> `[ol=1]text[/ol]`
  - Unordered: `- text` -> `[ul]text[/ul]`
- Code: `\`text\`` -> `[code]text[/code]`
- Horizontal rules: Not supported! Could theoretically be supported by insert `HSeparator` controls.
- Links: `[word](link)` -> `[url=link]word[/url]`. Hints and angle bracket syntax are supported.
- Images: `![alt text](path)` -> `[hint=alt text][img]path[/img][/hint]`
- HTML: Not supported and will likely never be supported.
- Tables: Supported, no example will be given :)
- Strikethrough: `~~text~~` -> `[s]text[/s]`
- Other markdown extensions: Not supported.

# Building

1. Install a [Rust compiler](https://www.rust-lang.org/)
2. Clone this repo
3. Run `cargo build --release`
4. Copy the `.dll` or `.so` file under `./target/release/` to your Godot project
5. Copy `emtobe.gdextension` to your Godot project (change the path to the compiled binary to match)
6. Use either `Emtobe` or `MarkdownLabel` in your project

# License

MPL-2.0

- 
