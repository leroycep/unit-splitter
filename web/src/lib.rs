use unit_splitter_core::group::{Group, Groups};
use unit_splitter_core::inventory::{self, InventoryParseResult};
use unit_splitter_core::requests::{self, RequestsParseResult};
use unit_splitter_core::split::{self, Split, SplitResult};

const TITLE: &'static str = "Unit Splitter";
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn get_version() -> &'static str {
    VERSION.unwrap_or("unknown")
}

pub struct Model {
    inventory_string: String,
    requests_string: String,
    inventory: InventoryParseResult,
    requests: RequestsParseResult,
    split: SplitResult,
}

#[derive(Debug)]
pub enum Msg {
    GotInventoryString(String),
    GotRequestString(String),
}

impl Default for Model {
    fn default() -> Self {
        Model {
            inventory_string: "".into(),
            requests_string: "".into(),
            inventory: inventory::parse(""),
            requests: requests::parse(""),
            split: Ok(unit_splitter_core::split::Split {
                filled_requests: std::collections::HashMap::new(),
                leftover_ranges: Vec::new(),
            }),
        }
    }
}

impl draco::App for Model {
    type Message = Msg;

    fn update(&mut self, _: &draco::Mailbox<Self::Message>, msg: Self::Message) {
        use crate::Msg::*;
        match msg {
            GotInventoryString(value) => {
                self.inventory_string = value;
                self.inventory = inventory::parse(&self.inventory_string);
            }
            GotRequestString(value) => {
                self.requests_string = value;
                self.requests = requests::parse(&self.requests_string);
            }
        }
        match (&self.inventory, &self.requests) {
            (Ok(inventory), Ok(requests)) => {
                self.split = split::split(&inventory, &requests);
            }
            _ => {
                // TODO: Make it apparent when output and input are desynchronized?
            }
        };
    }

    fn render(&self) -> draco::Node<Self::Message> {
        use draco::html as h;
        h::div()
            .class("container")
            .push(self.view_header())
            .push(h::nav())
            .push(self.view_main())
            .push(h::aside())
            .push(self.view_footer())
            .into()
    }
}

impl Model {
    fn view_header(&self) -> draco::Node<Msg> {
        use draco::html as h;
        h::header().push(TITLE).into()
    }

    fn view_main(&self) -> draco::Node<Msg> {
        use draco::html as h;
        use std::fmt::Write;

        let mut inventory_errs_str = String::new();
        if let Err(ref errors) = self.inventory {
            for e in errors {
                writeln!(inventory_errs_str, "{}", e);
            }
        }
        let inventory_errs = h::div()
            .class("inventory-result")
            .push(h::pre().push(inventory_errs_str));

        let mut requests_errs_str = String::new();
        if let Err(ref errors) = self.requests {
            for e in errors {
                writeln!(requests_errs_str, "{}", e);
            }
        }
        let requests_errs = h::div()
            .class("requests-result")
            .push(h::pre().push(requests_errs_str));

        h::main()
            .class("app")
            .push(
                h::div().push(h::h1().push("Units")).push(
                    h::textarea()
                        .class("input")
                        .attr("placeholder", "enter inventory")
                        .attr("value", self.inventory_string.clone())
                        .on_input(Msg::GotInventoryString),
                ),
            )
            .push(inventory_errs)
            .push(
                h::div().push(h::h1().push("Requests")).push(
                    h::textarea()
                        .class("input")
                        .attr("placeholder", "enter requests")
                        .attr("value", self.requests_string.clone())
                        .on_input(Msg::GotRequestString),
                ),
            )
            .push(requests_errs)
            .push(
                h::div()
                    .push(h::h1().push("Output"))
                    .push(self.view_output()),
            )
            .into()
    }

    fn view_output(&self) -> draco::Node<Msg> {
        use draco::html as h;
        let div = h::div().class("output-grid");
        match &self.split {
            Ok(Split {
                filled_requests,
                leftover_ranges,
            }) => div
                .append(filled_requests.iter().map(view_filled_request))
                .push(view_filled_request(("Leftover Units", leftover_ranges)))
                .into(),
            Err(e) => div.push(format!("{}", e)).into(),
        }
    }

    fn view_footer(&self) -> draco::Node<Msg> {
        use draco::html as h;
        h::footer()
            .push(AUTHORS)
            .push(" ")
            .push(
                h::a()
                    .attr("href", "changelog.html")
                    .push("v")
                    .push(get_version()),
            )
            .into()
    }
}

fn view_filled_request<S: AsRef<str>, I: AsRef<[Group]>>(
    (request_name, inventory): (S, I),
) -> draco::Node<Msg> {
    use draco::html as h;
    h::div()
        .class("output-row")
        .push(
            h::div()
                .class("output-request-name")
                .push(request_name.as_ref()),
        )
        .push(
            h::div()
                .class("output-inventory")
                .push(Groups(inventory.as_ref())),
        )
        .into()
}
