use iced::widget::text_input::{Appearance, StyleSheet};
use iced::{Color, Theme};

#[derive(Debug, Clone, Copy)]
pub struct TextInputValidateion(pub bool);

impl StyleSheet for TextInputValidateion {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        let palette = style.extended_palette();

        Appearance {
            background: if self.0 {
                palette.background.base
            } else {
                palette.danger.base
            }
            .color
            .into(),
            border_radius: 2.,
            border_width: 1.,
            border_color: palette.background.strong.color,
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        let palette = style.extended_palette();

        Appearance {
            background: if self.0 {
                palette.background.base
            } else {
                palette.danger.base
            }
            .color
            .into(),
            border_radius: 2.,
            border_width: 1.,
            border_color: palette.background.base.text,
        }
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        let palette = style.extended_palette();

        Appearance {
            background: if self.0 {
                palette.background.base
            } else {
                palette.danger.base
            }
            .color
            .into(),
            border_radius: 2.,
            border_width: 1.,
            border_color: palette.primary.strong.color,
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        style.extended_palette().background.strong.color
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        style.extended_palette().background.base.text
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        style.extended_palette().primary.weak.color
    }
}
