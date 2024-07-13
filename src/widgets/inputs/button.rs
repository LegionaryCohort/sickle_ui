use bevy::{prelude::*, ui::FocusPolicy};

use sickle_ui_scaffold::prelude::*;

use crate::widgets::layout::{
    container::UiContainerExt,
    label::{LabelConfig, UiLabelExt},
};

// TODO clean up this code and add the button click event stuff from my project in here

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ComponentThemePlugin::<Button>::default())
            .add_systems(Update, (toggle_checkbox, update_checkbox).chain());
    }
}

fn toggle_checkbox(
    mut q_checkboxes: Query<(&mut Button, &FluxInteraction), Changed<FluxInteraction>>,
) {
    for (mut checkbox, interaction) in &mut q_checkboxes {
        if *interaction == FluxInteraction::Released {
            checkbox.checked = !checkbox.checked;
        }
    }
}

fn update_checkbox(
    q_checkboxes: Query<(Entity, &Button), Changed<Button>>,
    mut commands: Commands,
) {
    for (entity, checkbox) in &q_checkboxes {
        commands
            .style_unchecked(checkbox.checkmark)
            .visibility(match checkbox.checked {
                true => Visibility::Inherited,
                false => Visibility::Hidden,
            });

        match checkbox.checked {
            true => commands
                .entity(entity)
                .add_pseudo_state(PseudoState::Checked),
            false => commands
                .entity(entity)
                .remove_pseudo_state(PseudoState::Checked),
        };
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Button {
    pub checked: bool,
    checkmark_background: Entity,
    checkmark: Entity,
    label: Entity,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            checked: false,
            checkmark_background: Entity::PLACEHOLDER,
            checkmark: Entity::PLACEHOLDER,
            label: Entity::PLACEHOLDER,
        }
    }
}

impl UiContext for Button {
    fn get(&self, target: &str) -> Result<Entity, String> {
        match target {
            Button::CHECKMARK_BACKGROUND => Ok(self.checkmark_background),
            Button::CHECKMARK => Ok(self.checkmark),
            Button::LABEL => Ok(self.label),
            _ => Err(format!(
                "{} doesn't exists for Checkbox. Possible contexts: {:?}",
                target,
                self.contexts()
            )),
        }
    }

    fn contexts(&self) -> Vec<&'static str> {
        vec![
            Button::CHECKMARK_BACKGROUND,
            Button::CHECKMARK,
            Button::LABEL,
        ]
    }
}

impl DefaultTheme for Button {
    fn default_theme() -> Option<Theme<Button>> {
        Button::theme().into()
    }
}

impl Button {
    pub const CHECKMARK_BACKGROUND: &'static str = "CheckmarkBackground";
    pub const CHECKMARK: &'static str = "Checkmark";
    pub const LABEL: &'static str = "Label";

    pub fn theme() -> Theme<Button> {
        let base_theme = PseudoTheme::deferred(None, Button::primary_style);
        let checked_theme =
            PseudoTheme::deferred(vec![PseudoState::Checked], Button::checked_style);
        Theme::new(vec![base_theme, checked_theme])
    }

    fn primary_style(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let theme_spacing = theme_data.spacing;
        let colors = theme_data.colors();

        style_builder
            .height(Val::Px(theme_spacing.inputs.checkbox.line_height))
            .justify_content(JustifyContent::Start)
            .align_items(AlignItems::Center)
            .margin(UiRect::horizontal(Val::Px(theme_spacing.gaps.small)))
            .background_color(Color::NONE)
            .border_radius(BorderRadius::all(Val::Px(
                theme_spacing.corners.extra_small,
            )));

        style_builder
            .switch_target(Button::CHECKMARK_BACKGROUND)
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center)
            .align_content(AlignContent::Center)
            .size(Val::Px(theme_spacing.inputs.checkbox.checkbox_size()))
            .margin(UiRect::all(Val::Px(theme_spacing.gaps.small)))
            .border(UiRect::all(Val::Px(
                theme_spacing.inputs.checkbox.border_size,
            )))
            .border_radius(BorderRadius::all(Val::Px(
                theme_spacing.corners.extra_small,
            )))
            .animated()
            .border_color(AnimatedVals {
                idle: colors.on(On::SurfaceVariant),
                hover: colors.on(On::Surface).into(),
                ..default()
            })
            .copy_from(theme_data.interaction_animation);

