/*
enum Option<T> {
    Some(T),
    None,
}

*/

fn map() {
    let chivalrous_deeds = Some(1000);
    println!("{:?}", match chivalrous_deeds {
        Some(count) => Some(count * 2),
        None => None
    })
}

fn map2() {
    let chivalrous_deeds = Some(1000);
    println!(chivalrous_deeds.map(|count| count * 2));
}

fn unwrap_or() {
    let chivalrous_deeds = Some(1000);
    println!("{:?}", match chivalrous_deeds {
        Some(count) => count,
        None => 3,
    }
}

fn unwrap_or2() {
    let chivalrous_deeds = Some(1000);
    println!("{:?}", chivalrous_deeds.unwrap_or(3));
}

fn chained_actions() {
    let chivalrous_deeds = Some(1000);
    let deeds = chivalrous_deeds
        .map(|count| count * 2)
        .unwrap_or(3);
    println!("{:?}", deeds);
}
