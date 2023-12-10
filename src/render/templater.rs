use handlebars::{Handlebars, RenderError};
use serde::Serialize;

pub struct Templater {
    registry: Handlebars<'static>,
}

impl Templater {
    pub fn default() -> Templater {
        let registry = Handlebars::new();
        Templater { registry }
    }

    pub fn rendere_template<T: Serialize>(
        &self,
        template_string: &str,
        data: &T,
    ) -> Result<String, RenderError> {
        self.registry.render_template(template_string, data)
    }
}
