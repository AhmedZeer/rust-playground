use rand::prelude::*;

fn main() {
    // shadowing: this ability gives a freedom to change immutable values
    let x : i32 = random::<i32>();
    let _x : i32 = if x > 0 {x} else {-2}; // inline if else

    let x = loop {
        let tmp : i32 = random::<i32>();
        if tmp > 0 && tmp % 2 == 0 && tmp % 3 == 0 && tmp % 5 == 0
        {
            break tmp;
        }
    };
    
    println!("x divisible to 2, 3 and 5");

    println!("value of x = {}", x);

    // define struct
    struct Music
    {
        name : String,
        musician : String,
        length : u32,
        is_liked : bool,
    }

    // init new Struct instance
    let ayri_gitme = Music
    {
        name : String::from("Ayrı Gitme"),
        musician : String::from("Aleyna Tilki"),
        length : 156,
        is_liked : true
    };

    println!("Music name : {} \nMusician : {}", ayri_gitme.name, ayri_gitme.musician);
    println!("Length : {} \nLiked : {}", ayri_gitme.length, ayri_gitme.is_liked);
    
    let sen_olsan_bari = Music
    {
        name : String::from("Sen Olsan Bari"),
        length : 189,
        ..ayri_gitme // copy other attributes from this "ayri_gitme" struct instance
    };

    println!("Music name : {} \nMusician : {}", sen_olsan_bari.name, sen_olsan_bari.musician);

    // define Tuples with struct keyword
    struct Words(String, String);
    // init tuple
    let amen = Words(String::from("Amen"), String::from("Uttered at the end of a prayer or hymn, meaning ‘so be it’."));
    println!("{}: {}",amen.0 ,amen.1);

}
