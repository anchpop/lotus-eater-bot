use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    guild: Guild,
    channel: Channel,
    dateRange: DateRange,
    messages: Vec<Message>,
    messageCount: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Guild {
    id: String,
    name: String,
    iconUrl: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Channel {
    id: String,
    #[serde(alias = "type")]
    type_: String,
    categoryId: String,
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
    timestampEdited: Option<String>,
    callEndedTimestamp: Option<String>,
    isPinned: bool,
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
    isBot: bool,
    avatarUrl: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Mention {
    id: String,
    name: String,
    discriminator: String,
    nickname: String,
    isBot: bool,
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
    let v: Data = serde_json::from_str(data).unwrap();
}
