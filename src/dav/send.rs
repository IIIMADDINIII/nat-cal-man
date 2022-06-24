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
                super::common::NAMESPACES.cal,
            ));
        }
        if self.fields.contains(PropFields::CAL_INBOX) {
            xml = xml.append(minidom::Element::builder(
                "schedule-inbox-URL",
                super::common::NAMESPACES.cal,
            ));
        }
        if self.fields.contains(PropFields::CAL_OUTBOX) {
            xml = xml.append(minidom::Element::builder(
                "schedule-outbox-URL",
                super::common::NAMESPACES.cal,
            ));
        }
        if self.fields.contains(PropFields::GET_C_TAG) {
            xml = xml.append(minidom::Element::builder(
                "getctag",
                super::common::NAMESPACES.cs,
            ));
        }
        xml.build()
    }
}

bitflags::bitflags! {
  pub struct PropFields: u8 {
    const DISPLAYNAME = 0b00000001;
    const CAL_HOME    = 0b00000010;
    const CAL_INBOX   = 0b00000100;
    const CAL_OUTBOX  = 0b00001000;
    const GET_C_TAG   = 0b00010000;
  }
}
