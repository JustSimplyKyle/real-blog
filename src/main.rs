use dioxus::prelude::*;

static GLOBAL_CSS: Asset = asset!("/assets/global.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: GLOBAL_CSS }
        Router::<Route> {}
    }
}

// ─────────────────────────────────────────────────────────────
//  Routing
// ─────────────────────────────────────────────────────────────

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[route("/blog")]
    Blog {},
    #[route("/friends")] // <-- Add this
    Friends {},
}

// ─────────────────────────────────────────────────────────────
//  NavBar layout — wraps every page
// ─────────────────────────────────────────────────────────────

#[component]
fn NavBar() -> Element {
    #[css_module("/assets/layout.css")]
    struct Styles;

    let route: Route = use_route();
    let is_home = matches!(route, Route::Home {});
    let is_blog = matches!(route, Route::Blog {});
    let is_friends = matches!(route, Route::Friends {});

    let home_class = if is_home {
        format!("{} {}", Styles::nav_link, Styles::nav_link_active)
    } else {
        Styles::nav_link.to_string()
    };
    let blog_class = if is_blog {
        format!("{} {}", Styles::nav_link, Styles::nav_link_active)
    } else {
        Styles::nav_link.to_string()
    };

    let friends_class = if is_friends {
        format!("{} {}", Styles::nav_link, Styles::nav_link_active)
    } else {
        Styles::nav_link.to_string()
    };

    rsx! {
        nav { class: Styles::navbar,
            // logo
            Link { to: Route::Home {}, class: Styles::nav_logo.to_string(),
                "kyle" span { "." }
            }
            // links
            div {
                class: Styles::nav_links,
                Link { to: Route::Home {}, class: "{home_class}", "home" }
                Link { to: Route::Blog {}, class: "{blog_class}", "blog" }
                Link { to: Route::Friends {}, class: "{friends_class}", "connections" }
            }
        }
        div {
            Outlet::<Route> {}
        }
    }
}

#[component]
fn Friends() -> Element {
    #[css_module("/assets/layout.css")]
    struct Styles;

    rsx! {
        div { class: Styles::page_container_padded,
            div { class: Styles::blog_hero,
                div { class: Styles::blog_hero_inner,
                    div { class: Styles::eyebrow, "connections" }
                    h1 { class: Styles::blog_title, "connecitons" }
                    div { class: Styles::blog_subtitle, "// cool people i know" }
                }
            }

            // Reusing your project grid for a nice 2-column layout!
            div { class: Styles::projects_grid,
                FriendCard {
                    name: "淳",
                    url: "https://chuen666666.github.io/",
                    desc: "群除我佬，我是肺霧",
                    avatar: "https://chuen666666.github.io/img/avatar.jpg"
                }
                // Just add more FriendCard blocks here when you get more links!
            }
        }
    }
}

#[component]
fn FriendCard(
    name: &'static str,
    url: &'static str,
    desc: &'static str,
    avatar: &'static str,
) -> Element {
    #[css_module("/assets/components.css")]
    struct Styles;

    rsx! {
        a {
            href: "{url}",
            target: "_blank",
            // Combine the base "card" and our new "friend" modifier using Dioxus styles mapping
            class: "{Styles::card} {Styles::friend}",
            style: "--card-accent: var(--accent2); --card-border: var(--accent2-mid);",

            // Avatar
            img {
                src: "{avatar}",
                alt: "{name} avatar",
                class: Styles::avatar,
            }

            // Text Content
            div {
                class: Styles::info,
                h3 { class: Styles::card_title, "{name}" }
                div { class: Styles::card_body, "{desc}" }
            }
        }
    }
}
// ─────────────────────────────────────────────────────────────
//  Atomic Components
// ─────────────────────────────────────────────────────────────

#[component]
fn TagCard(text: &'static str, color: Option<&'static str>, link: Option<&'static str>) -> Element {
    #[css_module("/assets/components.css")]
    struct Styles;

    let (text_c, border_c, bg_c) = match color {
        Some("accent2") => (
            "var(--accent2)",
            "var(--accent2-mid)",
            "var(--accent2-light)",
        ),
        Some("accent3") => (
            "var(--accent3)",
            "var(--accent3-mid)",
            "var(--accent3-light)",
        ),
        Some("gray") => (
            "#888880",
            "rgba(136,136,128,0.30)",
            "rgba(136,136,128,0.10)",
        ),
        _ => ("var(--accent)", "var(--accent-mid)", "var(--accent-light)"),
    };
    let style_str = format!("--tag-text:{text_c};--tag-border:{border_c};--tag-bg:{bg_c};");

    if let Some(url) = link {
        rsx! { a { href: "{url}", target: "_blank", class: Styles::tag, style: "{style_str}", "{text}" } }
    } else {
        rsx! { span { class: Styles::tag, style: "{style_str}", "{text}" } }
    }
}

