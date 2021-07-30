// This module should allow for instances of the `Message` struct to be
// constructed, so that they can be serialized to JSON and attached to
// POST requests to send out Discord Webhooks.

use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,

    // Since Discord's API refused my requests, I pasted the JSON
    // serialization of a message in https://discohook.com's JSON Editor
    // to find out which fields couldn't be `null`, and should be
    // excluded instead.

    // The fields with this macro could all be replaced with the
    // `crate::alternative::Alternative` type, however that currently
    // hasn't been done as `Option` can handle it fine.  The usage of
    // main usage of`Alt` is to distinguish between a missing and a
    // `null` field when deserializing, so using it here would add no
    // benefits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

#[derive(Serialize)]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    pub color: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Field>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<Footer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Thumbnail>,
}

#[derive(Serialize)]
pub struct Field {
    // `name` and `value` are both required by the API
    pub name: String,
    pub value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}

#[derive(Serialize)]
pub struct Author {
    // `name` is required, as if it isn't included it won't be rendered
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

#[derive(Serialize)]
pub struct Footer {
    // `text` is not required, as `icon_url` can be included without it
    // if `timestamp` is specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

#[derive(Serialize)]
pub struct Image {
    pub url: String,
}

#[derive(Serialize)]
pub struct Thumbnail {
    pub url: String,
}
