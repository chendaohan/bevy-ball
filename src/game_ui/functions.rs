use bevy::prelude::*;

use super::{components::PopupUserName, FONT_PATH};

struct ButtonProperties<'a, 'b, 'w, 's, T: Component> {
    parent: &'a mut ChildBuilder<'b, 'w, 's>,
    asset_server: &'a AssetServer,
    border_size: f32,
    width: Val,
    height: Val,
    text: &'a str,
    font_size: f32,
    background_color: Color,
    marker: T,
}

pub fn home_title(parent: &mut ChildBuilder, asset_server: &AssetServer) {
    parent.spawn(TextBundle::from_section(
        "Ball Game",
        TextStyle {
            font: asset_server.load(FONT_PATH),
            font_size: 110.,
            color: Color::BLACK,
        },
    ));
}

pub fn home_button<T: Component>(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    text: &str,
    marker: T,
) {
    let button_properties = ButtonProperties {
        parent,
        asset_server,
        border_size: 6.,
        width: Val::Px(300.),
        height: Val::Px(100.),
        text,
        font_size: 80.,
        background_color: Color::rgb_u8(0x36, 0x5f, 0xe0),
        marker,
    };
    button(button_properties);
}

pub fn other_button<T: Component>(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    text: &str,
    background_color: Color,
    marker: T,
) {
    let button_properties = ButtonProperties {
        parent,
        asset_server,
        border_size: 4.,
        width: Val::Auto,
        height: Val::Auto,
        text,
        font_size: 50.,
        background_color,
        marker,
    };
    button(button_properties);
}

fn button<T: Component>(button_properties: ButtonProperties<T>) {
    let ButtonProperties {
        parent,
        asset_server,
        border_size,
        width,
        height,
        text,
        font_size,
        background_color,
        marker,
    } = button_properties;
    parent
        .spawn((
            ButtonBundle {
                background_color: background_color.into(),
                border_color: Color::BLACK.into(),
                style: Style {
                    width,
                    height,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(border_size)),
                    ..default()
                },
                ..default()
            },
            marker,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size,
                    color: Color::BLACK,
                },
            ));
        });
}

pub fn list_item(parent: &mut ChildBuilder, asset_server: &AssetServer, text: &str) {
    parent
        .spawn(NodeBundle {
            background_color: Color::rgb_u8(0x49, 0xaa, 0xee).into(),
            border_color: Color::BLACK.into(),
            style: Style {
                width: Val::Percent(50.),
                height: Val::Px(70.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 50.,
                    color: Color::BLACK,
                },
            ));
        });
}

pub fn scores_and_game_title<T: Component>(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    text: &str,
    marker: Option<T>,
) {
    let text_bundle = TextBundle::from_section(
        text,
        TextStyle {
            font: asset_server.load(FONT_PATH),
            font_size: 80.,
            color: Color::BLACK,
        },
    );
    if let Some(marker) = marker {
        parent.spawn((text_bundle, marker));
    } else {
        parent.spawn(text_bundle);
    }
}

pub fn popup_name(parent: &mut ChildBuilder, asset_server: &AssetServer) {
    parent.spawn(TextBundle::from_section(
        "Name:",
        TextStyle {
            font: asset_server.load(FONT_PATH),
            font_size: 50.,
            color: Color::BLACK,
        },
    ));
}

pub fn popup_input_border(parent: &mut ChildBuilder, asset_server: &AssetServer) {
    parent
        .spawn(NodeBundle {
            background_color: Color::WHITE.into(),
            border_color: Color::BLACK.into(),
            style: Style {
                width: Val::Px(350.),
                height: Val::Px(70.),
                border: UiRect::all(Val::Px(4.)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 50.,
                        color: Color::BLACK,
                    },
                ),
                PopupUserName,
            ));
        });
}

pub fn changed_state<T: States>(
    interaction: &Interaction,
    background_color: &mut BackgroundColor,
    game_state: &mut NextState<T>,
    changed_state: T,
    pressed_background: BackgroundColor,
    hovered_background: BackgroundColor,
    none_background: BackgroundColor,
) {
    match interaction {
        Interaction::Pressed => {
            *background_color = pressed_background;
            game_state.set(changed_state);
        }
        Interaction::Hovered => {
            *background_color = hovered_background;
        }
        Interaction::None => {
            *background_color = none_background;
        }
    }
}