        style_builder
            .switch_target(Button::CHECKMARK_BACKGROUND)
            .animated()
            .background_color(AnimatedVals {
                idle: Color::NONE,
                hover: colors.accent(Accent::Primary).into(),
                ..default()
            })
            .copy_from(theme_data.interaction_animation);

        style_builder
            .switch_target(Button::CHECKMARK)
            .size(Val::Px(theme_spacing.inputs.checkbox.checkmark_size))
            .icon(theme_data.icons.checkmark.with(
                colors.on(On::Primary),
                theme_spacing.inputs.checkbox.checkmark_size,
            ));

        let font = theme_data
            .text
            .get(FontStyle::Body, FontScale::Medium, FontType::Regular);
        style_builder
            .switch_target(Button::LABEL)
            .margin(UiRect::px(
                theme_spacing.gaps.small,
                theme_spacing.gaps.medium,
                0.,
                0.,
            ))
            .sized_font(font)
            .animated()
            .font_color(AnimatedVals {
                idle: colors.on(On::SurfaceVariant),
                hover: colors.on(On::Surface).into(),
                ..default()
            })
            .copy_from(theme_data.interaction_animation);
    }

    fn checked_style(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let theme_spacing = theme_data.spacing;
        let colors = theme_data.colors();

        style_builder
            .switch_target(Button::CHECKMARK_BACKGROUND)
            .animated()
            .border(AnimatedVals {
                idle: UiRect::all(Val::Px(0.)),
                hover: UiRect::all(Val::Px(theme_spacing.inputs.checkbox.border_size)).into(),
                ..default()
            })
            .copy_from(theme_data.interaction_animation);

        style_builder
            .switch_target(Button::CHECKMARK_BACKGROUND)
            .animated()
            .background_color(AnimatedVals {
                idle: colors.accent(Accent::Primary),
                enter_from: Some(Color::NONE),
                ..default()
            })
            .copy_from(theme_data.enter_animation);

        style_builder
            .switch_target(Button::CHECKMARK)
            .animated()
            .margin(AnimatedVals {
                idle: UiRect::all(Val::Px(theme_spacing.inputs.checkbox.border_size)),
                hover: UiRect::all(Val::Px(0.)).into(),
                ..default()
            })
            .copy_from(theme_data.interaction_animation);

        style_builder
            .switch_target(Button::CHECKMARK)
            .animated()
            .scale(AnimatedVals {
                idle: 1.,
                enter_from: Some(0.),
                ..default()
            })
            .copy_from(theme_data.enter_animation);
    }

    fn checkbox_container(name: String) -> impl Bundle {
        (
            Name::new(name),
            ButtonBundle::default(),
            TrackedInteraction::default(),
        )
    }

    fn checkmark_background() -> impl Bundle {
        (
            Name::new("Checkmark Background"),
            NodeBundle {
                focus_policy: FocusPolicy::Pass,
                ..default()
            },
            LockedStyleAttributes::lock(LockableStyleAttribute::FocusPolicy),
        )
    }

    fn checkmark() -> impl Bundle {
        (
            Name::new("Checkmark"),
            ImageBundle {
                focus_policy: FocusPolicy::Pass,
                ..default()
            },
            BorderColor::default(),
            LockedStyleAttributes::from_vec(vec![
                LockableStyleAttribute::FocusPolicy,
                LockableStyleAttribute::Visibility,
            ]),
        )
    }
}

pub trait UiButtonExt {
    fn button<E: Event>(&mut self, label: impl Into<Option<String>>) -> UiBuilder<Entity>;
}

impl UiButtonExt for UiBuilder<'_, Entity> {
    /// A simple checkbox with an optional label.
    ///
    /// ### PseudoState usage
    /// - `PseudoState::Checked`, when the checkbox is in a checked state
    fn button<E>(&mut self, label: impl Into<Option<String>>) -> UiBuilder<Entity> {
        let mut button = Button { ..default() };
        let label = label.into().unwrap_or_default();
        let has_label = !label.is_empty();
        let name = match has_label {
            true => format!("Button [{}]", label.clone()),
            false => "Button".to_owned(),
        };

        let mut input = self.container(Button::checkbox_container(name), |container| {
            button.checkmark_background = container
                .container(Button::checkmark_background(), |checkmark_bg| {
                    button.checkmark = checkmark_bg.spawn(Button::checkmark()).id();
                })
                .id();

            button.label = container
                .label(LabelConfig { label, ..default() })
                .style()
                .render(has_label)
                .id();
        });

        input.insert(button);

        input
    }
}
