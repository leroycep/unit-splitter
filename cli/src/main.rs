#[macro_use]
extern crate quicli;
extern crate unit_splitter_core as core;

use core::group::Groups;
use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Cli {
    available_units: String,
    unit_requests: Vec<String>,
}

main!(|args: Cli| {
    let inventory = {
        let result = core::inventory::parse(&args.available_units);
        match result {
            Ok(r) => r,
            Err(errors) => {
                println!("Error parsing inventory:");
                for e in errors {
                    println!("  {}", e);
                }
                return Ok(());
            }
        }
    };
    let requests = {
        let result = core::requests::parse(&args.unit_requests.join(" "));
        match result {
            Ok(r) => r,
            Err(errors) => {
                println!("Error parsing requests:");
                for e in errors {
                    println!("  {}", e);
                }
                return Ok(());
            }
        }
    };
    let total_unit_count: u32 = inventory.iter().map(|g| g.count()).sum();
    println!(
        "Dividing {} units between {} requests",
        total_unit_count,
        args.unit_requests.len()
    );

    let split = core::split::split(&inventory, &requests);
    match split {
        Ok(core::split::Split {
            filled_requests,
            leftover_ranges,
        }) => {
            println!("");
            for (request_name, inventory) in filled_requests.iter() {
                println!("{}: {}", request_name, Groups(inventory));
            }
            println!("Leftover Units: {}", Groups(&leftover_ranges));
        }
        Err(e) => {
            println!("Error splitting units: {}", e);
        }
    }
});
