use std::io;
use rand::seq::SliceRandom;

fn main() {

    let alphabet: Vec<char> = "bcdfghjklmnpqrstvwxyz".chars().collect();
    let vowels: Vec<char> = "aeiou".chars().collect();

    println!("Hello, please insert the first letter of your company");
    let mut letter1 = String::new();
    io::stdin().read_line(&mut letter1);

    println!("Now, please insert the second letter of your company");
    let mut letter2 = String::new();
    io::stdin().read_line(&mut letter2);

     println!("Now, please tell me how long should it be the name");
     let mut number_char = String::new();
        io::stdin().read_line(&mut number_char)
            .expect("Failed to read line");
        let number_char: u32 = number_char.trim().parse().unwrap(); 
           

    for i in 1..=300 {
        //for n in 1..=number_char{
            let sample_letter: Vec<_> = alphabet
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();
            let sample_letter2: Vec<_> = alphabet
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();
            let sample_letter3: Vec<_> = alphabet
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();

            let vowels1: Vec<_> = vowels
            .choose_multiple(&mut rand::thread_rng(), 1)
            .collect();
            let vowels2: Vec<_> = vowels
            .choose_multiple(&mut rand::thread_rng(), 1)
            .collect();

            let s: String = sample_letter.into_iter().collect();
            let s1: String = sample_letter2.into_iter().collect();
            let s2: String = sample_letter3.into_iter().collect();



            let vowels_final: String = vowels1.into_iter().collect();
            let vowels_final2: String = vowels2.into_iter().collect();

            let string_final = letter1.trim().to_string() + &letter2.trim().to_string() + &s.to_string() + 
            &vowels_final.trim().to_string() + &s1.to_string() + &vowels_final2.trim().to_string() + &s2.to_string(); 

            println!("{:?}", string_final);
   // }
}
}
