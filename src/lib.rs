pub mod strokes;
use crate::strokes::Strokes;
use strokes::{CharElement, KanjiApi};

#[derive(Debug)]
pub struct Gokaku {
    tenkaku: u32,
    gaikaku: u32,
    jinkaku: u32,
    dikaku: u32,
    soukaku: u32,
}

impl Gokaku {
    fn new(tenkaku: u32, gaikaku: u32, jinkaku: u32, dikaku: u32, soukaku: u32) -> Gokaku {
        Gokaku {
            tenkaku,
            gaikaku,
            jinkaku,
            dikaku,
            soukaku,
        }
    }
}

pub fn get_all_strokes(last_name: &str, first_name: &str) -> (Vec<CharElement>, Vec<CharElement>) {
    let last_name_length = last_name.chars().count();
    let first_name_length = first_name.chars().count();

    let mut last_name_items = Vec::new();
    let mut first_name_items = Vec::new();

    let api = KanjiApi::new();

    if first_name_length > last_name_length {
        for _ in 0..first_name_length - last_name_length {
            last_name_items.push(CharElement::new_dummy());
        }
    }
    for c in last_name.chars() {
        let count = api.get_strokes(c);
        last_name_items.push(CharElement::new(c, count));
    }
    for c in first_name.chars() {
        let count = api.get_strokes(c);
        first_name_items.push(CharElement::new(c, count));
    }
    if last_name_length > first_name_length {
        for _ in 0..last_name_length - first_name_length {
            first_name_items.push(CharElement::new_dummy());
        }
    }

    (last_name_items, first_name_items)
}

pub fn calc_jikaku(last_name_items: &[CharElement], first_name_items: &[CharElement]) -> Gokaku {
    let tenkaku: u32 = last_name_items.iter().map(|x| x.strokes).sum();
    let dikaku: u32 = first_name_items.iter().map(|x| x.strokes).sum();
    let jinkaku: u32 =
        last_name_items.iter().last().unwrap().strokes + first_name_items.get(0).unwrap().strokes;
    let gaikaku = if (tenkaku + dikaku - jinkaku) > 0 {
        tenkaku + dikaku - jinkaku
    } else {
        jinkaku
    };
    let soukaku: u32 = last_name_items
        .iter()
        .filter(|x| x.character.is_some())
        .map(|x| x.strokes)
        .sum::<u32>()
        + first_name_items
            .iter()
            .filter(|x| x.character.is_some())
            .map(|x| x.strokes)
            .sum::<u32>();
    Gokaku::new(tenkaku, gaikaku, jinkaku, dikaku, soukaku)
}

pub fn print_name_info(last_name_items: &[CharElement], first_name_items: &[CharElement]) {
    for elem in last_name_items {
        if let Some(c) = elem.character {
            println!("{} -> {}", c, elem.strokes);
        }
    }
    for elem in first_name_items {
        if let Some(c) = elem.character {
            println!("{} -> {}", c, elem.strokes);
        }
    }
}

pub fn print_gokaku(x: &Gokaku) {
    println!("天格: {}", x.tenkaku);
    println!("外格: {}", x.gaikaku);
    println!("人格: {}", x.jinkaku);
    println!("地格: {}", x.dikaku);
    println!("総格: {}", x.soukaku);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_gokaku_basic_00001() {
        let last_name_items: [CharElement; 2] =
            [CharElement::new('山', 3), CharElement::new('田', 5)];
        let first_name_items: [CharElement; 2] =
            [CharElement::new('花', 7), CharElement::new('子', 3)];
        let result = calc_jikaku(&last_name_items, &first_name_items);

        assert_eq!(result.tenkaku, 8u32);
        assert_eq!(result.gaikaku, 6u32);
        assert_eq!(result.jinkaku, 12u32);
        assert_eq!(result.dikaku, 10u32);
        assert_eq!(result.soukaku, 18u32);
    }
}
