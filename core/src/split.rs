use crate::group::Group;
use crate::range::Range;
use crate::request::Request;

pub type SplitResult = Result<Split, SplitError>;

pub fn split(inventory: &[Group], requests: &[Request]) -> SplitResult {
    let mut inventory = inventory.to_vec();
    let mut filled_requests = Vec::new();
    for request in requests {
        let mut groups_used_ranges = vec![];
        for (group_idx, amount) in request.amounts().iter().enumerate() {
            let group = match inventory.get_mut(group_idx) {
                Some(group) => group,
                None => {
                    // This request is asking for units from a non existant group.
                    // Is an error, because it would only allow excluding groups
                    // at the end. May be added back as a feature later.
                    return Err(SplitError::TooManyGroupsRequested {
                        request_name: request.name().into(),
                    });
                }
            };

            let unused = {
                // TODO: NonLexicalLifetimes strikes again!
                let ranges = group.ranges();

                match split_ranges(ranges, *amount) {
                    Ok((used, unused)) => {
                        groups_used_ranges.push(group.with_ranges(used));
                        unused
                    }

                    Err(amount_needed) => {
                        return Err(SplitError::NotEnough {
                            group_name: group.name().into(),
                            amount_needed: amount_needed,
                        });
                    }
                }
            };
            *group = group.with_ranges(unused);
        }
        filled_requests.push((request.name().into(), groups_used_ranges));
    }
    return Ok(Split {
        filled_requests: filled_requests,
        leftover_ranges: inventory,
    });
}

#[derive(Debug, PartialEq)]
pub struct Split {
    pub filled_requests: Vec<(String, Vec<Group>)>,
    pub leftover_ranges: Vec<Group>,
}

#[derive(Fail, Debug, PartialEq)]
pub enum SplitError {
    #[fail(
        display = "There are not enough units in group {}. {} more needed",
        group_name, amount_needed
    )]
    NotEnough {
        group_name: String,
        amount_needed: u32,
    },

    #[fail(
        display = "The request \"{}\" is asking for units from a non-existant group.",
        request_name
    )]
    TooManyGroupsRequested { request_name: String },
}

fn split_ranges(ranges: &[Range], mut amount: u32) -> Result<(Vec<Range>, Vec<Range>), u32> {
    let mut ranges_iter = ranges.iter();
    let mut used_ranges = Vec::new();
    let mut unused_ranges = Vec::new();
    loop {
        let range = match ranges_iter.next() {
            Some(r) => r,
            None => break,
        };
        let (used, unused, amount_left) = range.split(amount);

        if let Some(used) = used {
            used_ranges.push(used);
        }
        amount = amount_left;
        if let Some(range) = unused {
            unused_ranges.push(range);
            break;
        }
    }
    if amount > 0 {
        Err(amount)
    } else {
        unused_ranges.extend(ranges_iter.map(|x| x.clone()));
        Ok((used_ranges, unused_ranges))
    }
}

#[cfg(test)]
mod tests {
    use crate::group::Group;
    use crate::range::Range;
    use crate::request::Request;
    use crate::split::{split, Split, SplitError};

    #[test]
    fn simple() {
        let inventory = vec![
            Group::new("A".into(), vec![Range::new(1, 100)]),
            Group::new("B".into(), vec![Range::new(101, 200)]),
            Group::new("C".into(), vec![Range::new(201, 300)]),
        ];
        let requests = vec![
            Request::new("X".into(), vec![32, 32, 32]),
            Request::new("Y".into(), vec![32, 32, 32]),
            Request::new("Z".into(), vec![32, 32, 32]),
        ];

        let result = split(&inventory, &requests);

        let mut expected_filled = Vec::new();
        expected_filled.push((
            "X".into(),
            vec![
                Group::new("A".into(), vec![Range::new(1, 32)]),
                Group::new("B".into(), vec![Range::new(101, 132)]),
                Group::new("C".into(), vec![Range::new(201, 232)]),
            ],
        ));
        expected_filled.push((
            "Y".into(),
            vec![
                Group::new("A".into(), vec![Range::new(33, 64)]),
                Group::new("B".into(), vec![Range::new(133, 164)]),
                Group::new("C".into(), vec![Range::new(233, 264)]),
            ],
        ));
        expected_filled.push((
            "Z".into(),
            vec![
                Group::new("A".into(), vec![Range::new(65, 96)]),
                Group::new("B".into(), vec![Range::new(165, 196)]),
                Group::new("C".into(), vec![Range::new(265, 296)]),
            ],
        ));

        assert_eq!(
            result,
            Ok(Split {
                filled_requests: expected_filled,
                leftover_ranges: vec![
                    Group::new("A".into(), vec![Range::new(97, 100)]),
                    Group::new("B".into(), vec![Range::new(197, 200)]),
                    Group::new("C".into(), vec![Range::new(297, 300)]),
                ],
            })
        );
    }

    #[test]
    fn brokeit() {
        // A=1-10,15,18
        // H: 5
        // J: 6
        let inventory = vec![Group::new("A".into(), vec![Range::new(1, 10), Range::num(15), Range::num(18)])];
        let requests = vec![Request::new("H".into(), vec![5]), Request::new("J".into(), vec![6])];

        let result = split(&inventory, &requests);

        let mut expected_filled = Vec::new();
        expected_filled.push((
            "H".into(),
            vec![
                Group::new("A".into(), vec![Range::new(1, 5)]),
            ],
        ));
        expected_filled.push((
            "J".into(),
            vec![
                Group::new("A".into(), vec![Range::new(6, 10), Range::num(15)]),
            ],
        ));

        assert_eq!(
            result,
            Ok(Split {
                filled_requests: expected_filled,
                leftover_ranges: vec![
                    Group::new("A".into(), vec![Range::num(18)]),
                ],
            })
        );
    }

    #[test]
    fn not_enough() {
        let inventory = vec![Group::new("A".into(), vec![Range::new(1, 10)])];
        let requests = vec![Request::new("X".into(), vec![32])];

        let result = split(&inventory, &requests);

        assert_eq!(
            result,
            Err(SplitError::NotEnough {
                group_name: "A".into(),
                amount_needed: 22,
            })
        );
    }

    #[test]
    fn greedy_request() {
        let inventory = vec![Group::new("A".into(), vec![Range::new(1, 10)])];
        let requests = vec![Request::new("X".into(), vec![10, 10])];

        let result = split(&inventory, &requests);

        assert_eq!(
            result,
            Err(SplitError::TooManyGroupsRequested {
                request_name: "X".into(),
            })
        );
    }

}
