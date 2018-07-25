use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    UpdateOutput(Vec<(String, String)>),
}

impl Output {
    fn update_output(&mut self, output: &[(String, String)]) {
        let mut text = String::new();
        for (procedure_name, units) in output.into_iter() {
            use std::fmt::Write;
            write!(text, "\n{}: <span size='20000'>{}</span>", procedure_name, units);
        }
        self.label.set_markup(text.as_str());
    }
}

/// This widget shows the generated output. May be extended to something more than a label
/// in the future.
#[widget]
impl ::relm::Widget for Output {
    fn model() -> () {
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::UpdateOutput(output) => {
                self.update_output(&output);
            }
        }
    }

    view! {
        #[name="label"]
        gtk::Label {
            selectable: true
        }
    }
}
