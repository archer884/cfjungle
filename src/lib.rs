use serde::{Serialize, Serializer, Deserialize, Deserializer};
use std::fmt::{self, Debug};

static TEMPLATE_FORMAT_VERSION: &str = "2010-09-09";

#[derive(Debug)]
pub struct Template {
    name: String,
}

impl Serialize for Template {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[allow(non_snake_case)]
        struct InternalTemplate<'t> {
            AWSTemplateFormatVersion: &'t str,
            name: &'t str,
        }

        let serialization_template = InternalTemplate {
            AWSTemplateFormatVersion: TEMPLATE_FORMAT_VERSION,
            name: &self.name,
        };

        serialization_template.serialize(serializer)
    }
}


#[derive(Copy, Clone)]
pub enum TemplateFormatVersion {
    Version2010,
}

impl Debug for TemplateFormatVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(TEMPLATE_FORMAT_VERSION)
    }
}
