#[macro_use]
extern crate quicli;
extern crate unit_splitter_core as app_core;

use quicli::prelude::*;
use app_core::group::Group;
use app_core::range::Range;
use app_core::unit_requests::UnitRequests;
use std::collections::HashMap;


#[derive(Debug, StructOpt)]
struct Cli {
    available_units: String,
    unit_requests: Vec<String>,
}

main!(|args: Cli| {
    let groups = app_core::parse::parse_units(&args.available_units)
        .context("Failed to parse the available units")?;
    let groups = validate_input(&groups)
        .context("Failed to validate available units")?;
    let total_unit_count: u32 = groups.iter().map(|g| g.count()).sum();
    println!("Dividing {} units between {} requests", total_unit_count, args.unit_requests.len());
});

fn validate_input<'a>(input: &[(Option<&'a str>, Vec<(u32, u32)>)]) -> Result<Vec<Group<'a>>> {
    let mut result = vec![];
    for (name, ranges) in input {
        let mut validated_ranges = vec![];
        for (first, last) in ranges {
            let range = Range::new(*first, *last)
                .context("Invalid range")?;
            validated_ranges.push(range);
        }
        result.push(Group::new(*name, validated_ranges));
    }
    Ok(result)
}

fn split_units<'a>(groups: &[Group], requests: UnitRequests) -> Result<HashMap<String, Vec<Group<'a>>>> {
    let mut result = HashMap::new();
    let mut units_left = Vec::from(groups);
    for ((request_id, group_name), amount) in requests.amounts().iter() {
        let mut used_ranges = vec![];
        let mut amount_left = amount;
        while *amount_left > 0 {
            let (used, left, amount_remaining) = collect_units_from_range();
        }
        let group;
        for range in group.ranges {
        }
    }
    Ok(result)
}

fn collect_units_from_range(range: &Range, amount: u32) -> (Range, Option<Range>, u32) {
    assert!(amount > 0);
    if amount >= range.count() {
        (range.clone(), None, amount - range.count())
    } else {
        let first2 = range.first() + amount;
        let range_used = Range::new(range.first(), first2 - 1).unwrap();
        let range_left = Range::new(first2, range.last()).unwrap();
        (range_used, Some(range_left), 0)
    }
}
