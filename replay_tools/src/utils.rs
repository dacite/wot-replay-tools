use wot_replay_parser::ReplayParser;

pub fn map_id(parser: &ReplayParser) -> Option<String> {
    let map_id = parser
        .replay_json_start()
        .ok()?
        .pointer("/mapName")?
        .as_str()?
        .to_string();

    Some(map_id)
}
