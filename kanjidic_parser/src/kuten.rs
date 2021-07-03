/// A kuten representation of a JIS X 0208 or JIS X 0212 character.
/// http://unicode-iphone.blogspot.com/2010/05/kuten-code-to-unicode.html
pub struct Kuten {
    /// The Ku part of the matrix position.
    pub ku: u8,

    /// The Ten part of the matrix position.
    pub ten: u8,
}

/// A kuten representation of a JIS X 0213 character.
/// http://unicode-iphone.blogspot.com/2010/05/kuten-code-to-unicode.html
pub struct PlanarKuten {
    /// The plane on which a kuten representation is found.
    pub plane: u8,

    /// The Ku part of the matrix position.
    pub ku: u8,

    /// The Ten part of the matrix position.
    pub ten: u8,
}