#[component]
fn Badge(label: &'static str, value: &'static str, color: Option<&'static str>) -> Element {
    #[css_module("/assets/components.css")]
    struct Styles;

    let accent = match color {
        Some("accent2") => "var(--accent2)",
        _ => "var(--accent)",
    };
    rsx! {
        div { class: Styles::badge, style: "--badge-accent:{accent};",
            div { class: Styles::badge_label, "{label}" }
            div { class: Styles::badge_value, "{value}" }
        }
    }
}

#[component]
fn ProjectCard(
    title: &'static str,
    tags: Element,
    color: Option<&'static str>,
    children: Element,
) -> Element {
    #[css_module("/assets/components.css")]
    struct Styles;

    let (accent, border) = match color {
        Some("accent2") => ("var(--accent2)", "var(--accent2-mid)"),
        _ => ("var(--accent)", "var(--accent-mid)"),
    };
    rsx! {
        div { class: Styles::card, style: "--card-accent:{accent};--card-border:{border};",
            h3 { class: Styles::card_title, "{title}" }
            div { class: Styles::card_body, {children} }
            div { class: Styles::card_tags, {tags} }
        }
    }
}

// ─────────────────────────────────────────────────────────────
//  Home page
// ─────────────────────────────────────────────────────────────

#[component]
fn Home() -> Element {
    #[css_module("/assets/layout.css")]
    struct Styles;

    rsx! {
        div { class: Styles::page_container_padded,
            Hero {}
            StatStrip {}

            div { class: Styles::section_title, "about me" }
            About {}

            div { class: Styles::section_title, "games i play" }
            Games {}

            div { class: Styles::section_title, "things i love" }
            LoveGrid {}

            div { class: Styles::section_title, "projects" }
            Projects {}
        }
    }
}

#[component]
fn Hero() -> Element {
    #[css_module("/assets/layout.css")]
    struct Styles;

    rsx! {
        div { class: Styles::hero,
            div { class: Styles::hero_inner,
                div { class: Styles::eyebrow, "hi, i'm" }
                h1 { class: Styles::hero_name, "Kyle" }
                p { class: Styles::hero_bio,
                    "i write " strong { "Rust" } ". i run " strong { "NixOS" }
                    ". i touch " strong { "hardware" } " sometimes. "
                }
                div { class: Styles::tags_row,
                    TagCard { text: "Rust",     color: None,            link: None }
                    TagCard { text: "NixOS",    color: Some("accent2"), link: None }
                    TagCard { text: "Embedded", color: Some("accent3"), link: None }
                    TagCard {
                        text: "github.com/JustSimplyKyle",
                        color: Some("gray"),
                        link: Some("https://github.com/JustSimplyKyle")
                    }
                }
            }
            // div { class: Styles::portrait,
            //     img { src: asset!("/assets/IMG_1498.jpg"), alt: "Kyle's portrait" }
            // }
        }
    }
}

#[component]
fn StatStrip() -> Element {
    #[css_module("/assets/layout.css")]
    struct Styles;

    rsx! {
        div { class: Styles::stat_strip,
            Badge { label: "daily driver",  value: "NixOS",    color: Some("accent2") }
            Badge { label: "main language", value: "Rust",     color: None }
            Badge { label: "projects",      value: "GitHub →", color: Some("accent2") }
        }
    }
}

