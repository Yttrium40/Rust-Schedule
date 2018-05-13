struct Event {
    mut name: String,
    mut time: u16,         // uses 24-hr time internally, e.g. 1230 for 12:30 p.m.
    mut location: String,
    mut description: String,
}
