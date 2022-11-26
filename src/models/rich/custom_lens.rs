use druid::Lens;

use crate::{traits::reader::BookReading, utils::rich_text_fn::rebuild_rendered_text};

use super::{rich_text::RichText};

pub struct SelectedPageLens;

impl<B: BookReading> Lens<B, RichText> for SelectedPageLens {
    fn with<V, F: FnOnce(&RichText) -> V>(&self, data: &B, f: F) -> V {
        f(&rebuild_rendered_text(data.get_page_of_chapter().as_str()))
    }

    fn with_mut<V, F: FnOnce(&mut RichText) -> V>(&self, data: &mut B, f: F) -> V {
        f(&mut rebuild_rendered_text(data.get_page_of_chapter().as_str()))
    }
}

pub struct DualPage0Lens;
pub struct DualPage1Lens;


impl<B: BookReading> Lens<B, RichText> for DualPage0Lens {
    fn with<V, F: FnOnce(&RichText) -> V>(&self, data: &B, f: F) -> V {
        f(&rebuild_rendered_text(data.get_dual_pages().0.as_str()))
    }

    fn with_mut<V, F: FnOnce(&mut RichText) -> V>(&self, data: &mut B, f: F) -> V {
        f(&mut rebuild_rendered_text(data.get_dual_pages().0.as_str()))
    }
}

impl<B: BookReading> Lens<B, RichText> for DualPage1Lens {
    fn with<V, F: FnOnce(&RichText) -> V>(&self, data: &B, f: F) -> V {
        f(&rebuild_rendered_text(data.get_dual_pages().1.as_str()))
    }

    fn with_mut<V, F: FnOnce(&mut RichText) -> V>(&self, data: &mut B, f: F) -> V {
        f(&mut rebuild_rendered_text(data.get_dual_pages().1.as_str()))
    }
}