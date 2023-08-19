use std::collections::HashMap;

use rusqlite::{Row, Connection, Result};

#[derive(Debug)]
struct RevlogEntry {
    id: i64,
    cid: i64,
    usn: i64,
    button_chosen: i64,
    interval: i64,
    last_interval: i64,
    ease_factor: i64,
    taken_millis: i64,
    review_kind: i64,
}

fn row_to_revlog_entry(row: &Row) -> Result<RevlogEntry> {
    Ok(RevlogEntry {
        id: row.get(0)?,
        cid: row.get(1)?,
        usn: row.get(2)?,
        button_chosen: row.get(3)?,
        interval: row.get(4)?,
        last_interval: row.get(5)?,
        ease_factor: row.get(6)?,
        taken_millis: row.get(7).unwrap_or_default(),
        review_kind: row.get(8).unwrap_or_default(),
    })
}

fn read_collection() -> Vec<RevlogEntry> {
    let db = Connection::open("tests/data/collection.anki21").unwrap();
    let filter_out_suspended_cards = false;
    let filter_out_flags = vec![];
    let flags_str = if !filter_out_flags.is_empty() {
        format!("AND flags NOT IN ({})", filter_out_flags.iter().map(|x: &i32| x.to_string()).collect::<Vec<String>>().join(", "))
    } else {
        "".to_string()
    };

    let suspended_cards_str = if filter_out_suspended_cards { "AND queue != -1" } else { "" };

    let query = format!(
        "SELECT * FROM revlog WHERE cid IN (
             SELECT id
             FROM cards
             WHERE queue != 0
             {}
             {}
         )",
        suspended_cards_str, flags_str
    );

    let revlogs = db.prepare_cached(&query).unwrap()
    .query_and_then([],  row_to_revlog_entry).unwrap()
    .collect::<Result<Vec<RevlogEntry>>>().unwrap();
    revlogs
}

fn group_by_cid(revlogs: Vec<RevlogEntry>) -> Vec<Vec<RevlogEntry>> {
    let mut grouped: HashMap<i64, Vec<RevlogEntry>> = HashMap::new();
    for revlog in revlogs {
        grouped.entry(revlog.cid).or_insert_with(Vec::new).push(revlog);
    }

    grouped.into_iter().map(|(_, v)| v).collect()
}

#[test]
fn test() {
    let revlogs = read_collection();
    dbg!(revlogs.len());
    let revlogs = group_by_cid(revlogs);
    dbg!(revlogs.len());
    for r in revlogs {
        dbg!(&r);
        break;
    }
}