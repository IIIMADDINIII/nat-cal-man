pub struct Namespaces<'a> {
    pub dav: &'a str,
    pub cal: &'a str,
    pub cs: &'a str,
}

pub const NAMESPACES: Namespaces = Namespaces { dav: "DAV:", cal: "urn:ietf:params:xml:ns:caldav", cs: "http://calendarserver.org/ns/" };
