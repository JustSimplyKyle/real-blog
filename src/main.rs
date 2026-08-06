mod apply_traits;

use apply_traits::ApplyConditional;
use cosmic::iced::alignment::Vertical;
use cosmic::iced::{self, Alignment, Font, Length, Size, Task};
use cosmic::{Apply, Core, Element, app, executor, theme, widget};

const GITHUB: &str = "https://github.com/JustSimplyKyle";
const MAIMAI: &str = "https://www.tomomai.lol/profile/simplykyle/intl";
const TETRIO: &str = "https://ch.tetr.io/u/ultimatekyle";
const HUNINN: Font = Font::with_name("jf-openhuninn-2.1");
const HUNINN_ASSET: manganis::Asset = manganis::asset!("/assets/fonts/jf-openhuninn-2.1.ttf");

#[cfg(all(target_arch = "wasm32", feature = "hot-patch"))]
fn hot_patch_client() -> dioxus::prelude::Element {
    dioxus::dioxus_core::VNode::empty()
}

fn launch(font: Vec<u8>) -> cosmic::iced::Result {
    cosmic::iced::advanced::graphics::text::font_system()
        .write()
        .expect("lock font system")
        .load_font(font.into());

    app::run::<BlogApp>(
        app::Settings::default()
            .size(Size::new(1100.0, 850.0))
            .client_decorations(false),
        (),
    )
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init().expect("initialize browser logging");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        #[cfg(feature = "hot-patch")]
        {
            let document = web_sys::window()
                .expect("browser window")
                .document()
                .expect("browser document");
            let root = document
                .create_element("div")
                .expect("create hot-patch root");
            root.set_attribute("hidden", "")
                .expect("hide hot-patch root");
            document
                .body()
                .expect("document body")
                .append_child(&root)
                .expect("mount hot-patch root");

            dioxus::web::launch::launch_cfg(
                hot_patch_client,
                dioxus::web::Config::new().rootelement(root),
            );
        }

        wasm_bindgen_futures::spawn_local(async {
            let response = gloo_net::http::Request::get(&HUNINN_ASSET.to_string())
                .send()
                .await
                .expect("fetch application font");
            let font = response
                .binary()
                .await
                .expect("read application font response");

            launch(font).expect("launch application");
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    launch(std::fs::read(HUNINN_ASSET.resolve()).expect("read application font"))
        .expect("launch application");
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Page {
    #[default]
    Home,
    Blog,
    Connections,
}

#[derive(Clone, Debug)]
enum Message {
    Navigate(widget::segmented_button::Entity),
    Open(&'static str),
}

struct BlogApp {
    core: Core,
    page: Page,
    navigation: widget::segmented_button::SingleSelectModel,
    compact: bool,
}

impl cosmic::Application for BlogApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "dev.justsimplykyle.blog";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(mut core: Core, _flags: ()) -> (Self, Task<cosmic::Action<Message>>) {
        core.window.show_headerbar = false;
        core.window.content_container = false;
        let navigation = widget::segmented_button::Model::builder()
            .insert(|button| button.text("home").data(Page::Home).activate())
            .insert(|button| button.text("blog").data(Page::Blog))
            .insert(|button| button.text("connections").data(Page::Connections))
            .build();

        (
            Self {
                core,
                page: Page::Home,
                navigation,
                compact: false,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<cosmic::Action<Message>> {
        match message {
            Message::Navigate(id) => {
                if let Some(page) = self.navigation.data::<Page>(id).copied() {
                    self.navigation.activate(id);
                    self.page = page;
                }
            }
            Message::Open(url) => open_url(url),
        }

        Task::none()
    }

    fn on_window_resize(&mut self, _id: cosmic::iced::window::Id, width: f32, _height: f32) {
        self.compact = width < 720.0;
    }

    fn view(&self) -> Element<'_, Message> {
        let page = match self.page {
            Page::Home => self.home(),
            Page::Blog => self.blog(),
            Page::Connections => self.connections(),
        };

        widget::column![
            self.navigation(),
            page.apply(widget::scrollable).height(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .apply(widget::container)
        .class(theme::Container::Background)
        .into()
    }
}

impl BlogApp {
    fn navigation(&self) -> Element<'_, Message> {
        let spacing = theme::spacing();
        let nav = widget::segmented_control::horizontal(&self.navigation)
            .width(Length::Shrink)
            .style(theme::SegmentedButton::NavBar)
            .on_activate(Message::Navigate);

        widget::row![
            widget::text::heading("kyle.")
                .class(theme::Text::Accent)
                .width(Length::Fill),
            nav,
        ]
        .align_y(Alignment::Center)
        .padding([spacing.space_xs, spacing.space_s])
        .apply(widget::container)
        .class(theme::Container::Primary)
        .width(Length::Fill)
        .into()
    }

    fn home(&self) -> Element<'_, Message> {
        let spacing = theme::spacing();
        let section_title = |label| {
            widget::row![
                widget::text::title1(label).class(theme::Text::Accent),
                widget::divider::horizontal::light(),
            ]
            .align_y(Alignment::Center)
            .spacing(spacing.space_xs)
            .apply(Element::from)
        };
        let about =
            card(
                widget::column::with_children(vec![
                    widget::text::body("I like coding, yeah *flashy news* i read the code. I care about each little minute detail that makes an api good or bad to use.").into(),
                    widget::text::body("I daily-drive NixOS. The ability of reproducing an entire system gives me joy.").into(),
                    widget::text::body("I also had a ton of fun dealing with embedded works, espically tinkering with the type system to make certain hardware bugs impossible.").into(),
                    widget::text::body("Certainlly a bit too Rust-pilled, yk this website is built with libcosmic... somehow").into(),
                ])
                .spacing(spacing.space_s)
                .into(),
            );
        let games =
            card(
                widget::column::with_children(vec![
                    widget::text::body("I spend an unreasonable amount of time playing rhythm games. My main game is maimai & paradigm reboot, but I also play in FALSUS, Phigros, and Arcaea.").into(),
                    link_button("maimai profile ↗", MAIMAI),
                    widget::text::body("Away from rhythm games, I am also an avid factorio lover, Slay the Spire 2 enjoyer, or occasionally ~suffering~ in TETR.IO.").into(),
                    link_button("TETR.IO stats ↗", TETRIO),
                ])
                .spacing(spacing.space_xs)
                .into(),
            );

        widget::column![
            self.hero(),
            self.stats(),
            section_title("about me"),
            about,
            section_title("games i play"),
            games,
            section_title("things i love"),
            self.love_grid(),
            section_title("projects"),
            self.projects(),
        ]
        .spacing(spacing.space_s)
        .padding([
            spacing.space_l,
            spacing.space_m,
            spacing.space_xxl,
            spacing.space_m,
        ])
        .max_width(960)
        .apply(widget::container)
        .center_x(Length::Fill)
        .into()
    }

    fn hero(&self) -> Element<'_, Message> {
        let spacing = theme::spacing();
        let tags = widget::row![
            tag("Rust"),
            tag("NixOS"),
            tag("Embedded"),
            link_button("github.com/JustSimplyKyle", GITHUB),
        ]
        .spacing(spacing.space_xxs)
        .align_y(Vertical::Center)
        .wrap();

        widget::column![
            accent_caption("hiii I'm"),
            widget::text::title1("Kyle"),
            widget::text::body("a taiwanese open source developer and an rhythm game addict"),
            tags,
        ]
        .spacing(spacing.space_s)
        .padding(spacing.space_l)
        .apply(widget::container)
        .class(theme::Container::Primary)
        .width(Length::Fill)
        .into()
    }

    fn stats(&self) -> Element<'_, Message> {
        let spacing = theme::spacing();
        let stat = |label, value| {
            card(
                widget::column![
                    widget::text::caption(label),
                    widget::text::title4(value).class(theme::Text::Accent),
                ]
                .spacing(spacing.space_xxxs)
                .width(Length::Fill)
                .into(),
            )
        };
        let cards = vec![
            stat("Daily Driver", "NixOS"),
            stat("Language of Choice", "Rust"),
            stat("Projects", "GitHub →"),
        ];

        responsive_grid(cards, 3, self.compact)
    }

    fn love_grid(&self) -> Element<'_, Message> {
        let spacing = theme::spacing();
        let love = |name, description| {
            card(
                widget::column![
                    widget::text::title4(name).class(theme::Text::Accent),
                    widget::text::body(description),
                ]
                .spacing(spacing.space_xxs)
                .into(),
            )
        };
        let cards = vec![
            love(
                "Rust",
                "Make invalid states unrepresentable. If the types are right, it self documents and prevent logic bugs.",
            ),
            love(
                "Nix · NixOS",
                "Declarative AND reproducible. THE programmer operating system.",
            ),
            love(
                "Embedded Systems",
                "Microcontrollers, PCB layout, and async bare-metal. Still an absolute beginner tho.",
            ),
            love("Typst", "Modern typesetting at its finest."),
        ];

        responsive_grid(cards, 2, self.compact)
    }

    fn projects(&self) -> Element<'_, Message> {
        let spacing = theme::spacing();
        let project = |title, description, labels: &[&'static str], url| {
            let tags = labels
                .iter()
                .map(|&label| tag(label))
                .apply(widget::Row::from_iter)
                .spacing(spacing.space_xxxs)
                .align_y(Vertical::Center)
                .push(link_button("GitHub ↗", url));

            card(
                widget::column![
                    widget::text::title4(title).class(theme::Text::Accent),
                    widget::text::body(description),
                    tags.wrap(),
                ]
                .spacing(spacing.space_xs)
                .into(),
            )
        };
        let projects = vec![
            project(
                "RC Car PCB",
                "A custom ESP32 breakout board to replace a breadboard jumpers. Drives an A4988 stepper and an OLED status display.",
                &["hardware", "PCB", "ESP32"],
                "https://github.com/JustSimplyKyle/rc-car/tree/retest",
            ),
            project(
                "catbox-cli",
                "A proper command-line uploader for catbox.moe: concurrent uploads, live progress bars, and clean error handling through error_set.",
                &["async", "CLI", "Rust"],
                "https://github.com/JustSimplyKyle/catbox-cli",
            ),
            project(
                "gsat-parsing",
                "A Rust + OpenCV pipeline that turns Taiwan exam-score distribution images into structured data.",
                &["OpenCV", "Rust"],
                "https://github.com/justsimplykyle/gsat-parsing",
            ),
            project(
                "infi75 keyboard RE",
                "Captured USB packets, reverse-engineered a vendor lighting protocol, and rebuilt its music-reactive modes for Linux(cava-backed) in Rust.",
                &["reverse engineering", "USB", "Rust"],
                "https://github.com/JustSimplyKyle/infi75-custom",
            ),
            project(
                "rMPP manga reader",
                "A custom QML frontend injected into the reMarkable Paper Pro through an xovi hook.",
                &["QML", "hook injection", "Rust"],
                "https://github.com/JustSimplyKyle/rmpp-appload",
            ),
            project(
                "videosubextract",
                "A rewritten frontend of videosubfinder using libcosmic.",
                &["iced", "frontend", "Rust"],
                "https://github.com/JustSimplyKyle/videosubextract",
            ),
        ];

        responsive_grid(projects, 2, self.compact)
    }

    fn blog(&self) -> Element<'_, Message> {
        let spacing = theme::spacing();

        page_shell(
            page_hero("writing", "blog", "thoughts, projects, anything really"),
            card(
                widget::column![
                    widget::text::body("Nothing here yet."),
                    widget::text::heading(
                        "Posts are *probably* going to be written in Typst and rendered using typst.ts.",
                    )
                    .class(theme::Text::Accent),
                ]
                .spacing(spacing.space_xxs)
                .into(),
            ),
        )
    }

    fn connections(&self) -> Element<'_, Message> {
        let spacing = theme::spacing();

        page_shell(
            page_hero("connections", "connections", "// cool people i know"),
            card(
                widget::row![
                    widget::text::title2("淳")
                        .class(theme::Text::Accent)
                        .width(Length::Shrink),
                    widget::column![
                        widget::text::heading("淳"),
                        widget::text::body("群除我佬，我是肺霧").font(HUNINN),
                    ]
                    .width(Length::Fill),
                    link_button("visit ↗", "https://chuen666666.github.io/"),
                ]
                .align_y(Alignment::Center)
                .spacing(spacing.space_s)
                .into(),
            ),
        )
    }
}

