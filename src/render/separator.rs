pub fn palet_iterator(data: &Vec<String>, reverse: bool) -> Box<dyn Iterator<Item = &String> + '_> {
    let iter = data.into_iter();

    if reverse {
        Box::new(iter.rev().cycle())
    } else {
        Box::new(iter.cycle())
    }
}

pub struct Separator<'a> {
    reverse: bool,
    // fg_colors: Box<dyn Iterator<Item = &'a String> + 'a>,
    bg_colors: Box<dyn Iterator<Item = &'a String> + 'a>,
    // prev_fg_color: String,
    prev_bg_color: String,
}

impl Separator<'_> {
    pub fn new<'a>(
        reverse: bool,
        // fg_colors: &Vec<String>,
        bg_colors: &'a Vec<String>,
    ) -> Separator<'a> {
        let mut palet_iterator = palet_iterator(bg_colors, reverse);
        let first_color = if reverse {
            palet_iterator
                .next()
                .map(|s| format!("#{}", s))
                .unwrap_or("default".to_string())
        } else {
            palet_iterator
                .next()
                .map(|s| format!("#{}", s))
                .unwrap_or("default".to_string())
        };
        Separator {
            reverse,
            // fg_colors: palet_iterator(fg_colors, reverse),
            bg_colors: palet_iterator,
            // prev_fg_color: "default".to_string(),
            prev_bg_color: first_color,
        }
    }

    //fg, bg
    pub fn get_color_pair(&mut self, is_edge: bool) -> (String, String) {
        let default_color = "default".to_string();
        if self.reverse {
            let result = if is_edge {
                (self.prev_bg_color.clone(), default_color.clone())
            } else {
                let color = self
                    .bg_colors
                    .next()
                    .map(|s| format!("#{}", s))
                    .unwrap_or(default_color.clone());

                let r = (color.clone(), self.prev_bg_color.clone());
                self.prev_bg_color = color.clone();
                r
            };
            result
        } else {
            let color = self
                .bg_colors
                .next()
                .map(|s| format!("#{}", s))
                .unwrap_or(default_color.clone());

            let result = if is_edge {
                (self.prev_bg_color.clone(), default_color.clone())
            } else {
                (self.prev_bg_color.clone(), color.clone())
            };
            self.prev_bg_color = color.clone();
            result
        }
    }
}
