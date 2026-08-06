use cosmic::iced::{Alignment, Font, Length, Size, Task};
use cosmic::{Apply, Core, Element, app, executor, theme, widget};

const GITHUB: &str = "https://github.com/JustSimplyKyle";
const MAIMAI: &str = "https://www.tomomai.lol/profile/simplykyle/intl";
const TETRIO: &str = "https://ch.tetr.io/u/ultimatekyle";
const HUNINN: Font = Font::with_name("jf-openhuninn-2.1");
const HUNINN_BYTES: &[u8] = include_bytes!("../assets/fonts/jf-openhuninn-2.1.ttf");

fn main() -> cosmic::iced::Result {
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init().expect("initialize browser logging");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    app::run::<BlogApp>(
        app::Settings::default()
            .size(Size::new(1100.0, 850.0))
            .client_decorations(false),
        (),
    )
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
    Navigate(Page),
    Open(&'static str),
}

struct BlogApp {
    core: Core,
    page: Page,
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

        (
            Self {
                core,
                page: Page::Home,
                compact: false,
            },
            cosmic::iced::font::load(HUNINN_BYTES).discard(),
        )
    }

    fn update(&mut self, message: Message) -> Task<cosmic::Action<Message>> {
        match message {
            Message::Navigate(page) => self.page = page,
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
        let nav = [
            (Page::Home, "home"),
            (Page::Blog, "blog"),
            (Page::Connections, "connections"),
        ]
        .into_iter()
        .fold(
            widget::row![].spacing(spacing.space_xxxs),
            |row, (page, label)| {
                let button = if self.page == page {
                    widget::button::suggested(label)
                } else {
                    widget::button::text(label)
                };
                row.push(button.on_press(Message::Navigate(page)))
            },
        );

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

        widget::column![
            self.hero(),
            self.stats(),
            section_title("about me"),
            about(),
            section_title("games i play"),
            games(),
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
        .wrap();

        widget::column![
            accent_caption("hiii I'm"),
            widget::text::title1("Kyle"),
            widget::text::body("a taiwanese open source developer and an rhythm game addict"),
            tags,
        ]
        .spacing(spacing.space_xs)
        .padding(spacing.space_l)
        .apply(widget::container)
        .class(theme::Container::Primary)
        .width(Length::Fill)
        .into()
    }

    fn stats(&self) -> Element<'_, Message> {
        let spacing = theme::spacing();
        let cards = vec![
            stat("Daily Driver", "NixOS"),
            stat("Language of Choice", "Rust"),
            stat("Projects", "GitHub →"),
        ];

        if self.compact {
            widget::column::with_children(cards)
                .spacing(spacing.space_xs)
                .into()
        } else {
            widget::row::with_children(cards)
                .spacing(spacing.space_xs)
                .width(Length::Fill)
                .into()
        }
    }

    fn love_grid(&self) -> Element<'_, Message> {
        let cards = vec![
            love(
                "Rust",
                "Make invalid states unrepresentable. If the types are right, whole categories of bugs cannot happen.",
            ),
            love(
                "Nix · NixOS",
                "Declarative AND reproducible. THE programmer operating system",
            ),
            love(
                "Embedded Systems",
                "Microcontrollers, PCB layout, and async bare-metal. Still learning, but very fun.",
            ),
            love("Typst", "Modern typesetting at its finest."),
        ];

        responsive_grid(cards, self.compact)
    }

    fn projects(&self) -> Element<'_, Message> {
        let projects = vec![
            project(
                "RC Car PCB",
                "Designed a custom ESP32 breakout board to replace a rats-nest of breadboard jumpers. Drives an A4988 stepper and an OLED status display.",
                &["hardware", "PCB", "ESP32"],
                "https://github.com/JustSimplyKyle/rc-car/tree/retest",
            ),
            project(
                "catbox-cli",
                "A proper command-line uploader for catbox.moe: concurrent uploads, live progress bars, and clean error handling.",
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
                "Captured USB packets, reverse-engineered a vendor lighting protocol, and rebuilt its music-reactive modes for Linux in Rust.",
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

        responsive_grid(projects, self.compact)
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

fn section_title(label: &'static str) -> Element<'static, Message> {
    let spacing = theme::spacing();

    widget::row![
        widget::text::title1(label).class(theme::Text::Accent),
        widget::divider::horizontal::light(),
    ]
    .align_y(Alignment::Center)
    .spacing(spacing.space_xs)
    .into()
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

fn stat(label: &'static str, value: &'static str) -> Element<'static, Message> {
    let spacing = theme::spacing();

    card(
        widget::column![
            widget::text::caption(label),
            widget::text::title4(value).class(theme::Text::Accent),
        ]
        .spacing(spacing.space_xxxs)
        .width(Length::Fill)
        .into(),
    )
}

fn about() -> Element<'static, Message> {
    let spacing = theme::spacing();

    card(
        widget::column::with_children(vec![
            widget::text::body("I like coding, yeah *flashy news* i read the code. I care about each little minute detail that makes an api good or bad to use.").into(),
            widget::text::body("I daily-drive NixOS. The ability of reproducing an entire system gives me joy.").into(),
            widget::text::body("I also had a ton of fun dealing with embedded works, espically tinkering with the type system to make certain hardware bugs impossible.").into(),
            widget::text::body("Certainlly a bit too Rust-pilled, yk this website is built with libcosmic... somehow").into(),
        ])
        .spacing(spacing.space_xs)
        .into(),
    )
}

fn games() -> Element<'static, Message> {
    let spacing = theme::spacing();

    card(
        widget::column::with_children(vec![
            widget::text::body("I spend an unreasonable amount of time playing rhythm games. My main game is maimai & paradigm reboot, but I also play in FALSUS, Phigros, and Arcaea.").into(),
            link_button("maimai profile ↗", MAIMAI),
            widget::text::body("Away from rhythm games, I am also an avid factorio lover, Slay the Spire 2 enjoyer, or occasionally ~suffering~ in TETR.IO.").into(),
            link_button("TETR.IO stats ↗", TETRIO),
        ])
        .spacing(spacing.space_xs)
        .into(),
    )
}

fn love(name: &'static str, description: &'static str) -> Element<'static, Message> {
    let spacing = theme::spacing();

    card(
        widget::column![
            widget::text::title4(name).class(theme::Text::Accent),
            widget::text::body(description),
        ]
        .spacing(spacing.space_xxs)
        .into(),
    )
}

fn project(
    title: &'static str,
    description: &'static str,
    tags: &'static [&'static str],
    url: &'static str,
) -> Element<'static, Message> {
    let spacing = theme::spacing();
    let tags = tags
        .iter()
        .fold(widget::row![].spacing(spacing.space_xxxs), |row, label| {
            row.push(tag(label))
        });

    card(
        widget::column![
            widget::text::title4(title).class(theme::Text::Accent),
            widget::text::body(description),
            tags.wrap(),
            link_button("GitHub ↗", url),
        ]
        .spacing(spacing.space_xs)
        .into(),
    )
}

fn responsive_grid(
    cards: Vec<Element<'static, Message>>,
    compact: bool,
) -> Element<'static, Message> {
    let spacing = theme::spacing();

    if compact {
        widget::column::with_children(cards)
            .spacing(spacing.space_xs)
            .into()
    } else {
        let mut rows = widget::column![].spacing(spacing.space_xs);
        let mut cards = cards.into_iter();

        while let Some(left) = cards.next() {
            let mut row = widget::row![left]
                .spacing(spacing.space_xs)
                .width(Length::Fill);

            if let Some(right) = cards.next() {
                row = row.push(right);
            } else {
                row = row.push(widget::space::horizontal().width(Length::Fill));
            }

            rows = rows.push(row);
        }

        rows.width(Length::Fill).into()
    }
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
    .spacing(spacing.space_xxxs)
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