fn accent_caption(
    label: impl Into<std::borrow::Cow<'static, str>> + 'static,
) -> Element<'static, Message> {
    widget::text::caption_heading(label)
        .class(theme::Text::Accent)
        .into()
}

fn tag(label: &'static str) -> Element<'static, Message> {
    let spacing = theme::spacing();

    widget::text::caption_heading(label)
        .class(theme::Text::Accent)
        .apply(widget::container)
        .padding([spacing.space_xxxs, spacing.space_xxs])
        .class(theme::Container::Card)
        .into()
}

fn link_button(label: &'static str, url: &'static str) -> Element<'static, Message> {
    widget::button::text(label)
        .class(theme::Button::Link)
        .on_press(Message::Open(url))
        .into()
}

fn card(content: Element<'static, Message>) -> Element<'static, Message> {
    let spacing = theme::spacing();

    content
        .apply(widget::container)
        .class(theme::Container::Primary)
        .padding(spacing.space_m)
        .width(Length::Fill)
        .into()
}

fn responsive_grid(
    cards: Vec<Element<'static, Message>>,
    column: usize,
    compact: bool,
) -> Element<'static, Message> {
    let spacing = theme::spacing();

    cards
        .into_iter()
        .apply(pairwise)
        .fold(iced::widget::Grid::new(), |grid, (lhs, rhs)| {
            grid.push(lhs).push_maybe(rhs)
        })
        .columns(column)
        .apply_if_some(compact.then_some(1), iced::widget::Grid::columns)
        .height(Length::Shrink)
        .spacing(spacing.space_xs)
        .into()
}

