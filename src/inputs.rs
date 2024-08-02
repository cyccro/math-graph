use gtk4::{prelude::*, Box, Button, Entry, EntryBuffer, Orientation};

use crate::helpers::parent_of;

pub struct Input {
    text: std::rc::Rc<std::cell::RefCell<String>>,
    entry: Entry,
}
impl Input {
    pub fn new(placeholder: &str) -> Self {
        let txt = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
        let clone = txt.clone();
        let clone_2 = txt.clone();
        let entry = Entry::with_buffer(&EntryBuffer::new(Some(placeholder)));
        if let Some(entry) = entry.delegate() {
            entry.connect_insert_text(move |_a, b, _c| {
                clone.borrow_mut().push_str(b);
            });
            entry.connect_delete_text(move |_, _, _| {
                clone_2.borrow_mut().pop();
            });
        }
        Self { text: txt, entry }
    }
    pub fn link_f(&self, f: impl Fn(String, char) + 'static) {
        let content_clone = self.text.clone();
        if let Some(entry) = self.entry.delegate() {
            entry.connect_insert_text(move |_, char, _| {
                f(
                    content_clone.borrow().to_string(),
                    char.chars().next().unwrap(),
                );
            });
        }
    }
    pub fn content(&self) -> String {
        self.text.borrow().to_string()
    }
    pub fn gtk(&self) -> Entry {
        self.entry.clone()
    }
}

pub struct FormulaInput {
    gtk: Box,
    input: Input,
}

impl FormulaInput {
    pub fn new(orientation: Orientation, space: i32) -> Self {
        let gtk = Box::new(orientation, space);
        let input = Input::new("");
        gtk.append(&input.gtk());
        Self { gtk, input }
    }
    pub fn content(&self) -> String {
        self.input.content()
    }
    pub fn gtk(&self) -> Box {
        self.gtk.clone()
    }
}
pub struct Inputs<'a> {
    inputs: Vec<&'a FormulaInput>,
    gtk: Box,
}

impl<'a> Inputs<'a> {
    pub fn new(distance: i32, orientation: Orientation) -> Self {
        let gtk = Box::new(orientation, distance);
        gtk.append(&{
            let btn = Button::with_label("Add");
            btn.connect_clicked(|btn| {
                if let Some(parent) = parent_of::<Box>(btn) {
                    let input = FormulaInput::new(Orientation::Horizontal, 4);
                    input.input.link_f(|input_content, added_char| {
                        println!("{input_content} {added_char}")
                    });
                    parent.append(&input.gtk());
                }
            });
            btn
        });
        Self {
            inputs: Vec::new(),
            gtk: gtk.clone(),
        }
    }
    pub fn contents(&self) -> Vec<String> {
        let mut vec = Vec::with_capacity(self.inputs.len());
        for input in self.inputs.iter() {
            vec.push(input.content());
        }
        vec
    }
    pub fn append(&mut self, input: &'a FormulaInput) {
        self.gtk.append(&input.gtk());
        self.inputs.push(input);
    }
    pub fn gtk(&self) -> Box {
        self.gtk.clone()
    }
}
