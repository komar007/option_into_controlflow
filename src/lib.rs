use std::ops::ControlFlow;

pub trait OptionExt {
    /// The type wrapped by [`Option`]
    type Item;

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
    fn continue_or_else<B, F>(self, b: F) -> ControlFlow<B, Self::Item>
    where
        F: FnOnce() -> B;

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
    fn break_or_else<C, F>(self, c: F) -> ControlFlow<Self::Item, C>
    where
        F: FnOnce() -> C;

    /// Transforms the [`Option<T>`] into a [`ControlFlow<B, T>`], mapping [`Some(v)`] to
    /// [`Continue(v)`] and [`None`] to [`Break(b)`].
    ///
    /// A non-lazy version of [`continue_or_else`].
    ///
    /// [`Continue(v)`]: ControlFlow::Continue
    /// [`Break(b)`]: ControlFlow::Break
    /// [`Some(v)`]: Some
    /// [`continue_or_else`]: OptionExt::continue_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::ops::ControlFlow;
    /// use into_controlflow::OptionExt as _;
    ///
    /// let x = Some("foo");
    /// assert_eq!(x.continue_or(0), ControlFlow::Continue("foo"));
    ///
    /// let x: Option<&str> = None;
    /// assert_eq!(x.continue_or(0), ControlFlow::Break(0));
    /// ```
    fn continue_or<B>(self, b: B) -> ControlFlow<B, Self::Item>
    where
        Self: Sized,
    {
        self.continue_or_else(|| b)
    }

    /// Transforms the [`Option<T>`] into a [`ControlFlow<B, T>`], mapping [`Some(v)`] to
    /// [`Continue(v)`] and [`None`] to the default value of `B`.
    ///
    /// [`Continue(v)`]: ControlFlow::Continue
    /// [`Some(v)`]: Some
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::ops::ControlFlow;
    /// use into_controlflow::OptionExt as _;
    ///
    /// let x = Some("foo");
    /// let y: ControlFlow<i32, _> = x.continue_or_default();
    /// assert_eq!(y, ControlFlow::Continue("foo"));
    ///
    /// let x: Option<&str> = None;
    /// assert_eq!(x.continue_or_default(), ControlFlow::Break(0));
    /// ```
    fn continue_or_default<B>(self) -> ControlFlow<B, Self::Item>
    where
        Self: Sized,
        B: Default,
    {
        self.continue_or_else(Default::default)
    }

    /// Transforms the [`Option<T>`] into a [`ControlFlow<T, C>`], mapping [`Some(v)`] to
    /// [`Break(v)`] and [`None`] to [`Continue(c)`].
    ///
    /// A non-lazy version of [`break_or_else`].
    ///
    /// [`Continue(c)`]: ControlFlow::Continue
    /// [`Break(v)`]: ControlFlow::Break
    /// [`Some(v)`]: Some
    /// [`break_or_else`]: OptionExt::break_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::ops::ControlFlow;
    /// use into_controlflow::OptionExt as _;
    ///
    /// let x = Some("foo");
    /// assert_eq!(x.break_or(0), ControlFlow::Break("foo"));
    ///
    /// let x: Option<&str> = None;
    /// assert_eq!(x.break_or(0), ControlFlow::Continue(0));
    /// ```
    fn break_or<C>(self, c: C) -> ControlFlow<Self::Item, C>
    where
        Self: Sized,
    {
        self.break_or_else(|| c)
    }

    /// Transforms the [`Option<T>`] into a [`ControlFlow<T, C>`], mapping [`Some(v)`] to
    /// [`Break(v)`] and [`None`] to the default value of `C`.
    ///
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
    /// let y: ControlFlow<_, i32> = x.break_or_default();
    /// assert_eq!(y, ControlFlow::Break("foo"));
    ///
    /// let x: Option<&str> = None;
    /// assert_eq!(x.break_or_default(), ControlFlow::Continue(0));
    /// ```
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

    fn continue_or_else<B, F>(self, b: F) -> ControlFlow<B, T>
    where
        F: FnOnce() -> B,
    {
        match self {
            Some(v) => ControlFlow::Continue(v),
            None => ControlFlow::Break(b()),
        }
    }

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
