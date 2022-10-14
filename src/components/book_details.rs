use druid::Widget;

use super::library::Library;

pub struct BookDetails;

impl Widget<Library> for BookDetails {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Library,
        env: &druid::Env,
    ) {
        ctx.request_paint();
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Library,
        env: &druid::Env,
    ) {
        ctx.request_layout();
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &Library,
        data: &Library,
        env: &druid::Env,
    ) {
        ctx.request_layout();
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &Library,
        env: &druid::Env,
    ) -> druid::Size {
        if data.get_selected_book().is_some() {
            bc.max()
        } else {
            bc.min()
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Library, env: &druid::Env) {
        ()
    }
}
