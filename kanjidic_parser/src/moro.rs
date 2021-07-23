/// An entry in the dictionary Daikanwajiten.
pub struct Moro {
    /// The volume
    pub volume: Option<u8>,

    /// The page
    pub page: Option<u16>,

    /// The item number
    pub item: u16,
}