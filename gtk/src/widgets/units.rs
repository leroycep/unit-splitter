
use gtk;
use gtk::prelude::*;
use relm::Widget;
use relm_attributes::widget;
use gtk::TextBuffer;
use core::group::Group;
use core::parse::parse_units;

#[derive(Clone)]
pub struct Model {
    pub relm: ::relm::Relm<Units>,
    pub buffer: TextBuffer,
    pub units: Result<Vec<Group>, ()>,
}

#[derive(Msg)]
pub enum Msg {
    TextEdited,
    UpdateUnits(Vec<Group>),
}

#[widget]
impl Widget for Units {
    fn init_view(&mut self) {
        self.text_view.set_buffer(&self.model.buffer);

        connect!(
            self.model.relm,
            self.model.buffer,
            connect_end_user_action(_),
            Msg::TextEdited
        );
    }

    fn model(relm: &::relm::Relm<Self>, _: ()) -> Model {
        let buffer = TextBuffer::new(None);
        Model {
            relm: relm.clone(),
            buffer: buffer,
            units: Ok(vec![]),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::TextEdited => {
                let (start, end) = self.model.buffer.get_bounds();
                let text = self.model.buffer.get_text(&start, &end, false).unwrap_or(String::new());
                match parse_units(&text) {
                    Ok(units) => {
                        self.model.units = Ok(units.clone());
                        self.model.relm.stream().emit(Msg::UpdateUnits(units));
                    }
                    Err(e) => {
                        println!("error parsing units: {:?}", e);
                    }
                }
            }
            // \/ For other users to listen to
            Msg::UpdateUnits(_) => (),
        }
    }

    view! {
        #[name="text_view"]
        gtk::TextView {
        },
    }
}
