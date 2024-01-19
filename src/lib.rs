mod godot;

use std::fmt::Display;

use pulldown_cmark::{Alignment, CowStr, Event, HeadingLevel, LinkType, Options, Parser, Tag};

#[derive(Debug)]
enum EmtobeError {}

impl Display for EmtobeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug)]
struct TableBuilder {
    column_count: usize,

    rows: Vec<Vec<String>>,
    row_builder: Vec<String>,

    cell_builder: Option<String>,
}

impl TableBuilder {
    fn new(column_count: usize) -> Self {
        Self {
            column_count,

            rows: vec![],
            row_builder: vec![],

            cell_builder: None,
        }
    }

    fn push_cell(&mut self, text: String) {
        self.cell_builder = Some(text);
    }

    fn finish_cell(&mut self) {
        if let Some(cell) = self.cell_builder.take() {
            self.row_builder.push(cell);
        } else {
            self.row_builder.push("".into());
        }
    }

    fn build_row(&mut self) {
        assert_eq!(
            self.row_builder.len(),
            self.column_count,
            "Row builder column count didn't match table column count"
        );
        self.rows.push(self.row_builder.drain(..).collect());
    }

    fn build_table(self) -> String {
        let mut builder = vec![];
        builder.push(format!("[table={}]\n", self.column_count));

        for row in self.rows {
            for col in row {
                builder.push(format!("[cell]{col}[/cell]"));
            }
            builder.push("\n".into());
        }

        builder.push("[/table]\n".into());
        builder.into_iter().collect()
    }
}

/// https://www.markdownguide.org/basic-syntax/
#[derive(Debug)]
struct Emtobe {
    builder: Vec<String>,
    table_builder: Option<TableBuilder>,
}

impl Emtobe {
    fn new() -> Self {
        Self {
            builder: vec![],
            table_builder: None,
        }
    }

    // TODO refactor to use a tag stack
    fn parse(&mut self, text: impl AsRef<str>) -> Result<String, EmtobeError> {
        let parser = Parser::new_ext(
            text.as_ref(),
            Options::ENABLE_STRIKETHROUGH | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES,
        );
        for event in parser {
            match event {
                Event::Start(tag) => self.handle_open_tag(tag),
                Event::End(tag) => self.handle_close_tag(tag),
                Event::Text(text) => {
                    if let Some(builder) = self.table_builder.as_mut() {
                        builder.push_cell(text.to_string());
                    } else {
                        self.builder.push(text.to_string());
                    }
                }
                Event::Code(text) => self.builder.push(text.to_string()),
                Event::Html(text) => self.builder.push(text.to_string()),
                Event::FootnoteReference(_) => {
                    unreachable!("Footnotes should be disabled, this is a bug")
                }
                Event::SoftBreak => self.builder.push(" ".into()),
                Event::HardBreak => self.builder.push("\n\n".into()),
                Event::Rule => todo!(),
                Event::TaskListMarker(_) => {
                    unreachable!("Task lists should be disabled, this is a bug")
                }
            }
        }

        // End with only 1 newline
        if let Some(v) = self.builder.last_mut() {
            if v.ends_with("\n\n") {
                v.pop();
            }
        }
        let output = self.builder.drain(..).collect::<String>();
        if self.table_builder.is_some() {
            self.table_builder = None;
        }

        Ok(output)
    }

