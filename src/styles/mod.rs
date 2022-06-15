use iced::container;

pub mod colors;

/**
 * Default element style sheets
 * 
 * For example, to create a default Rabl container:
 *  let container = Container::new().style(styles::Container)
 */

 pub struct Container;
 impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: colors::BACKGROUND.into(),
            text_color: iced::Color::WHITE.into(),
            .. container::Style::default()
        }
    }
}