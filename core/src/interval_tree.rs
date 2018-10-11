// Used to learn from: https://www.geeksforgeeks.org/interval-tree/

use ::range::Range;

pub struct IntervalTree<D: Clone> {
    node: Option<IntervalTreeNode<D>>,
}
impl<D: Clone> IntervalTree<D> {
    pub fn new() -> Self {
        Self {
            node: None,
        }
    }

    pub fn insert(&mut self, interval: Range, data: D) {
        match self.node {
            Some(ref mut node) => node.insert(interval, data),
            None => self.node = Some(IntervalTreeNode::new(interval, data)),
        }
    }

    pub fn overlap_search(&self, interval: &Range, overlaps: &mut Vec<(Range, D)>) {
        if let Some(ref node) = self.node {
            node.overlap_search(interval, overlaps);
        }
    }
}

pub struct IntervalTreeNode<D: Clone> {
    data: D,
    interval: Range,
    max: usize,
    left: Option<Box<IntervalTreeNode<D>>>,
    right: Option<Box<IntervalTreeNode<D>>>,
}

impl<D: Clone> IntervalTreeNode<D> {
    pub fn new(interval: Range, data: D) -> Self {
        let max = interval.last();
        let left = None;
        let right = None;
        IntervalTreeNode {
            data,
            interval,
            max,
            left,
            right
        }
    }

    pub fn insert(&mut self, interval: Range, data: D) {
        let l = self.interval.first();

        if self.max < interval.last() {
            self.max = interval.last();
        }

        if interval.first() < l {
            match self.left {
                Some(ref mut node) => {
                    node.insert(interval, data);
                }
                None => {
                    self.left = Some(Box::new(IntervalTreeNode::new(interval, data)));
                }
            }
        } else {
            match self.right {
                Some(ref mut node) => {
                    node.insert(interval, data);
                }
                None => {
                    self.right = Some(Box::new(IntervalTreeNode::new(interval, data)));
                }
            }
        }
    }

    pub fn overlap_search(&self, interval: &Range, overlaps: &mut Vec<(Range, D)>) {
        if interval.overlaps(&self.interval) {
            overlaps.push((self.interval.clone(), self.data.clone()));
        }

        if let Some(ref node) = self.left {
            if node.max >= interval.first() {
                node.overlap_search(interval, overlaps);
            }
        }

        if let Some(ref node) = self.right {
            node.overlap_search(interval, overlaps);
        }
    }
}
