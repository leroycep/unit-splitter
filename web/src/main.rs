extern crate yew;
extern crate unit_splitter_web;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use unit_splitter_web::Model;

pub struct Context {
    console: ConsoleService,
}

impl AsMut<ConsoleService> for Context {
    fn as_mut(&mut self) -> &mut ConsoleService {
        &mut self.console
    }
}

fn main() {
    yew::initialize();
    let app: App<Model> = App::new();
    app.mount_to_body();
    yew::run_loop();
}