fn pairwise<T>(mut iter: impl Iterator<Item = T>) -> impl Iterator<Item = (T, Option<T>)> {
    std::iter::from_fn(move || Some((iter.next()?, iter.next())))
}

fn page_hero(
    eyebrow: &'static str,
    title: &'static str,
    subtitle: &'static str,
) -> Element<'static, Message> {
    let spacing = theme::spacing();

    widget::column![
        accent_caption(eyebrow),
        widget::text::title1(title),
        widget::text::body(subtitle),
    ]
    .spacing(spacing.space_s)
    .padding(spacing.space_l)
    .apply(widget::container)
    .class(theme::Container::Primary)
    .width(Length::Fill)
    .into()
}

fn page_shell(
    hero: Element<'static, Message>,
    content: Element<'static, Message>,
) -> Element<'static, Message> {
    let spacing = theme::spacing();

    widget::column![hero, content]
        .spacing(spacing.space_s)
        .padding([
            spacing.space_l,
            spacing.space_m,
            spacing.space_xxl,
            spacing.space_m,
        ])
        .max_width(960)
        .apply(widget::container)
        .center_x(Length::Fill)
        .into()
}

#[cfg(target_arch = "wasm32")]
fn open_url(url: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.open_with_url_and_target(url, "_blank");
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn open_url(_url: &str) {}
