use std::ops::ControlFlow;

pub trait IntoControlFlow {
    type Item;
    fn continue_or<B>(self, b: B) -> ControlFlow<B, Self::Item>;
    fn continue_or_else<B, F>(self, b: F) -> ControlFlow<B, Self::Item>
    where
        F: FnOnce() -> B;
    fn break_or<C>(self, c: C) -> ControlFlow<Self::Item, C>;
    fn break_or_else<C, F>(self, c: F) -> ControlFlow<Self::Item, C>
    where
        F: FnOnce() -> C;
}

impl<T> IntoControlFlow for Option<T> {
    type Item = T;

    fn continue_or<B>(self, b: B) -> ControlFlow<B, Self::Item> {
        self.continue_or_else(|| b)
    }

    fn continue_or_else<B, F>(self, b: F) -> ControlFlow<B, Self::Item>
    where
        F: FnOnce() -> B,
    {
        match self {
            Some(v) => ControlFlow::Continue(v),
            None => ControlFlow::Break(b()),
        }
    }

    fn break_or<C>(self, c: C) -> ControlFlow<Self::Item, C> {
        self.break_or_else(|| c)
    }

    fn break_or_else<C, F>(self, c: F) -> ControlFlow<Self::Item, C>
    where
        F: FnOnce() -> C,
    {
        match self {
            Some(v) => ControlFlow::Break(v),
            None => ControlFlow::Continue(c()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::ControlFlow;

    use crate::IntoControlFlow;

    #[test]
    fn test_value() {
        assert_eq!(Some(1).continue_or(2), ControlFlow::Continue(1));
        assert_eq!(None::<i32>.continue_or(2), ControlFlow::Break(2));
        assert_eq!(Some(1).break_or(2), ControlFlow::Break(1));
        assert_eq!(None::<i32>.break_or(2), ControlFlow::Continue(2));
    }

    #[test]
    fn test_continue_or_else() {
        let mut called = false;
        assert_eq!(
            Some(1).continue_or_else(|| {
                called = true;
                2
            }),
            ControlFlow::Continue(1)
        );
        assert!(!called);

        assert_eq!(None::<i32>.continue_or_else(|| 2), ControlFlow::Break(2));
    }

    #[test]
    fn test_break_or_else() {
        let mut called = false;
        assert_eq!(
            Some(1).break_or_else(|| {
                called = true;
                2
            }),
            ControlFlow::Break(1)
        );
        assert!(!called);

        assert_eq!(None::<i32>.break_or_else(|| 2), ControlFlow::Continue(2));
    }
}
