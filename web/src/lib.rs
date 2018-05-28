extern crate stdweb;
#[macro_use]
extern crate yew;
extern crate unit_splitter_core as core;
extern crate slab;

use std::collections::VecDeque;
use std::collections::HashMap;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use slab::Slab;
use core::parse::parse_units;
use core::range::Range;
use core::split::{RequestId, GroupId};

pub struct Model {
    unit_string: String,
    tests: Slab<String>,
    groups: Slab<String>,
    group_ranges: HashMap<GroupId, VecDeque<Range>>,
    requests: HashMap<RequestId, usize>,
}

pub enum Msg {
    GotUnits(String),
    EditTestName(usize, String),
    RemoveTest(usize),
    AddTest,
    EditAmount(RequestId, String),
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<ConsoleService>,
{
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            unit_string: "".into(),
            tests: Slab::new(),
            groups: Slab::new(),
            group_ranges: HashMap::new(),
            requests: HashMap::new(),
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::GotUnits(value) => {
                self.unit_string = value;
                context.as_mut().log("unit string updated");
                let parse = parse_units(&self.unit_string);
                context.as_mut().log(&format!("parse: {:?}", parse));
                match parse {
                    Ok(parse) => {
                        context.as_mut().log(&format!("parse: {:?}", parse));
                        self.groups.clear();
                        for group in parse {
                            let group_id = self.groups.insert(group.name().to_string());
                            self.group_ranges.insert(group_id, group.ranges().to_vec().into());
                        }
                    }
                    Err(_) => { }
                }
            }
            Msg::EditTestName(idx, value) => {
                self.tests[idx] = value;
                context.as_mut().log("test name updated");
            }
            Msg::RemoveTest(idx) => {
                context.as_mut().log(&format!("removed test \"{}\" from tests", &self.tests[idx]));
                self.tests.remove(idx);
            }
            Msg::AddTest => {
                self.tests.insert(String::new());
                context.as_mut().log("added test");
            }
            Msg::EditAmount(amount_id, amount_string) => {
                if let Ok(amount) = amount_string.parse() {
                    self.requests.insert(amount_id, amount);
                    context.as_mut().log("edited amount");
                }
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
                <div>
                    <h1>{ "Units" }</h1>
                    <input type="text",
                        value=&self.unit_string,
                        oninput=|e: InputData| Msg::GotUnits(e.value),
                        placeholder="enter unit string",>
                    </input>
                </div>
                <div>
                    <h1>{ "Requests" }</h1>
                    <div>
                        <div><span>{"Test Name"}</span>{ self.view_group_headers() }</div>
                        { for self.tests.iter().map(|(i,name)| self.view_test(i, name)) }
                    </div>
                    <button onclick=|_| Msg::AddTest,>{ "[+]" }</button>
                </div>
                { self.view_output() }
            </div>
        }
    }
}

impl Model {
    fn view_group_headers<CTX>(&self) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static
    {
        html! {
            <span>
                { for self.groups.iter().map(|(_id, name)| html! { <span>{name}</span> }) }
            </span>
        }
    }

    fn view_test<CTX>(&self, test_id: usize, name: &str) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static
    {
        html! {
            <div>
                <button onclick=move |_| Msg::RemoveTest(test_id),>{ "[-]" }</button>
                <input
                    type="text",
                    value=name,
                    oninput=move |e: InputData| Msg::EditTestName(test_id, e.value),
                    placeholder="enter test name",></input>
                { self.view_requests(test_id) }
            </div>
        }
    }

    fn view_requests<CTX>(&self, test_id: usize) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static
    {
        html! {
            <span>
                { for self.groups.iter().map(|(group_id, _v)| self.view_request(test_id, group_id)) }
            </span>
        }
    }

    fn view_request<CTX>(&self, test_id: usize, group_id: usize) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static
    {
        let amount = self.requests.get(&RequestId {test_id, group_id}).map(|e| e.to_string()).unwrap_or_else(|| "".to_string());
        html! {
            <input
                type="number",
                value=amount,
                oninput=move |e: InputData| Msg::EditAmount(RequestId { test_id, group_id }, e.value),>
            </input>
        }
    }

    fn view_output<CTX>(&self) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static
    {
        use core::split::split;
        let (used_ranges, unused_ranges) = split(&self.group_ranges, &self.requests).unwrap();
        html! {
            <div>
                <h1>{ "Output" }</h1>
                { for used_ranges.iter().map(|(test_id, ranges)| self.view_test_ranges(&self.tests[*test_id], ranges)) }
                { self.view_test_ranges("Unused ranges", &unused_ranges) }
            </div>
        }
    }

    fn view_test_ranges<CTX>(&self, test_name: &str, group_ranges: &core::split::Ranges) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static
    {
        let mut ranges_string = String::new();
        let mut should_have_comma_groups = false;
        for (group_id, ranges) in group_ranges.iter() {
            if should_have_comma_groups {
                ranges_string.push_str(", ");
            }
            ranges_string.push_str(&self.groups[*group_id]);
            ranges_string.push_str("=");
            let mut should_have_comma = false;
            for range in ranges.iter() {
                if should_have_comma {
                    ranges_string.push_str(", ");
                }
                range.write_to_string(&mut ranges_string);
                should_have_comma = true;
            }
            should_have_comma_groups = true;
        }
        html! {
            <div>
                <span>{test_name}{": "}{ranges_string}</span>
            </div>
        }
    }
}
