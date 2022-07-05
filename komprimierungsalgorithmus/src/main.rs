use std::env;

fn main() {
    // let Some((first, elements)) = x.split_first()
    let to_compress: Vec<String> = env::args().collect();
    let mut compressed: Vec<String> = vec![];
    let mut akku: u64 = 1;
    let mut last_item: String = "%".to_string();

    for i in 1..to_compress.len() {
        let item: String = to_compress[i].clone();
        if *item == last_item {
            akku += 1;
        } else {
            // umschaltdingsi oder sind wir am ende?
            // akku angugn
            //wenn akku größer as 4 passieren dinge
            if akku > 4 {
                compressed.push("%".to_string());
                compressed.push(akku.to_string());
                compressed.push(last_item.clone());
            } else {
                if last_item != "%" {
                    for _j in 0..akku {
                        compressed.push(last_item.clone());
                    }
                }
            }
            akku = 1;
        }
        last_item = item.clone();
    }
    // schluss abfangen

    if akku > 4 {
        compressed.push("%".to_string());
        compressed.push(akku.to_string());
        compressed.push(last_item.clone());
    } else {
        if last_item != "%" {
            for _j in 0..akku {
                compressed.push(last_item.clone());
            }
        }
    }
    println!("{:?}", compressed);
}