    fn handle_open_tag(&mut self, tag: Tag<'_>) {
        let node: Option<String> = match tag {
            Tag::Paragraph => None,
            Tag::Heading(level, _, _) => Some(format!(
                "[font_size={}]",
                match level {
                    HeadingLevel::H1 => 36,
                    HeadingLevel::H2 => 24,
                    HeadingLevel::H3 => 18,
                    HeadingLevel::H4 => 12,
                    HeadingLevel::H5 => 10,
                    HeadingLevel::H6 => 8,
                }
            )),
            Tag::BlockQuote => Some("> ".into()),
            Tag::CodeBlock(_) => Some("[code]".into()),
            Tag::List(v) => Some(if let Some(_) = v {
                "[ol type=1]\n".into()
            } else {
                "[ul]\n".into()
            }),
            Tag::Item => None,
            Tag::FootnoteDefinition(_) => {
                unreachable!("Footnotes should be disabled, this is a bug")
            }
            Tag::Table(v) => {
                self.table_builder = Some(TableBuilder::new(v.len()));
                None
            }
            Tag::TableHead => None,
            Tag::TableRow => None,
            Tag::TableCell => None,
            Tag::Emphasis => Some("[i]".into()),
            Tag::Strong => Some("[b]".into()),
            Tag::Strikethrough => Some("[s]".into()),
            Tag::Link(link_type, url, title) => match link_type {
                LinkType::Autolink | LinkType::Email => Some("[url]".into()),
                LinkType::Inline => Some(if title.is_empty() {
                    format!("[url={url}]")
                } else {
                    format!("[hint={title}][url={url}]")
                }),
                _ => None,
            },
            Tag::Image(_, path, title) => Some(if !title.is_empty() {
                format!(
                    "[hint={}][img]{}[/img][/hint]",
                    title.to_string(),
                    path.to_string()
                )
            } else {
                format!("[img]{}[/img]", path.to_string())
            }),
        };

        if let Some(bb_code) = node {
            self.builder.push(bb_code);
        }
    }

    fn handle_close_tag(&mut self, tag: Tag<'_>) {
        let node: Option<String> = match tag {
            Tag::Paragraph => Some("\n\n".into()),
            Tag::Heading(_, _, _) => Some("[/font_size]\n\n".into()),
            Tag::BlockQuote => None,
            Tag::CodeBlock(_) => Some("[/code]".into()),
            Tag::List(v) => Some(if let Some(_) = v {
                "[/ol]\n\n".into()
            } else {
                "[/ul]\n\n".into()
            }),
            Tag::Item => Some("\n".into()),
            Tag::FootnoteDefinition(_) => {
                unreachable!("Footnotes should be disabled, this is a bug")
            }
            Tag::Table(_) => {
                let builder = self.table_builder.take();
                if let Some(builder) = builder {
                    Some(builder.build_table())
                } else {
                    unreachable!("Table should be present")
                }
            }
            Tag::TableHead | Tag::TableRow => {
                if let Some(builder) = self.table_builder.as_mut() {
                    builder.build_row();
                } else {
                    unreachable!("Table should be present");
                }
                None
            }
            Tag::TableCell => {
                if let Some(builder) = self.table_builder.as_mut() {
                    builder.finish_cell();
                } else {
                    unreachable!("Table should be present");
                }
                None
            }
            Tag::Emphasis => Some("[/i]".into()),
            Tag::Strong => Some("[/b]".into()),
            Tag::Strikethrough => Some("[/s]".into()),
            Tag::Link(_, _, title) => Some(if title.is_empty() {
                "[/url]".into()
            } else {
                "[/url][/hint]".into()
            }),
            Tag::Image(_, _, title) => Some(if title.is_empty() {
                "[/img]".into()
            } else {
                "[/img]".into()
            }),
        };

        if let Some(bb_code) = node {
            self.builder.push(bb_code);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let mut emtobe = Emtobe::new();

        let output = emtobe.parse(
            r"# Test Header

## Subheader

*Hello* **world**!

Goodbye.

- hello
- world

[godot](godotengine.org 'blah')

<https://godotengine.org>

New paragraph
with text on the next line but same paragraph.

| first | second | third |
| --- | --- | --- |
| hello | world | |
",
        );

        assert!(output.is_ok());
        assert_eq!(
            output.unwrap(),
            r"[font_size=36]Test Header[/font_size]

[font_size=24]Subheader[/font_size]

[i]Hello[/i] [b]world[/b]!

Goodbye.

[ul]
hello
world
[/ul]

[hint=blah][url=godotengine.org]godot[/url][/hint]

[url]https://godotengine.org[/url]

New paragraph with text on the next line but same paragraph.

[table=3]
[cell]first[/cell][cell]second[/cell][cell]third[/cell]
[cell]hello[/cell][cell]world[/cell][cell][/cell]
[/table]
"
        );
    }

    #[test]
    fn header() {
        assert_eq!(
            Emtobe::new().parse("# Hello World").unwrap(),
            "[font_size=36]Hello World[/font_size]\n"
        )
    }
}
