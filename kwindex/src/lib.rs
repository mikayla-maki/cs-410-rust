/// Each slice in this struct's list is a word in some
/// in-memory text document.
#[derive(Debug, Default, Clone)]
pub struct KWIndex<'a>(Vec<&'a str>);

impl<'a> KWIndex<'a> {
    /// Make a new empty target words list.
    pub fn new() -> Self {
        KWIndex(Vec::default())
    }

    /// Parse the `target` text and add the sequence of
    /// valid words contained in it to this `KWIndex`
    /// index.
    ///
    /// This is a "builder method": calls can be
    /// conveniently chained to build up an index.
    ///
    /// Words are separated by whitespace or punctuation,
    /// and consist of a span of one or more consecutive
    /// letters (any UTF-8 character in the "letter" class)
    /// with no internal punctuation.
    ///
    /// For example, the text
    ///
    /// ```text
    /// "It ain't over untïl it ain't, over."
    /// ```
    ///
    /// contains the sequence of words `"It"`, `"over"`,
    /// `"untïl"`, `"it"`, `"over"`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use kwindex::KWIndex;
    /// let kwindex = KWIndex::new()
    ///     .extend_from_text("Hello world.");
    /// assert_eq!(2, kwindex.len());
    /// assert_eq!(1, kwindex.count_matches("world"));
    /// ```
    pub fn extend_from_text(mut self, target: &'a str) -> Self {
        self.0.extend(
            target
                .split_whitespace()
                .map(|word| word.trim_matches(|c: char| c.is_ascii_punctuation()))
                .filter(|word| word.chars().all(|c| c.is_alphabetic())),
        );
        self
    }

    /// Count the number of occurrences of the given `keyword`
    /// that are indexed by this `KWIndex`.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use kwindex::KWIndex;
    /// let kwindex = KWIndex::new()
    ///     .extend_from_text("b b b-banana b");
    /// assert_eq!(3, kwindex.count_matches("b"));
    /// ```
    pub fn count_matches(&self, keyword: &str) -> usize {
        self.0.iter().filter(|word| **word == keyword).count()
    }

    /// Return the *n*-th uppercase word (all characters are
    /// Unicode uppercase, *n*-th counting from 0) that is indexed
    /// by this `KWIndex`, if any.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use kwindex::KWIndex;
    /// let kwindex = KWIndex::new()
    ///     .extend_from_text("I am THE WALRUS");
    /// assert_eq!(Some("THE"), kwindex.nth_uppercase(1));
    /// ```
    pub fn nth_uppercase(&self, n: usize) -> Option<&str> {
        match self
            .0
            .iter()
            .filter(|word| word.chars().all(|c| c.is_uppercase()))
            .nth(n)
        {
            Some(n) => Some(*n),
            None => None,
        }
    }

    /// Count the number of words that are indexed by this
    /// `KWIndex`.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use kwindex::KWIndex;
    /// let kwindex = KWIndex::new()
    ///     .extend_from_text("Can't stop this!");
    /// assert_eq!(2, kwindex.len());
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Is this index empty?
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
