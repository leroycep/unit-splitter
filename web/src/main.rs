extern crate unit_splitter_web;
extern crate yew;

use unit_splitter_web::Model;
use yew::prelude::*;
use yew::services::console::ConsoleService;

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
