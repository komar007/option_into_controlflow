use std::ops::ControlFlow;

pub trait OptionExt {
    /// The output value
    type Item;

    /// Transforms the output value into a [`Continue`] if present, or produces a [`Break(b())`]
    /// otherwise.
    ///
    /// [`Continue`]: ControlFlow::Continue
    /// [`Break(b())`]: ControlFlow::Break
    fn continue_or_else<B, F>(self, b: F) -> ControlFlow<B, Self::Item>
    where
        F: FnOnce() -> B;

    /// Transforms the output value into a [`Break`] if present, or produces a [`Continue(c())`]
    /// otherwise.
    ///
    /// [`Break`]: ControlFlow::Break
    /// [`Continue(c())`]: ControlFlow::Continue
    fn break_or_else<C, F>(self, c: F) -> ControlFlow<Self::Item, C>
    where
        F: FnOnce() -> C;

    /// Non-lazy version of [`continue_or_else`].
    ///
    /// [`continue_or_else`]: OptionExt::continue_or_else
    fn continue_or<B>(self, b: B) -> ControlFlow<B, Self::Item>
    where
        Self: Sized,
    {
        self.continue_or_else(|| b)
    }

    /// Default-value version of [`continue_or_else`].
    ///
    /// [`continue_or_else`]: OptionExt::continue_or_else
    fn continue_or_default<B>(self) -> ControlFlow<B, Self::Item>
    where
        Self: Sized,
        B: Default,
    {
        self.continue_or_else(Default::default)
    }

    /// Non-lazy version of [`break_or_else`].
    ///
    /// [`break_or_else`]: OptionExt::break_or_else
    fn break_or<C>(self, c: C) -> ControlFlow<Self::Item, C>
    where
        Self: Sized,
    {
        self.break_or_else(|| c)
    }

    /// Default-value version of [`break_or_else`].
    ///
    /// [`break_or_else`]: OptionExt::break_or_else
    fn break_or_default<C>(self) -> ControlFlow<Self::Item, C>
    where
        Self: Sized,
        C: Default,
    {
        self.break_or_else(Default::default)
    }
}

impl<T> OptionExt for Option<T> {
    type Item = T;

    /// Transforms the [`Option<T>`] into a [`ControlFlow<B, T>`], mapping [`Some(v)`] to
    /// [`Continue(v)`] and [`None`] to [`Break(b())`].
    ///
    /// [`Continue(v)`]: ControlFlow::Continue
    /// [`Break(b())`]: ControlFlow::Break
    /// [`Some(v)`]: Some
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::ops::ControlFlow;
    /// use into_controlflow::OptionExt as _;
    ///
    /// let x = Some("foo");
    /// assert_eq!(x.continue_or_else(|| 0), ControlFlow::Continue("foo"));
    ///
    /// let x: Option<&str> = None;
    /// assert_eq!(x.continue_or_else(|| 0), ControlFlow::Break(0));
    /// ```
    fn continue_or_else<B, F>(self, b: F) -> ControlFlow<B, T>
    where
        F: FnOnce() -> B,
    {
        match self {
            Some(v) => ControlFlow::Continue(v),
            None => ControlFlow::Break(b()),
        }
    }

    /// Transforms the [`Option<T>`] into a [`ControlFlow<T, C>`], mapping [`Some(v)`] to
    /// [`Break(v)`] and [`None`] to [`Continue(c())`].
    ///
    /// [`Continue(c())`]: ControlFlow::Continue
    /// [`Break(v)`]: ControlFlow::Break
    /// [`Some(v)`]: Some
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::ops::ControlFlow;
    /// use into_controlflow::OptionExt as _;
    ///
    /// let x = Some("foo");
    /// assert_eq!(x.break_or_else(|| 0), ControlFlow::Break("foo"));
    ///
    /// let x: Option<&str> = None;
    /// assert_eq!(x.break_or_else(|| 0), ControlFlow::Continue(0));
    /// ```
    fn break_or_else<C, F>(self, c: F) -> ControlFlow<T, C>
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

    use crate::OptionExt as _;

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

    #[test]
    fn test_default() {
        assert_eq!(
            Some(1).continue_or_default::<()>(),
            ControlFlow::Continue(1)
        );
        assert_eq!(None::<i32>.continue_or_default(), ControlFlow::Break(0));
        assert_eq!(Some(1).break_or_default::<()>(), ControlFlow::Break(1));
        assert_eq!(None::<i32>.break_or_default(), ControlFlow::Continue(0));
    }
}
