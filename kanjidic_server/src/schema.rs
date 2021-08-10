table! {
    kanji (id) {
        id -> Int4,
        literal -> Text,
        accepted_stroke_count -> Int4,
        frequency -> Nullable<Int4>,
        jlpt -> Nullable<Int4>,
    }
}