#[component]
fn About() -> Element {
    #[css_module("/assets/layout.css")]
    struct Styles;

    rsx! {
        div { class: Styles::about_block,
            p { class: Styles::about_para,
                "I like " strong { "Rust" } " a lot. What really gets me is the type system — \
                 the idea that you can make invalid states "
                em { "literally unrepresentable" }
                " in your program. \
                 If the types line up, a whole class of bugs just can't exist. \
                 That's a really powerful thing."
            }
            p { class: Styles::about_para,
                "I daily-drive " strong { "NixOS" }
                ". if i ever nuke my drive or get a new machine, \
                 i can rebuild my entire system exactly as it was in minutes. \
                 that kind of reliability is hard to give up once you have it."
            }
            p { class: Styles::about_para,
                "I also enjoy " strong { "embedded" }
                " stuff — PCBs, microcontrollers, motors. \
                 writing code that directly moves physical things scratches a completely \
                 different itch than anything higher up the stack."
            }
            p { class: Styles::about_para,
                "most of my projects start because a tool I needed "
                em { "didn't support Linux" }
                " or a format I wanted "
                em { "didn't have a parser" }
                ". so I write one."
            }
        }
    }
}

#[component]
fn LoveGrid() -> Element {
    #[css_module("/assets/layout.css")]
    struct Styles;

    rsx! {
        div { class: Styles::love_grid,
            div {
                class: Styles::love_card,
                style: "--love-accent: var(--accent); --love-glow: var(--accent-light);",
                div { class: Styles::love_glyph, "🦀" }
                div { class: Styles::love_name, "Rust" }
                p { class: Styles::love_desc,
                    "the type system lets you make invalid states unrepresentable. \
                     if the types are right, whole categories of bugs just can't happen. \
                     zero-cost abstractions are a nice bonus."
                }
            }
            div {
                class: Styles::love_card,
                style: "--love-accent: var(--accent2); --love-glow: var(--accent2-light);",
                div { class: Styles::love_glyph, "❄️" }
                div { class: Styles::love_name, "Nix · NixOS" }
                p { class: Styles::love_desc,
                    "declarative, reproducible, rollback-able. \
                     nuke the drive, run one command, \
                     get your entire system back exactly as it was."
                }
            }
            div {
                class: Styles::love_card,
                style: "--love-accent: var(--accent3); --love-glow: var(--accent3-light);",
                div { class: Styles::love_glyph, "🔧" }
                div { class: Styles::love_name, "Embedded Systems" }
                p { class: Styles::love_desc,
                    "microcontrollers, PCB layout, async bare-metal. \
                     still learning, but very fun."
                }
            }
            div {
                class: Styles::love_card,
                style: "--love-accent: var(--accent2); --love-glow: var(--accent2-light);",
                div { class: Styles::love_glyph, "🐧" }
                div { class: Styles::love_name, "Linux" }
                p { class: Styles::love_desc,
                    "the freedom to customize linux is just god i just love it"
                }
            }
            div {
                class: Styles::love_card,
                style: "--love-accent: var(--accent2); --love-glow: var(--accent2-light);",
                div { class: Styles::love_glyph, "\u{270D}" }
                div { class: Styles::love_name, "Typst" }
                p { class: Styles::love_desc,
                    "modern typesetting at its finest."
                }
            }
        }
    }
}

