use iced::widget::text_input::{Appearance, StyleSheet};
use iced::{theme, Color, Theme};

#[derive(Debug, Clone)]
pub enum Input {
    Valid(String),
    Invalid(String),
}

impl Input {
    pub fn new(input: String, validation: impl Fn(&str) -> bool) -> Self {
        if validation(&input) {
            Self::Valid(input)
        } else {
            Self::Invalid(input)
        }
    }

    pub fn unwrap_valid(&self) -> &str {
        if let Self::Valid(v) = self {
            v
        } else {
            panic!("Unwrap invalid input");
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Valid(v) => v.as_str(),
            Self::Invalid(v) => v.as_str(),
        }
    }

    pub const fn is_valid(&self) -> bool {
        match self {
            Self::Valid(_) => true,
            Self::Invalid(_) => false,
        }
    }

    pub const fn style(&self) -> TextInputValidation {
        TextInputValidation(self.is_valid())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TextInputValidation(bool);

impl StyleSheet for TextInputValidation {
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

impl From<TextInputValidation> for theme::TextInput {
    fn from(v: TextInputValidation) -> Self {
        Self::Custom(Box::new(v))
    }
}
