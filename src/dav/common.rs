pub struct Namespaces<'a> {
    pub dav: &'a str,
    pub c: &'a str,
    pub cs: &'a str,
}

pub const NAMESPACES: Namespaces = Namespaces {
    dav: "DAV:",
    c: "urn:ietf:params:xml:ns:caldav",
    cs: "http://calendarserver.org/ns/",
};