#[component]
fn Projects() -> Element {
    #[css_module("/assets/layout.css")]
    struct Styles;

    rsx! {
        div { class: Styles::projects_grid,
            div { class: Styles::projects_col,
                ProjectCard {
                    title: "RC Car PCB",
                    color: None,
                    tags: rsx! {
                        TagCard { text: "hardware", color: Some("accent2"), link: None }
                        TagCard { text: "PCB",      color: Some("accent2"), link: None }
                        TagCard { text: "ESP32",    color: Some("accent3"), link: None }
                        TagCard {
                            text: "GitHub",
                            color: Some("gray"),
                            link: Some("https://github.com/JustSimplyKyle/rc-car/tree/retest")
                        }
                    },
                    "Designed a custom ESP32 breakout board to replace a rats-nest of breadboard \
                     jumpers. Drives an A4988 stepper and an OLED status display."
                    br {}
                    img {
                        src: asset!("/assets/pcb.png"),
                        alt: "PCB photo",
                        style: "width:100%;margin-top:0.75rem;border-radius:6px;opacity:0.9;"
                    }
                }
                ProjectCard {
                    title: "catbox-cli",
                    color: Some("accent2"),
                    tags: rsx! {
                        TagCard { text: "async", color: Some("accent2"), link: None }
                        TagCard { text: "CLI",   color: Some("accent2"), link: None }
                        TagCard { text: "Rust",  color: None,            link: None }
                        TagCard {
                            text: "GitHub",
                            color: Some("gray"),
                            link: Some("https://github.com/JustSimplyKyle/catbox-cli")
                        }
                    },
                    "A proper command-line uploader for catbox.moe. \
                     Concurrent uploads, live progress bars, clean error handling."
                }
            }
            div { class: Styles::projects_col,
                ProjectCard {
                    title: "gsat-parsing",
                    color: None,
                    tags: rsx! {
                        TagCard { text: "OpenCV", color: Some("accent2"), link: None }
                        TagCard { text: "Rust",   color: None,            link: None }
                        TagCard {
                            text: "GitHub",
                            color: Some("gray"),
                            link: Some("https://github.com/justsimplykyle/gsat-parsing")
                        }
                    },
                    "Taiwan's ministry of education publishes exam score distributions \
                     as images only. So I wrote a Rust + OpenCV pipeline to \
                     binarize, denoise, detect intersections, and spit out structured data."
                }
                ProjectCard {
                    title: "infi75 keyboard RE",
                    color: None,
                    tags: rsx! {
                        TagCard { text: "reverse engineering", color: Some("accent2"), link: None }
                        TagCard { text: "USB",                 color: Some("accent2"), link: None }
                        TagCard { text: "Rust",                color: None,            link: None }
                        TagCard {
                            text: "GitHub",
                            color: Some("gray"),
                            link: Some("https://github.com/JustSimplyKyle/infi75-custom")
                        }

                    },
                    "The vendor had no Linux driver. So I captured USB packets in \
                     Wireshark, reverse-engineered the lighting protocol, \
                     and reimplemented the music-reactive modes using cava in Rust."
                }
                ProjectCard {
                    title: "rMPP manga reader",
                    color: Some("accent2"),
                    tags: rsx! {
                        TagCard { text: "QML",            color: Some("accent2"), link: None }
                        TagCard { text: "hook injection", color: Some("accent2"), link: None }
                        TagCard { text: "Rust",           color: None,            link: None }
                        TagCard {
                            text: "GitHub",
                            color: Some("gray"),
                            link: Some("https://github.com/JustSimplyKyle/rmpp-appload")
                        }
                    },
                    "Custom frontend injected into the reMarkable Paper Pro via \
                     an xovi hook."
                }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────
//  Blog page
// ─────────────────────────────────────────────────────────────

#[component]
fn Blog() -> Element {
    #[css_module("/assets/layout.css")]
    struct Styles;

    rsx! {
        div { class: Styles::page_container_padded,
            div { class: Styles::blog_hero,
                div { class: Styles::blog_hero_inner,
                    div { class: Styles::eyebrow, "writing" }
                    h1 { class: Styles::blog_title, "blog" }
                    div { class: Styles::blog_subtitle, "// thoughts, projects, things i figured out" }
                }
            }
            div { class: Styles::blog_note,
                "nothing here yet. "
                strong { "posts are going to be written in Typst" }
                " and be rendered using typst.ts"
            }
        }
    }
}

#[component]
fn Games() -> Element {
    #[css_module("/assets/layout.css")]
    struct Styles;

    rsx! {
        div { class: Styles::about_block,
            p { class: Styles::about_para,
                "i spend an unreasonable amount of time playing rhythm games. my main game is "
                strong { "maimai" } " ("
                a { href: "https://www.tomomai.lol/profile/simplykyle/intl", target: "_blank", "see my profile" }
                "), but i also play "
                em { "in FALSUS" } ", " em { "Phigros" } ", and " em { "Arcaea" }
                " (though i'm definitely still a noob at that last one)."
            }
            p { class: Styles::about_para,
                "when i'm not hitting buttons to a beat, i'm usually grinding "
                strong { "Celeste" }
                " (currently suffering through the Strawberry Jam mod on Expert difficulty), loving that rogue like of "
                strong { "Slay the Spire 2" }
                " (got to ascension 10 barring regent), or occasionally tetrising in "
                strong { "TETR.IO" } " ("
                a { href: "https://ch.tetr.io/u/ultimatekyle", target: "_blank", "stats" }
                " — though i'm a bit less active there these days)."
            }
            p { class: Styles::about_para,
                "i generally don't play big AAA titles at all, but "
                strong { "NieR:Automata" }
                " is the one major exception i've made. the soundtrack and story just hit completely different."
            }
        }
    }
}
