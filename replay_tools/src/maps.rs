use std::collections::HashMap;

use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct BoundingBox {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl BoundingBox {
    pub fn width(&self) -> u32 {
        (self.min_x as i32).abs_diff(self.max_x as i32)
    }
    
    pub fn height(&self) -> u32 {
        (self.min_y as i32).abs_diff(self.max_y as i32)
    }
}

pub fn to_2d_coordinates(b_box: &BoundingBox, x: f32, y: f32) -> (f32, f32) {
    let x_translate = b_box.width() as f32 - b_box.max_x;
    let y_translate = b_box.height() as f32 + b_box.min_y;

    (x + x_translate, (y - y_translate).abs())
}

pub static MAPS: Lazy<HashMap<&'static str, BoundingBox>> = Lazy::new(|| {
    let mut maps = HashMap::new();
    maps.insert(
        "01_karelia",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "02_malinovka",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "04_himmelsdorf",
        BoundingBox {
            min_x: -300.000000,
            min_y: -300.000000,
            max_x: 400.000000,
            max_y: 400.000000,
        },
    );
    maps.insert(
        "05_prohorovka",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "07_lakeville",
        BoundingBox {
            min_x: -400.000000,
            min_y: -400.000000,
            max_x: 400.000000,
            max_y: 400.000000,
        },
    );
    maps.insert(
        "06_ensk",
        BoundingBox {
            min_x: -300.000000,
            min_y: -300.000000,
            max_x: 300.000000,
            max_y: 300.000000,
        },
    );
    maps.insert(
        "11_murovanka",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "13_erlenberg",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "10_hills",
        BoundingBox {
            min_x: -400.000000,
            min_y: -400.000000,
            max_x: 400.000000,
            max_y: 400.000000,
        },
    );
    maps.insert(
        "18_cliff",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "19_monastery",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "28_desert",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "35_steppes",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "37_caucasus",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "33_fjord",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "34_redshire",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "36_fishing_bay",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "38_mannerheim_line",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "08_ruinberg",
        BoundingBox {
            min_x: -425.000000,
            min_y: -400.000000,
            max_x: 425.000000,
            max_y: 450.000000,
        },
    );
    maps.insert(
        "14_siegfried_line",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "23_westfeld",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "29_el_hallouf",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "31_airfield",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "17_munchen",
        BoundingBox {
            min_x: -300.000000,
            min_y: -300.000000,
            max_x: 300.000000,
            max_y: 300.000000,
        },
    );
    maps.insert(
        "44_north_america",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "45_north_america",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "47_canada_a",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "59_asia_great_wall",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "63_tundra",
        BoundingBox {
            min_x: -400.000000,
            min_y: -400.000000,
            max_x: 400.000000,
            max_y: 400.000000,
        },
    );
    maps.insert(
        "60_asia_miao",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "101_dday",
        BoundingBox {
            min_x: -400.000000,
            min_y: -500.000000,
            max_x: 600.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "114_czech",
        BoundingBox {
            min_x: -400.000000,
            min_y: -500.000000,
            max_x: 600.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "112_eiffel_tower_ctf",
        BoundingBox {
            min_x: -400.000000,
            min_y: -400.000000,
            max_x: 400.000000,
            max_y: 400.000000,
        },
    );
    maps.insert(
        "115_sweden",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "03_campania_big",
        BoundingBox {
            min_x: -400.000000,
            min_y: -400.000000,
            max_x: 450.000000,
            max_y: 450.000000,
        },
    );
    maps.insert(
        "212_epic_random_valley",
        BoundingBox {
            min_x: -700.000000,
            min_y: -700.000000,
            max_x: 700.000000,
            max_y: 700.000000,
        },
    );
    maps.insert(
        "217_er_alaska",
        BoundingBox {
            min_x: -700.000000,
            min_y: -700.000000,
            max_x: 700.000000,
            max_y: 700.000000,
        },
    );
    maps.insert(
        "99_poland",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "90_minsk",
        BoundingBox {
            min_x: -506.218903,
            min_y: -520.719910,
            max_x: 527.311340,
            max_y: 512.810303,
        },
    );
    maps.insert(
        "222_er_clime",
        BoundingBox {
            min_x: -700.000000,
            min_y: -800.000000,
            max_x: 700.000000,
            max_y: 600.000000,
        },
    );
    maps.insert(
        "250_br_battle_city2-1",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 1500.000000,
            max_y: 1500.000000,
        },
    );
    maps.insert(
        "95_lost_city_ctf",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "83_kharkiv",
        BoundingBox {
            min_x: -550.000000,
            min_y: -500.000000,
            max_x: 450.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "105_germany",
        BoundingBox {
            min_x: -525.000000,
            min_y: -550.000000,
            max_x: 525.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "208_bf_epic_normandy",
        BoundingBox {
            min_x: -1500.000000,
            min_y: -1500.000000,
            max_x: 1500.000000,
            max_y: 1500.000000,
        },
    );
    maps.insert(
        "251_br_battle_city3",
        BoundingBox {
            min_x: -1000.000000,
            min_y: -1000.000000,
            max_x: 1000.000000,
            max_y: 1000.000000,
        },
    );
    maps.insert(
        "252_br_battle_city4",
        BoundingBox {
            min_x: -1000.000000,
            min_y: -1000.000000,
            max_x: 1000.000000,
            max_y: 1000.000000,
        },
    );
    maps.insert(
        "127_japort",
        BoundingBox {
            min_x: -500.000000,
            min_y: -500.000000,
            max_x: 500.000000,
            max_y: 500.000000,
        },
    );
    maps.insert(
        "128_last_frontier_v",
        BoundingBox {
            min_x: -400.000000,
            min_y: -600.000000,
            max_x: 600.000000,
            max_y: 400.000000,
        },
    );
    maps.insert(
        "209_wg_epic_suburbia",
        BoundingBox {
            min_x: -1500.000000,
            min_y: -1500.000000,
            max_x: 1500.000000,
            max_y: 1500.000000,
        },
    );

    maps
});
