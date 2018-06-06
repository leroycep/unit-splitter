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
use core::split::{RequestId, GroupId, TestId};

pub struct Model {
    unit_string: String,
    // Todo: Make this hold the actual error associated with the parsing
    unit_string_is_valid: bool,
    tests: Slab<String>,
    groups: Vec<String>,
    test_order: Vec<TestId>,
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
            unit_string_is_valid: false,
            tests: Slab::new(),
            groups: Vec::new(),
            group_ranges: HashMap::new(),
            test_order: vec![],
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
                            let group_id = self.groups.len();
                            self.groups.push(group.name().to_string());
                            self.group_ranges.insert(group_id, group.ranges().to_vec().into());
                        }
                        self.unit_string_is_valid = true;
                    }
                    Err(_) => {
                        self.unit_string_is_valid = false;
                    }
                }
            }
            Msg::EditTestName(idx, value) => {
                self.tests[idx] = value;
                context.as_mut().log("test name updated");
            }
            Msg::RemoveTest(idx) => {
                context.as_mut().log(&format!("removed test \"{}\" from tests", &self.tests[idx]));
                self.remove_test(idx);
            }
            Msg::AddTest => {
                self.add_test();
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
            <div class="body",>
                <div>
                    <h1>{ "Units" }</h1>
                    <textarea class=("indent", "unit-string-input", if self.unit_string_is_valid { "valid" } else { "invalid" }),
                        value=&self.unit_string,
                        oninput=|e: InputData| Msg::GotUnits(e.value),
                        placeholder="enter unit string",>
                    </textarea>
                </div>
                <div>
                    <h1>{ "Requests" }</h1>
                    <div class="indent",>
                        <table>
                            <tr><td></td><td>{"Test Name"}</td>{ self.view_group_headers() }</tr>
                            { for self.test_order.iter().map(|request_id| self.view_test(*request_id)) }
                            <tr><td><button class="button", onclick=|_| Msg::AddTest,>{ "[+]" }</button></td></tr>
                        </table>
                    </div>
                </div>
                { self.view_output() }
            </div>
        }
    }
}

impl Model {
    fn add_test(&mut self) {
        let test_id = self.tests.insert(String::new());
        self.test_order.push(test_id);
    }

    fn remove_test(&mut self, test_id: TestId) {
        for group_id in 0..self.groups.len() {
            self.requests.remove(&RequestId { test_id, group_id });
        }
        self.tests.remove(test_id);
        self.test_order.retain(|x| *x != test_id);
    }

    fn view_group_headers<CTX>(&self) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static
    {
        html! {
            { for self.groups.iter().map(|name| html! { <td>{name}</td> }) }
        }
    }

    fn view_test<CTX>(&self, test_id: usize) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static
    {
        let name = &self.tests[test_id];
        html! {
            <tr>
                <td><button class="button", onclick=move |_| Msg::RemoveTest(test_id),>{ "[-]" }</button></td>
                <td><input
                    type="text",
                    value=name,
                    oninput=move |e: InputData| Msg::EditTestName(test_id, e.value),
                    placeholder="enter test name",></input></td>
                { self.view_requests(test_id) }
            </tr>
        }
    }

    fn view_requests<CTX>(&self, test_id: usize) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static
    {
        html! {
            { for self.groups.iter().enumerate().map(|(group_id, _v)| html!{ <td>{ self.view_request(test_id, group_id) }</td> }) }
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
        let groups: Vec<_> = (0 .. self.groups.len()).collect();
        match split(&self.group_ranges, &self.requests, &self.test_order[..], &groups[..]) {
            Ok((used_ranges, unused_ranges)) => html! {
                <div>
                    <h1>{ "Output" }</h1>
                    <div class="indent",>
                        <div>{
                            for self.test_order.iter().map(|&test_id| {
                                let name = &self.tests[test_id];
                                let ranges = used_ranges.get(&test_id).expect("all requests should end up in `used_ranges`");
                                self.view_test_ranges(name, ranges)
                            })
                        }</div>
                        <div>{ self.view_test_ranges("Unused ranges", &unused_ranges) }</div>
                    </div>
                </div>
            },
            Err(()) => html! {
                <div>
                    <h1>{ "Output" }</h1>
                    <div class="indent",>
                        { "Unable to split units into requests. Do you have enough units?" }
                    </div>
                </div>
            },
        }
    }

    fn view_test_ranges<CTX>(&self, test_name: &str, group_ranges: &core::split::Ranges) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static
    {
        let mut ranges_string = String::new();
        let mut should_have_comma_groups = false;
        for (group_id, group_name) in self.groups.iter().enumerate() {
            let ranges = group_ranges.get(&group_id);
            if ranges.is_none() {
                continue;
            }
            let ranges = ranges.expect("loop will continue if this is none");

            if should_have_comma_groups {
                ranges_string.push_str(", ");
            }
            ranges_string.push_str(group_name);
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
