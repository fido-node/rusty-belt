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

#[cfg(test)]
mod tests {
    use crate::render::{templater::Templater, view::representation::sample_data::*};

    #[test]
    fn test_render_session_template() {
        let templater = Templater::default();
        let session = sample_session();
        let template = "session_name: \": {{v}}\"";
        let result = templater.rendere_template(template, &session).unwrap();
        assert_eq!(result, "session_name: \": sample_session\"");
    }

    #[test]
    fn test_render_vpn_template() {
        let templater = Templater::default();
        let vpn = sample_vpn();
        let template = "\"vpn: {{#if v}}󰖂: {{#each v}}{{this}}{{#unless @last}}, {{/unless}}{{/each}}{{else}}No VPNs connected.{{/if}}\"";
        let result = templater.rendere_template(template, &vpn).unwrap();
        assert_eq!(result, "\"vpn: 󰖂: vpn1, vpn2\"");
    }

    #[test]
    fn test_render_mem_template() {
        let templater = Templater::default();
        let mem = sample_mem();
        let template = "\"mem: : {{v.total}}/{{v.available}}/{{v.used}}\"";
        let result = templater.rendere_template(template, &mem).unwrap();
        assert_eq!(result, "\"mem: : 16 GB/8 GB/8 GB\"");
    }

    #[test]
    fn test_render_cpu_template() {
        let templater = Templater::default();
        let cpu = sample_cpu();
        let template = "\"cpu: : {{v.consumption}}%\"";
        let result = templater.rendere_template(template, &cpu).unwrap();
        assert_eq!(result, "\"cpu: : 75%\"");
    }

    #[test]
    fn test_render_load_average_template() {
        let templater = Templater::default();
        let load_average = sample_la();
        let template = "\"load_average: LA: {{v.one}}, {{v.five}}, {{v.fifteen}}\"";
        let result = templater.rendere_template(template, &load_average).unwrap();
        assert_eq!(result, "\"load_average: LA: 1.25, 2.50, 3.75\"");
    }

    #[test]
    fn test_render_swap_template() {
        let templater = Templater::default();
        let swap = sample_swap();
        let template = "\"swap: Swap: {{v.total}}/{{v.used}}\"";
        let result = templater.rendere_template(template, &swap).unwrap();
        assert_eq!(result, "\"swap: Swap: 4 GB/1 GB\"");
    }

    #[test]
    fn test_render_disk_template() {
        let templater = Templater::default();
        let disk = sample_disk();
        let template = "\"disk:  {{v.mount_point}} {{v.used_percents}}%\"";
        let result = templater.rendere_template(template, &disk).unwrap();
        assert_eq!(result, "\"disk:  /mnt/data 50%\"");
    }

    #[test]
    fn test_render_shell_template() {
        let templater = Templater::default();
        let shell = sample_shell();
        let template = "\"shell: {{v.stdout}}\"";
        let result = templater.rendere_template(template, &shell).unwrap();
        assert_eq!(result, "\"shell: Sample shell output.\"");
    }
}
