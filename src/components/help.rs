use dioxus::prelude::*;
use math_macro::math;

use crate::{components::{VIDEO_GAMEPLAY, rem}, game::{ColorSkin, GameState, ScreenState}};

#[component]
fn Emph(children: Element) -> Element {
    rsx! {
        strong {
            color: "#ff0",
            {children}
        }
    }
}


#[component]
pub fn Help(game_state: Signal<GameState>) -> Element {
    let st = game_state.read();
    let skin = st.skin;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; font-size: 3.75rem; color: #fff; padding: 4rem;",
            class: "help",

            div {
                text_align: "left",

                p {
                    margin_top: "0",
                    "The ",Emph {"tableau"}," consists of 8 columns. Cards in the tableau are stacked by "
                    Emph{"decrementing ranks"} " and " Emph{"alternating color"} " (",
                    {if skin.colors != ColorSkin::FourColor {"red/black"} else {"warm/cool"}}
                    ,"). Only one card may be moved at a time (but see the later section on ",Emph {"supermoves"},"). 
                    Any card may be moved into an empty tableau column."
                }

                p {
                    "There are four ",Emph{"free cells"}," that may each store a single card of any kind."
                }

                p {
                    "To ",Emph{"win the game"},", stack all the cards to the " Emph{"foundations"} " in incrementing order by suit."
                }

                p {
                    Emph{"Shortcut notes:"},
                    ul {
                        li {
                            Emph{"Supermoves:"},
                            " Even though the rules only allow one card to be moved at a time, this app implements ",
                            Emph{"supermoves"},", which allows several cards in sequence to be moved, if there are enough empty
                            free cells and/or columns to make the intermediate moves. The maximum number of cards that 
                            may be moved to a ",Emph{"filled"}," column is ",
                            span { 
                                dangerous_inner_html: math!(r"C = (N+1) \cdot 2^M")
                            },
                            " , where ",
                            span { 
                                dangerous_inner_html: math!("N")
                            },
                            " is the number of empty free cells, and ",
                            span { 
                                dangerous_inner_html: math!("M")
                            },
                            " is the number of empty columns. The maximum number that may be moved to an ",Emph{"empty"}," column is ",
                            span { 
                                dangerous_inner_html: math!("C/2")
                            }, " ."
                        }

                        li {
                            Emph{"Double click:"},
                            " Double-clicking on a card will try to move it to the foundations if possible, or a free cell otherwise."
                        }
                    }
                }

                div {
                    position: "absolute",
                    bottom: rem(2.),
                    width: "92rem",
                    display: "flex",
                    justify_content: "center",

                    a {
                        href: VIDEO_GAMEPLAY,
                        target: "_blank",
                        text_decoration: "none",
                        margin_right: rem(4.),
                        div {
                            width: rem(30.),
                            position: "relative",
                            class: "game-button",
                            "Example video"
                        }
                    }

                    div {
                        width: rem(30.),
                        position: "relative",
                        class: "game-button",
                        onclick: move |_| game_state.write().screen_state = ScreenState::Game,
                        "Back to game"
                    }
                }
            }
        }
    }
}