use std::{collections::HashMap, rc::Rc};

pub trait NoteManagement {
    // get all notes
    fn get_notes(&self) -> HashMap<(usize, usize), String>;
    // get note for the current chapter
    fn get_current_note(&self) -> Option<String>;
    // add/edit a note for the current chapter and page
    fn edit_note(&mut self, note: String); 
    // delete a note for the current chapter and page
    fn delete_note(&mut self);
    // delete all notes
    fn delete_all_notes(&mut self);
}