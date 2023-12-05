use crate::theme::Theme;

trait View {
    fn render(theme: dyn Theme) -> String;
}
