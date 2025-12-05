use std::{any::type_name_of_val, slice};

pub fn name_of_type<T>(val: &T) -> &'static str {
    type_name_of_val(val).split("::").last().unwrap()
}

#[derive(Debug)]
pub struct Stack<T, A, B>
where
    T: Clone,
    A: AsRef<T> + AsMut<T>,
    B: AsRef<T>,
{
    stack: Vec<T>,
    head: A,
    tail: B,
    function: fn(&mut T, &T),
}

impl<T, A, B> Stack<T, A, B>
where
    T: Clone,
    A: AsRef<T> + AsMut<T>,
    B: AsRef<T>,
{
    pub fn new(head: A, tail: B, function: fn(&mut T, &T)) -> Self {
        Stack { stack: vec![], head, tail, function }
    }
    pub fn head(&self) -> &A {
        &self.head
    }
    pub fn head_mut(&mut self) -> &mut A {
        &mut self.head
    }
    pub fn set_head(&mut self, head: A) {
        self.head = head;
    }
    pub fn tail(&self) -> &B {
        &self.tail
    }
    pub fn tail_mut(&mut self) -> &mut B {
        &mut self.tail
    }
    pub fn set_tail(&mut self, tail: B) {
        self.tail = tail;
    }
    pub fn last(&self) -> &T {
        self.stack.last().unwrap_or(self.head.as_ref())
    }
    pub fn first(&self) -> &T {
        self.stack.first().unwrap_or(self.tail.as_ref())
    }
    pub fn pop(&mut self) -> T {
        self.stack.pop().unwrap_or(self.head.as_ref().clone())
    }
    pub fn pop_if(&mut self, predicate: impl FnOnce(&mut T) -> bool) -> Option<T> {
        self.stack.pop_if(predicate)
    }
    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }
    pub fn push_tail(&mut self) {
        self.stack.push(self.tail.as_ref().clone());
    }
    pub fn len(&self) -> usize {
        self.stack.len()
    }
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn iter_full(
        &self,
    ) -> std::iter::Chain<
        std::iter::Chain<std::array::IntoIter<&T, 1>, slice::Iter<'_, T>>,
        std::array::IntoIter<&T, 1>,
    > {
        [self.head.as_ref()].into_iter().chain(self.stack.iter()).chain([self.tail.as_ref()])
    }
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.stack.iter()
    }
    pub fn into_iter(self) -> <std::vec::Vec<T> as std::iter::IntoIterator>::IntoIter {
        self.stack.into_iter()
    }

    pub fn fold_into_self(self) -> Self {
        let Self { mut stack, function, .. } = self;
        stack.iter_mut().fold(Stack { stack: vec![], ..self }, |mut acc, mut current| {
            function(current, acc.last());
            acc.push(current.clone());
            acc
        })
    }
    pub fn rfold_into_self(self, init: &[T]) -> Self {
        let Self { mut stack, head, function, .. } = self;

        let mut stack = stack.iter_mut().rfold(
            Stack { head, stack: init.into(), ..self },
            |mut acc, mut current| {
                function(current, acc.last());
                acc.push(current.clone());
                acc
            },
        );

        let last = stack.last().clone();
        function(stack.head.as_mut(), &last);
        stack.stack.reverse();
        stack
    }
}

// todo Could make a more "case" specific collection
// Chain
//  - could keep elements "spaced". User should supply how much "space" should be between each item and a closure which can evaluate "space" between two items.
//      > When head or tail is updated, the list checks and updates all elements.
//      > "Space" between items could instead be calculated from the space between head and tail.
//      > GetSpaceToPreviousItem(..) -> T, T != SpaceBetweenItems => UpdateSpaceTowardsPreviousItemBy(T)
//  - Could execute a function on each element whenever head or tail is updated. User supplies a closure which takes the current and previous item, and returns an updated current item.
//      > This would be less specific than above. The above could probably be achieved in this version, with the right closure.
//      > Could be used on all kind of types, that have a relationship that require them to be updated when something happens to the "main" type (i.e. the head or tail)
//      > Probably only on types that have some inherent relationship, so they almost never have to be handled individually.
//  - What would be the benefit of using such a collection compared to e.g. Vec<T> and implement the relationship yourself?
//      > It would guarantee that all items are updated when the head/tail is updated.

// impl<T, A, B> TryFrom<&[T]> for Stack<T, A, B>
// where
//     T: Clone,
//     A: AsRef<T>,
//     B: AsRef<T>,
// {
//     type Error = anyhow::Error;

//     fn try_from(slice: &[T]) -> std::result::Result<Self, Self::Error> {
//         match slice {
//             [] => Err(anyhow!("Cannot convert to Stack from empty list")),
//             [single] => Ok(Stack {
//                 stack: vec![],
//                 head: single.clone(),
//                 tail: single.clone(),
//             }),
//             [head, stack @ .., tail] => Ok(Stack {
//                 stack: stack.into(),
//                 head: head.clone(),
//                 tail: tail.clone(),
//             }),
//         }
//     }
// }

fn match_slice_examples(vec: &[i32]) {
    match vec {
        [] => println!("Empty"),
        [head] => println!("1 element: {:?}", head),
        [-1, head, middle @ .., tail] => {
            println!("1: head, middle, tail: {:?}, {:?}, {:?}", head, middle, tail)
        }
        [-2, head, tail @ ..] => println!("2: head, tail: {:?}, {:?}", head, tail),
        [-3, head, middle @ .., _] => println!("3: head, middle: {:?}, {:?}", head, middle),
        [..] => println!("Catch all"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let vec = vec![0];
        match_slice_examples(&vec);
        let vec = vec![-1, 1, 2, 3, 4, 5];
        match_slice_examples(&vec);
        let vec = vec![-2, 1, 2, 3, 4, 5];
        match_slice_examples(&vec);
        let vec = vec![-3, 1, 2, 3, 4, 5];
        match_slice_examples(&vec);
    }
}