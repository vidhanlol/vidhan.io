use axum::{extract::Path, Router};
use hypertext::{html_elements, maud, maud_move, GlobalAttributes, Renderable};
use tracing::instrument;

use crate::{
    cached,
    document::{Document, DocumentParts},
    post::Post,
    SiteError, SiteResult,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(home))
        .route("/post/:slug", axum::routing::get(post))
}

#[derive(Debug, Clone, Copy)]
struct Link {
    pub name: &'static str,
    pub description: &'static str,
    pub href: &'static str,
}

impl Renderable for Link {
    fn render_to(self, output: &mut String) {
        maud! {
            a href=(self.href) {
                b { (self.name) }
                " - "
                (self.description)
            }
        }
        .render_to(output);
    }
}

const PROJECTS: &[Link] = &[
    Link {
        name: "site",
        description: "this website!",
        href: "https://github.com/vidhanio/site",
    },
    Link {
        name: "hypertext",
        description: "a blazing fast type-checked html macro.",
        href: "https://github.com/vidhanio/hypertext",
    },
    Link {
        name: "html-node",
        description: "an html macro for rust.",
        href: "https://github.com/vidhanio/html-node",
    },
    Link {
        name: "fncli",
        description: "an attribute macro to simplify writing simple clis in rust.",
        href: "https://github.com/vidhanio/fncli",
    },
    Link {
        name: "diswordle",
        description: "a discord bot to play wordle right in your discord server.",
        href: "https://github.com/vidhanio/diswordle",
    },
    Link {
        name: "checkpoint",
        description:
            "a discord bot to provide easy verification for discord servers in my school board.",
        href: "https://github.com/vidhanio/checkpoint",
    },
    Link {
        name: "serenity-commands",
        description: "a library for creating/parsing serenity slash commands.",
        href: "https://github.com/vidhanio/serenity-commands",
    },
];

const CONTACTS: &[Link] = &[
    Link {
        name: "email",
        description: "me@vidhan.io",
        href: "mailto:me@vidhan.io",
    },
    Link {
        name: "github",
        description: "vidhanio",
        href: "https://github.com/vidhanio",
    },
    Link {
        name: "twitter",
        description: "@vidhanio",
        href: "https://twitter.com/vidhanio",
    },
    Link {
        name: "linkedin",
        description: "/in/vidhanio",
        href: "https://www.linkedin.com/in/vidhanio",
    },
];

#[instrument(level = "debug")]
pub async fn home(doc: DocumentParts) -> Document<impl Renderable> {
    doc.build_simple(maud! {
        header {
            h1 { "👋 hi, i'm vidhan!" }
            hr;
        }

        section #about {
            p {
                "welcome to my personal website!"
                br;
                br;
                "i'm a software engineer and a computer science student at mcmaster university. \
                my favourite programming language is rust, but i also enjoy writing python. \
                i also love basketball! \
                my favourite player is lebron james and i'm a huge fan of the los angeles lakers."
            }

            a.box href=(cached!("/resume.pdf")) {
                b { "📄 resume.pdf" }
            }
        }

        section #posts {
            h2 { "📝 posts" }
            hr;
            ul {
                @for post in Post::ALL {
                    li {
                        a href={"/post/" (post.slug)} {
                            time datetime=(post.date_dashed()) {
                                (post.date_slashed())
                            }
                            " - "
                            b {
                                (post.title)
                            }
                        }
                    }
                }
            }
        }

        section #projects {
            h2 { "🛠️ projects" }
            hr;
            ul {
                @for &project in PROJECTS {
                    li { (project) }
                }
            }
        }

        section #contact {
            h2 { "💬 contact" }
            hr;
            ul {
                @for &contact in CONTACTS {
                    li { (contact) }
                }
            }
        }
    })
}

#[instrument(level = "debug", err(Debug))]
pub async fn post(
    doc: DocumentParts,
    Path(slug): Path<String>,
) -> SiteResult<Document<impl Renderable>> {
    let post = Post::get(&slug).ok_or(SiteError::PostNotFound(slug))?;

    Ok(doc.build(
        post.title,
        format!("/post/{}/og.png", post.slug),
        maud_move! {
            header {
                h1 {
                    (post.title)
                }
                time datetime=(post.date_dashed()) {
                    (post.date_slashed())
                }
                hr;
            }

            article {
                (post.content)

                @if !post.footnotes.is_empty() {
                    h2 #footnotes {
                        a href="#footnotes" { "footnotes" }
                    }

                    @for &(name, content) in post.footnotes {
                        p id={ "footnote-" (name) } {
                            a.footnote href={"#footnote-" (name)} {
                                "[" (name) "]"
                            }
                            " "
                            (content)
                        }
                    }
                }
            }
        },
    ))
}
