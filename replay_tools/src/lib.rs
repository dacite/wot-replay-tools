use serde::Serialize;
use wasm_bindgen::prelude::*;
use wot_replay_parser::{BattleEvent, ReplayParser};
extern crate console_error_panic_hook;
mod maps;
mod utils;

// WARNING: Only for use in WASM without threads
static mut REPLAY_PARSER: Option<ReplayParser> = None;

pub fn set_parser(parser: Option<ReplayParser>) {
    unsafe { REPLAY_PARSER = parser };
}

pub extern "C" fn get_parser() -> Option<&'static ReplayParser> {
    unsafe { REPLAY_PARSER.as_ref() }
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn parse_replay(replay: &[u8]) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let parser = ReplayParser::parse(replay.to_vec()).unwrap();
    set_parser(Some(parser));
    
    Ok(())
}

#[wasm_bindgen]
pub fn clear() {
    set_parser(None)
}

#[derive(Serialize)]
pub struct MapInfo {
    id: String,
    height: u32,
    width: u32
}

#[wasm_bindgen]
pub fn get_map() -> Result<String, String> {
    let parser = get_parser().unwrap();

    let map_id = utils::map_id(parser).ok_or_else(|| "Cannot find map".to_owned())?;
    let bounding_box = maps::MAPS.get(map_id.as_str()).ok_or_else(|| "Cannot find map bounding box".to_owned())?;

    let map_info = MapInfo{
        id: map_id,
        height: bounding_box.height(),
        width: bounding_box.width()
    };

    Ok(serde_json::to_string(&map_info).unwrap())
}

#[derive(Serialize)]
pub struct MapPosition {
    avatar_id: i32,
    x: f32,
    y: f32
}

#[wasm_bindgen]
pub fn positions() -> Result<String, String> {
    let parser = get_parser().unwrap();
    let map_id = utils::map_id(parser).ok_or_else(|| "Cannot find map".to_owned())?;
    let bounding_box = maps::MAPS.get(map_id.as_str()).ok_or_else(|| "Cannot find map bounding box".to_owned())?;

    let positions: Vec<_> = parser
        .event_stream()
        .unwrap()
        .flatten()
        .filter_map(|event| {
            if let BattleEvent::Position(p) = event {
                let (x, y) = maps::to_2d_coordinates(&bounding_box, p.position.x, p.position.y);
                Some(MapPosition {
                    avatar_id: p.entity_id,
                    x, y
                })
            } else {
                None
            }
        })
        .collect();

    Ok(serde_json::to_string(&positions).unwrap())
}
