
use gtk;
use gtk::prelude::*;
use relm::Widget;
use relm_attributes::widget;
use gtk::Orientation;

#[derive(Clone)]
pub struct Model {
    pub relm: ::relm::Relm<Procedure>,
    pub id: usize,
    pub name: String,
}

#[derive(Msg)]
pub enum Msg {
    EditName(String),
    RemoveClicked,
    ChangeName(String),
    Remove,
}

#[widget]
impl Widget for Procedure {
    fn init_view(&mut self) {
        self.entry.set_text(&self.model.name);
    }

    fn model(relm: &::relm::Relm<Self>, (id, name): (usize, String)) -> Model {
        Model {
            relm: relm.clone(),
            id: id,
            name: name,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::EditName(name) => {
                self.model.name = name.clone();
                self.model.relm.stream().emit(Msg::ChangeName(name));
            }
            Msg::RemoveClicked => {
                self.model.relm.stream().emit(Msg::Remove);
            }
            // \/ For other users to listen to
            Msg::ChangeName(_) => (),
            Msg::Remove => (),
        }
    }

    view! {
        gtk::Box {
            orientation: Orientation::Horizontal,
            #[name="entry"]
            gtk::Entry {
                child: {
                    fill: true,
                    expand: true,
                },
                changed(entry) => Msg::EditName(entry.get_text().unwrap()),
            },
            gtk::Button {
                label: "[-]",
                clicked(_) => Msg::RemoveClicked,
            },
        }
    }
}
