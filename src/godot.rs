use godot::{
    engine::{node::InternalMode, IRichTextLabel, RichTextLabel},
    prelude::*,
};

use crate::Emtobe;

struct GodotExtension {}

#[gdextension]
unsafe impl ExtensionLibrary for GodotExtension {}

#[derive(Debug, GodotClass)]
#[class(rename = Emtobe)]
struct EmtobeBinding {
    emtobe: Emtobe,
}

#[godot_api]
impl IRefCounted for EmtobeBinding {
    fn init(_: godot::obj::Base<Self::Base>) -> Self {
        Self {
            emtobe: Emtobe::new(),
        }
    }
}

#[godot_api]
impl EmtobeBinding {
    #[func]
    fn parse(&mut self, text: String) -> String {
        self.emtobe.parse(text).unwrap_or("parsing error".into())
    }
}

#[derive(Debug, GodotClass)]
#[class(base = RichTextLabel, tool)]
struct MarkdownLabel {
    emtobe: Emtobe,

    #[export(multiline)]
    #[var(get = get_markdown, set = set_markdown)]
    markdown: GString,
    #[base]
    base: Base<RichTextLabel>,
}

#[godot_api]
impl IRichTextLabel for MarkdownLabel {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            emtobe: Emtobe::new(),

            markdown: GString::new(),
            base,
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_use_bbcode(true);
    }
}

#[godot_api]
impl MarkdownLabel {
    #[func]
    fn get_markdown(&self) -> GString {
        self.markdown.clone()
    }

    #[func]
    fn set_markdown(&mut self, markdown: GString) {
        self.markdown = markdown;
        self.refresh();
    }

    #[func]
    fn append(&mut self, text: String) {
        match self.emtobe.parse(&text) {
            Ok(v) => {
                self.markdown = format!("{}{text}", self.markdown).into();
                self.base_mut().append_text(v.into());
            }
            Err(e) => godot_error!("{e}"),
        }
    }

    #[func]
    fn refresh(&mut self) {
        match self.emtobe.parse(self.markdown.to_string()) {
            Ok(v) => self.base_mut().set_text(v.into()),
            Err(e) => godot_error!("{e}"),
        }
    }
}
