use counter::Counter;
use itertools::Itertools;
use rand::{prelude::*, seq::SliceRandom};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::ops::Range;
use tabular::{Row, Table};

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(2);
    let data = include_str!("../data/ACXD.json");
    let v: Data = serde_json::from_str(data).unwrap();
    //stats(v);
    println!("{}", format_message(&v.messages[0]));
    println!("{}", format_some_messages(&v.messages, &mut rng));
}

fn format_some_messages(messages: &[Message], rng: &mut ChaCha8Rng) -> String {
    get_intervals(messages.len(), 4, 16, rng)
        .into_iter()
        .map(|r| format_messages(&messages[r]))
        .join("\n\n")
}

fn get_intervals(
    total_messages: usize,
    num_intervals: usize,
    interval_size: usize,
    rng: &mut ChaCha8Rng,
) -> Vec<Range<usize>> {
    fn merge_overlapping_ranges<'a, I>(mut ranges: I) -> Vec<Range<usize>>
    where
        I: Iterator<Item = Range<usize>>,
    {
        let mut merged_ranges: Vec<Range<usize>> = vec![];
        let mut next = ranges.next().clone().unwrap();
        for range in ranges {
            if range.start <= next.end {
                next.end = std::cmp::max(next.end, range.end);
            } else {
                merged_ranges.push(next);
                next = range;
            }
        }
        merged_ranges
    }
    let start_positions = (0..total_messages)
        .choose_multiple(rng, num_intervals)
        .into_iter()
        .sorted();
    merge_overlapping_ranges(
        start_positions
            .map(|start| start.clone()..std::cmp::min(start + interval_size, total_messages - 1)),
    )
}

fn format_messages(ms: &[Message]) -> String {
    ms.iter().map(|m| format_message(m)).join("")
}
fn format_message(m: &Message) -> String {
    format!(
        "!@[{}]: {}\n",
        m.author.name,
        m.content
            .split("\n")
            .map(|m| m.trim())
            .collect::<Vec<_>>()
            .join("\n| ")
    )
}

fn stats(v: &Data, rng: &mut ChaCha8Rng) {
    let samples = 10000.0;
    let avg_length = v
        .messages
        .choose_multiple(rng, samples as usize)
        .map(|m| m.content.len())
        .sum::<usize>() as f64
        / samples;
    println!("average message length: {:?}", avg_length);

    let message_counter: Counter<String, usize> = v
        .messages
        .iter()
        .map(|m| m.author.name.clone())
        .collect::<Counter<_>>();
    let table_size = 10;
    let mut message_table = Table::new("{:>}  {:<}");
    for (user, messages) in message_counter
        .most_common_ordered()
        .iter()
        .take(table_size)
    {
        message_table.add_row(Row::new().with_cell(user).with_cell(messages));
    }
    println!("Most messages: \n{}", message_table);

    let mut character_counter: Counter<String, usize> = Counter::new();
    for message in v.messages.iter() {
        character_counter[&message.author.name] += message.content.len()
    }
    let mut character_table = Table::new("{:>}  {:<}");
    for (user, messages) in character_counter
        .most_common_ordered()
        .iter()
        .take(table_size)
    {
        character_table.add_row(Row::new().with_cell(user).with_cell(messages));
    }
    println!("Most characters: \n{}", character_table);

    let mut react_counter: Counter<String, usize> = Counter::new();
    for message in v.messages.iter() {
        react_counter[&message.author.name] += message.reactions.len()
    }
    let mut react_table = Table::new("{:>}  {:<}");
    for (user, messages) in react_counter.most_common_ordered().iter().take(table_size) {
        react_table.add_row(Row::new().with_cell(user).with_cell(messages));
    }
    println!("Most reacts: \n{}", react_table);
}

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
