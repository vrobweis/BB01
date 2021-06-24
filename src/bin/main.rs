use druid::piet::InterpolationMode;
use druid::text::format::ParseFormatter;
use druid::widget::{prelude::*, FillStrat, Image};
use druid::widget::{
    Checkbox, CrossAxisAlignment, Flex, Label, RadioGroup, SizedBox, TextBox, WidgetExt,
};
use druid::{AppLauncher, Color, Data, ImageBuf, Lens, LensExt, WindowDesc};


#[derive(Clone, Data, Lens)]
struct AppData {
    state: u32 //replace with state enum
}



/// builds a child Flex widget from some paramaters.
struct Rebuilder {
    inner: Box<dyn Widget<AppData>>,
}

impl Rebuilder {
    fn new() -> Rebuilder {
        Rebuilder {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &AppData) {
        self.inner = build_widget(&data);
    }
}

impl Widget<AppData> for Rebuilder {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppData, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppData, data: &AppData, _env: &Env) {
        if !old_data.same(&data) {
            self.rebuild_inner(data);
            ctx.children_changed();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppData,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, env: &Env) {
        self.inner.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
}



fn build_widget(state: &AppData) -> Box<dyn Widget<AppData>> {
    let png_data = //ImageBuf::from_raw(include_bytes!("./assets/PicWithAlpha.png"), ImageFormat::Rgb).unwrap();
        ImageBuf::empty(); // TODO: Use real buffer for sample work
    let mut img = Image::new(png_data).fill_mode(FillStrat::Fill);
    let mut sized = SizedBox::new(img);
    sized.border(Color::grey(0.6), 2.0).center().boxed()
}

fn make_ui() -> impl Widget<AppData> {
    Flex::column()
        .must_fill_main_axis(true)
        .with_default_spacer()
        .with_flex_child(Rebuilder::new().center(), 1.0)
        .padding(10.0)
}

pub fn main() {
    let main_window = WindowDesc::new(make_ui)
        .window_size((650., 450.))
        .title("Flex Container Options");

    let state = AppData {
        state: 3
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(state)
        .expect("Failed to launch application");
}
