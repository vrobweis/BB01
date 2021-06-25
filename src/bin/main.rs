use druid::{
    *,
    piet::InterpolationMode,
    text::format::ParseFormatter,
    widget::{
        *,
    },
};


const APP_NAME: &str = "BB01";


#[derive(Clone, Data, Lens)]
struct AppData {
    state: u32, //replace with state enum
}




const DARK_GREY: Color = Color::grey8(0x3a);
const DARKER_GREY: Color = Color::grey8(0x11);
const LIGHTER_GREY: Color = Color::grey8(0xbb);

fn build_app() -> impl Widget<AppData> {
    let gradient = LinearGradient::new(
        UnitPoint::TOP_LEFT,
        UnitPoint::BOTTOM_RIGHT,
        (DARKER_GREY, LIGHTER_GREY),
    );

    // a custom background
    let polka_dots = Painter::new(|ctx, _, _| {
        let bounds = ctx.size().to_rect();
        let dot_diam = bounds.width().max(bounds.height()) / 20.;
        let dot_spacing = dot_diam * 1.8;
        for y in 0..((bounds.height() / dot_diam).ceil() as usize) {
            for x in 0..((bounds.width() / dot_diam).ceil() as usize) {
                let x_offset = (y % 2) as f64 * (dot_spacing / 2.0);
                let x = x as f64 * dot_spacing + x_offset;
                let y = y as f64 * dot_spacing;
                let circ = kurbo::Circle::new((x, y), dot_diam / 2.0);
                let purp = Color::rgb(1.0, 0.22, 0.76);
                ctx.fill(circ, &purp);
            }
        }
    });

    Flex::column()
        .with_flex_child(
            Flex::row()
                .with_flex_child(
                    Label::new("top left")
                        .center()
                        .border(DARK_GREY, 4.0)
                        .padding(10.0),
                    1.0,
                )
                .with_flex_child(
                    Label::new("top right")
                        .center()
                        .background(DARK_GREY)
                        .padding(10.0),
                    1.0,
                ),
            1.0,
        )
        .with_flex_child(
            Flex::row()
                .with_flex_child(
                    Label::new("bottom left")
                        .center()
                        .background(gradient)
                        .rounded(10.0)
                        .padding(10.0),
                    1.0,
                )
                .with_flex_child(
                    Label::new("bottom right")
                        .center()
                        .border(LIGHTER_GREY, 4.0)
                        .background(polka_dots)
                        .rounded(10.0)
                        .padding(10.0),
                    1.0,
                ),
            1.0,
        )
}

fn build_image_widget(state: &AppData) -> Box<dyn Widget<AppData>> {
    let png_data = //ImageBuf::from_raw(include_bytes!("./assets/PicWithAlpha.png"), ImageFormat::Rgb).unwrap();
        ImageBuf::empty(); // TODO: Use real buffer for sample work
    let img = Image::new(png_data).fill_mode(FillStrat::Fill);
    let sized = SizedBox::new(img);
    sized.border(Color::grey(0.6), 2.0).center().boxed()
}

fn macos_application_menu<T: Data>() -> MenuDesc<T> {
    MenuDesc::new(LocalizedString::new("macos-menu-application-menu"))
        .append(MenuItem::new(
            LocalizedString::new("macos-menu-about-app"),
            commands::SHOW_ABOUT,
        ))
        .append_separator()
        .append(
            MenuItem::new(
                LocalizedString::new("macos-menu-preferences"),
                commands::SHOW_PREFERENCES,
            )
            .hotkey(RawMods::Meta, ",")
            .disabled(),
        )
        .append_separator()
        .append(MenuDesc::new(LocalizedString::new("macos-menu-services")))
        .append(
            MenuItem::new(
                LocalizedString::new("macos-menu-hide-app"),
                commands::HIDE_APPLICATION,
            )
            .hotkey(RawMods::Meta, "h"),
        )
        .append(
            MenuItem::new(
                LocalizedString::new("macos-menu-hide-others"),
                commands::HIDE_OTHERS,
            )
            .hotkey(RawMods::AltMeta, "h"),
        )
        .append(
            MenuItem::new(
                LocalizedString::new("macos-menu-show-all"),
                commands::SHOW_ALL,
            )
            .disabled(),
        )
        .append_separator()
        .append(
            MenuItem::new(
                LocalizedString::new("macos-menu-quit-app"),
                commands::QUIT_APP,
            )
            .hotkey(RawMods::Meta, "q"),
        )
}

pub fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_app)
        .window_size((650., 450.))
        .title(APP_NAME)
        .resizable(true)
        .menu(macos_application_menu());

    let state = AppData {
        state: 1337
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(state)?;

    Ok(())
}
