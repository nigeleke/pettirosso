use dioxus::prelude::*;
use dioxus_sdk::storage::{use_storage, LocalStorage};
use uuid::Uuid;

use crate::application::{Model, Page};
use crate::domain::Player;

#[component]
pub fn App() -> Element {
    let package_name = env!("CARGO_PKG_NAME");
    let package_version = env!("CARGO_PKG_VERSION");

    let model = use_storage::<LocalStorage, _>(package_name.into(), Model::default);
    provide_context(model);

    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/assets/css/style.css") }

        header { h1 { "Pettirosso" } }
        main {
            NavBar {}
            match model.read().page() {
                Page::PlayerPool => rsx! { PlayerPoolPage {} },
                Page::Registration => rsx! { RegistrationPage {} },
                Page::Round => rsx! { RoundPage {} },
            }
        }
        footer {
            "Version {package_version}. Written by Nigel Eke; 2026. "
            a {
                href: "https://opensource.org/license/bsd-3-clause",
                target: "_blank",
                rel: "noopener noreferrer",
                "BSD-3-Clause"
            }
            " License."
        }
    }
}

#[component]
fn NavBar() -> Element {
    let mut model = use_context::<Signal<Model>>();
    let page = *model.read().page();

    let page_class = |this: Page| {
        if this == page {
            "nav-bar__current-page"
        } else {
            "nav-bar__other-page"
        }
    };

    rsx! {
        div {
            class: "nav-bar",
            button {
                class: page_class(Page::PlayerPool),
                onclick: move |_| model.write().set_page(Page::PlayerPool),
                "Player Pool"
            }
            button {
                class: page_class(Page::Registration),
                onclick: move |_| model.write().set_page(Page::Registration),
                "Registration"
            }
            button {
                class: page_class(Page::Round),
                onclick: move |_| model.write().set_page(Page::Round),
                "Rounds"
            }
        }
    }
}

#[component]
fn PlayerPoolPage() -> Element {
    let mut model = use_context::<Signal<Model>>();

    rsx! {
        div {
            class: "player-pool",
            PlayerNameInput {
                on_add: move |name| model.write().add_player(name),
            }
            EditablePlayerList {
                players: Vec::from_iter(model().pool().cloned()),
                edit_action: "-",
                on_edit: move |id| model.write().remove_player(id),
            }
        }
    }
}

#[component]
fn PlayerNameInput(on_add: EventHandler<String>) -> Element {
    let mut value = use_signal(String::default);

    rsx! {
        div {
            class: "player-name-input",
            input {
                name: "player-name-input",
                placeholder: "Enter player name",
                value,
                onchange: move |e| value.set(e.value()),
            }
            button {
                onclick: move |_| {
                    on_add.call(value());
                    value.set(String::default())
                },
                "+"
            }
        }
    }
}

#[component]
fn EditablePlayerList(
    players: Vec<Player>,
    edit_action: String,
    on_edit: EventHandler<Uuid>,
) -> Element {
    rsx! {
        ul {
            class: "editable-player-list",
            for player in players {
                li {
                    class: "editable-player-list__item",
                    id: "{player.id}",
                    button {
                        onclick: move |_| on_edit.call(player.id),
                        {edit_action.clone()}
                    }
                    span { {player.name} }
                }
            }
        }
    }
}

#[component]
fn RegistrationPage() -> Element {
    let mut model = use_context::<Signal<Model>>();

    rsx! {
        div {
            class: "registration",
            span { "Available" }
            span { "Playing" }
            EditablePlayerList {
                players: Vec::from_iter(model.read().available().cloned()),
                edit_action: "+",
                on_edit: move |id| model.write().register(id),
            }
            EditablePlayerList {
                players: Vec::from_iter(model().registered().cloned()),
                edit_action: "-",
                on_edit: move |id| model.write().deregister(id)
            }
        }
    }
}

#[component]
fn RoundPage() -> Element {
    let mut model = use_context::<Signal<Model>>();
    let mut rounds = use_signal(Vec::default);
    let mut round_number = use_signal(|| 0);
    let mut round = use_signal(|| None);

    use_effect(move || {
        rounds.set(Vec::from_iter(model.read().rounds().cloned()));
        round_number.set(model.read().round_number());
    });

    use_effect(move || {
        round.set(rounds.read().get(*round_number.read()).cloned());
    });

    rsx! {
        div {
            class: "round",
            if rounds.is_empty() {
                span {"No players"}
            } else if let Some(round) = round() {
                div {
                    class: "round__matches-list",
                    for m in round.matches() {
                        span { {m.player1().to_string()} }
                        span { "vs" }
                        span { {m.player2().to_string()} }
                    }
                }
                div {
                    class: "round__navigation",
                    button {
                        disabled: *round_number.read() == 0,
                        onclick: move |_| round_number.with_mut(|n| *n -= 1),
                        "<"
                    }
                    span { "Round: {*round_number.read() + 1}" }
                    button {
                        disabled: *round_number.read() == rounds.read().len() - 1,
                        onclick: move |_| round_number.with_mut(|n| *n += 1),
                        ">"
                    }
                }
                div {
                    class: "round__reset",
                    button {
                        class: "round__reset__button",
                        onclick: move |_| model.write().generate_round_robin(),
                        "Reschedule with same players"
                    }
                }
            }
        }
    }
}
