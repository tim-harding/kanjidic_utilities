pub struct PinYin<'a> {
    pub romanization: &'a str,
    pub tone: Tone,
}

pub enum Tone {
    High,
    Rising,
    Low,
    Falling,
    Neutral,
}
