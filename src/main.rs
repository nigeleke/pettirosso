mod application;
mod domain;
mod ui;

use crate::ui::App;

fn main() {
    dioxus::launch(App);
}
