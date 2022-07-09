use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    guild: Guild,
    channel: Channel,
    #[serde(alias = "dateRange")]
    date_range: DateRange,
    messages: Vec<Message>,
    #[serde(alias = "messageCount")]
    message_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Guild {
    id: String,
    name: String,
    #[serde(alias = "iconUrl")]
    icon_url: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Channel {
    id: String,
    #[serde(alias = "type")]
    type_: String,
    #[serde(alias = "categoryId")]
    category_id: String,
    category: String,
    name: String,
    topic: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DateRange {
    after: Option<String>,
    before: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    id: String,
    #[serde(alias = "type")]
    type_: String,
    timestamp: String,
    #[serde(alias = "timestampEdited")]
    timestamp_edited: Option<String>,
    #[serde(alias = "callEndedTimestamp")]
    call_ended_timestamp: Option<String>,
    #[serde(alias = "isPinned")]
    is_pinned: bool,
    content: String,
    author: Author,
    attachments: Vec<Attachment>,
    embeds: Vec<Embed>,
    stickers: Vec<Sticker>,
    reactions: Vec<Reaction>,
    mentions: Vec<Mention>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    id: String,
    name: String,
    discriminator: String,
    nickname: String,
    color: Option<String>,
    #[serde(alias = "isBot")]
    is_bot: bool,
    #[serde(alias = "avatarUrl")]
    avatar_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Mention {
    id: String,
    name: String,
    discriminator: String,
    nickname: String,
    #[serde(alias = "isBot")]
    is_bot: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Embed {}
#[derive(Serialize, Deserialize, Debug)]
struct Attachment {}
#[derive(Serialize, Deserialize, Debug)]
struct Reaction {}
#[derive(Serialize, Deserialize, Debug)]
struct Sticker {}

fn main() {
    let data = include_str!("../data/ACXD.json");
    let _v: Data = serde_json::from_str(data).unwrap();
}
