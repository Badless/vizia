use std::collections::VecDeque;

use crate::{DoubleEndedTreeTour, LayoutChildIterator, TourDirection, TourStep, Tree};
use vizia_id::GenerationalId;

/// Iterator for iterating through the tree in depth first preorder.
pub struct TreeIterator<'a, I>
where
    I: GenerationalId,
{
    tree: &'a Tree<I>,
    tours: DoubleEndedTreeTour<I>,
}

impl<'a, I> TreeIterator<'a, I>
where
    I: GenerationalId,
{
    pub fn new(tree: &'a Tree<I>, tours: DoubleEndedTreeTour<I>) -> Self {
        Self { tree, tours }
    }

    pub fn full(tree: &'a Tree<I>) -> Self {
        Self::subtree(tree, I::root())
    }

    pub fn subtree(tree: &'a Tree<I>, root: I) -> Self {
        Self { tree, tours: DoubleEndedTreeTour::new_same(Some(root)) }
    }
}

impl<I> Iterator for TreeIterator<'_, I>
where
    I: GenerationalId,
{
    type Item = I;
    fn next(&mut self) -> Option<I> {
        self.tours.next_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => (Some(node), TourStep::EnterFirstChild),
            TourDirection::Leaving => (None, TourStep::EnterNextSibling),
        })
    }
}

impl<I> DoubleEndedIterator for TreeIterator<'_, I>
where
    I: GenerationalId,
{
    fn next_back(&mut self) -> Option<I> {
        self.tours.next_back_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => (None, TourStep::EnterLastChild),
            TourDirection::Leaving => (Some(node), TourStep::EnterPrevSibling),
        })
    }
}

pub struct TreeBreadthIterator<'a, I>
where
    I: GenerationalId,
{
    tree: &'a Tree<I>,
    queue: VecDeque<I>,
}

impl<'a, I> TreeBreadthIterator<'a, I>
where
    I: GenerationalId,
{
    pub fn full(tree: &'a Tree<I>) -> Self {
        Self::subtree(tree, I::root())
    }

    pub fn subtree(tree: &'a Tree<I>, root: I) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        Self { tree, queue }
    }
}

impl<I> Iterator for TreeBreadthIterator<'_, I>
where
    I: GenerationalId,
{
    type Item = I;
    fn next(&mut self) -> Option<I> {
        if let Some(item) = self.queue.pop_front() {
            let child_iter = LayoutChildIterator::new(self.tree, item);
            for child in child_iter {
                self.queue.push_back(child);
            }

            return Some(item);
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TreeError;
    use vizia_id::{
        impl_generational_id, GENERATIONAL_ID_GENERATION_MASK, GENERATIONAL_ID_INDEX_BITS,
        GENERATIONAL_ID_INDEX_MASK,
    };

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Entity(u64);

    impl_generational_id!(Entity);

    #[test]
    fn simple_forward_backward() -> Result<(), TreeError> {
        let mut t = Tree::new();
        let r = Entity::root();
        let [a, b, c, d, e] = [1, 2, 3, 4, 5].map(|i| Entity::new(i, 0));
        t.add(a, r)?;
        t.add(b, r)?;
        t.add(c, a)?;
        t.add(d, a)?;
        t.add(e, b)?;
        let correct = [r, a, c, d, b, e];
        let forward = TreeIterator::full(&t);
        let backward = TreeIterator::full(&t).rev();
        assert!(forward.eq(correct.iter().cloned()));
        assert!(backward.eq(correct.iter().cloned().rev()));

        // correct DoubleEndedIterator behavior, each item yielded only once
        let mut double = TreeIterator::full(&t);
        let mut front = Vec::new();
        let mut back = Vec::new();
        loop {
            if let Some(x) = double.next() {
                front.push(x);
            }
            if let Some(x) = double.next_back() {
                back.push(x);
            } else {
                break;
            }
        }
        back.reverse();
        front.append(&mut back);
        assert!(front.iter().eq(correct.iter()));

        let correct = [a, c, d];
        let forward = TreeIterator::subtree(&t, a);
        let backward = TreeIterator::subtree(&t, a).rev();
        assert!(forward.eq(correct.iter().cloned()));
        assert!(backward.eq(correct.iter().cloned().rev()));
        Ok(())
    }

    #[test]
    fn simple_forward_bfs() -> Result<(), TreeError> {
        let mut t = Tree::new();
        let r = Entity::root();
        let [a, b, c, d, e] = [1, 2, 3, 4, 5].map(|i| Entity::new(i, 0));
        t.add(a, r)?;
        t.add(b, r)?;
        t.add(c, a)?;
        t.add(d, a)?;
        t.add(e, b)?;
        let correct = [r, a, b, c, d, e];
        let forward = TreeBreadthIterator::full(&t);

        assert!(forward.eq(correct.iter().cloned()));

        Ok(())
    }
}
