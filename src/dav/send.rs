pub trait ToXML {
    fn element(&self) -> minidom::Element;
    fn xml(&self) -> String {
        String::from(&self.element())
    }
}

pub struct Propfind {
    prop: Prop,
}

impl Propfind {
    pub fn new(fields: PropFields) -> Self {
        Self {
            prop: Prop::new(fields),
        }
    }
}

impl ToXML for Propfind {
    fn element(&self) -> minidom::Element {
        minidom::Element::builder("propfind", super::common::NAMESPACES.dav)
            .append(self.prop.element())
            .build()
    }
}

pub struct Prop {
    fields: PropFields,
}

impl Prop {
    pub fn new(fields: PropFields) -> Self {
        Self { fields }
    }
}

impl ToXML for Prop {
    fn element(&self) -> minidom::Element {
        let mut xml = minidom::Element::builder("prop", super::common::NAMESPACES.dav);
        if self.fields.contains(PropFields::DISPLAYNAME) {
            xml = xml.append(minidom::Element::builder(
                "displayname",
                super::common::NAMESPACES.dav,
            ));
        }
        if self.fields.contains(PropFields::CAL_HOME) {
            xml = xml.append(minidom::Element::builder(
                "calendar-home-set",
                super::common::NAMESPACES.c,
            ));
        }
        if self.fields.contains(PropFields::GET_C_TAG) {
            xml = xml.append(minidom::Element::builder(
                "getctag",
                super::common::NAMESPACES.cs,
            ));
        }
        if self.fields.contains(PropFields::SYNC_TOKEN) {
            xml = xml.append(minidom::Element::builder(
                "sync-token",
                super::common::NAMESPACES.dav,
            ));
        }
        if self.fields.contains(PropFields::GET_E_TAG) {
            xml = xml.append(minidom::Element::builder(
                "getetag",
                super::common::NAMESPACES.dav,
            ));
        }
        if self.fields.contains(PropFields::CALENDAR_DATA) {
            xml = xml.append(minidom::Element::builder(
                "calendar-data",
                super::common::NAMESPACES.c,
            ));
        }
        xml.build()
    }
}

bitflags::bitflags! {
  pub struct PropFields: u8 {
    const DISPLAYNAME   = 0b00000001;
    const CAL_HOME      = 0b00000010;
    const GET_C_TAG     = 0b00010000;
    const SYNC_TOKEN    = 0b00100000;
    const GET_E_TAG     = 0b01000000;
    const CALENDAR_DATA = 0b10000000;
  }
}

pub struct CalendarQuery {
    prop: Prop,
    filter: Filter,
}

impl CalendarQuery {
    pub fn new(fields: PropFields) -> Self {
        Self {
            prop: Prop::new(fields),
            filter: Filter::new(),
        }
    }
}

impl ToXML for CalendarQuery {
    fn element(&self) -> minidom::Element {
        minidom::Element::builder("calendar-query", super::common::NAMESPACES.c)
            .append(self.prop.element())
            .append(self.filter.element())
            .build()
    }
}

pub struct Filter {}

impl Filter {
    pub fn new() -> Self {
        Self {}
    }
}

impl ToXML for Filter {
    fn element(&self) -> minidom::Element {
        minidom::Element::builder("filter", super::common::NAMESPACES.c)
            .append(
                minidom::Element::builder("comp-filter", super::common::NAMESPACES.c)
                    .attr("name", "VCALENDAR")
                    .build(),
            )
            .build()
    }
}
