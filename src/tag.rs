pub trait Tagged {
    fn get_tag_names(&self) -> &'static [&'static str];

    fn includes_tag_name(&self, tag: &str) -> bool {
        self.get_tag_names().contains(&tag)
    }

    /// Returns all the instances of a tag of type ``T``.
    /// Best used for tags that are not unqiue (such as choice requirements).
    fn get_tags<T: Tag>(&self) -> Vec<T> {
        self.get_tag_names()
            .into_iter()
            .filter_map(|name| T::from(name))
            .collect()
    }

    /// Returns the first instance of a tag of type ``T``, if it exists.
    /// Best used for tags that are unique (such as a speaker name).
    fn get_tag<T: Tag>(&self) -> Option<T> {
        self.get_tag_names()
            .into_iter()
            .filter_map(|name| T::from(name))
            .next()
    }

    fn includes_tag<T: Tag + PartialEq>(&self, tag: &T) -> bool {
        self.get_tag_names()
            .into_iter()
            .filter_map(|name| T::from(name))
            .filter(|t| t == tag)
            .count()
            > 0
    }
}

pub trait Tag: Sized {
    /// Convert a tag name (alphanumeric and underscore characters) to a tag of this type.
    fn from(string: &str) -> Option<Self>;
}
