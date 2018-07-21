
use gtk;
use gtk::prelude::*;
use relm::Widget;
use relm_attributes::widget;
use gtk::Orientation;

pub struct Model {
    pub relm: ::relm::Relm<Procedure>,
    pub id: usize,
    pub name: String,
}

#[derive(Msg)]
pub enum Msg {
    EditName(String),
    Remove,
}

#[widget]
impl Widget for Procedure {
    fn model(relm: &::relm::Relm<Self>, id: usize) -> Model {
        Model {
            relm: relm.clone(),
            id: id,
            name: String::new(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::EditName(name) => {
                self.model.name = name;
            }
            Msg::Remove => (),
        }
    }

    view! {
        gtk::Box {
            orientation: Orientation::Horizontal,
            gtk::Entry {
                changed(entry) => Msg::EditName(entry.get_text().unwrap()),
            },
            gtk::Button {
                label: "[-]",
                clicked(_) => Msg::Remove,
            },
        }
    }
}
