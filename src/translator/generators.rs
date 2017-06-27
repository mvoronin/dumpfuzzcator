extern crate rand;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::iter;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use rand::distributions::Alphanumeric;


pub fn generate_int(length: usize) -> String {
    let choices = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let mut rng = thread_rng();

    (0..10).map(|_| {choices.choose(&mut rng).unwrap()}).collect()
}

pub fn generate_string(length: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(length).collect()
}

pub fn generate_male_firstname() -> String {
    let male_names = vec![
        "James", "John", "Robert", "Michael", "William", "David", "Richard", "Charles", "Joseph",
        "Thomas", "Christopher", "Daniel", "Paul", "Mark", "Donald", "George", "Kenneth", "Steven",
        "Edward", "Brian", "Ronald", "Anthony", "Kevin", "Jason", "Matthew", "Gary", "Timothy",
        "Jose", "Larry", "Jeffrey", "Frank", "Scott", "Eric", "Stephen", "Andrew", "Raymond",
        "Gregory", "Joshua", "Jerry", "Dennis", "Walter", "Patrick", "Peter", "Harold", "Douglas",
        "Henry", "Carl", "Arthur", "Ryan", "Roger", "Joe", "Juan", "Jack", "Albert", "Jonathan",
        "Justin", "Terry", "Gerald", "Keith", "Samuel", "Willie", "Ralph", "Lawrence", "Nicholas",
        "Roy", "Benjamin", "Bruce", "Brandon", "Adam", "Harry", "Fred", "Wayne", "Billy", "Steve",
        "Louis", "Jeremy", "Aaron", "Randy", "Howard", "Eugene", "Carlos"
    ];

    male_names.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn generate_female_firstname() -> String {
    let female_names = vec![
        "Mary", "Patricia", "Linda", "Barbara", "Elizabeth", "Jennifer", "Maria", "Susan", "Margaret",
        "Dorothy", "Lisa", "Nancy", "Karen", "Betty", "Helen", "Sandra", "Donna", "Carol", "Ruth",
        "Sharon", "Michelle", "Laura", "Sarah", "Kimberly", "Deborah", "Jessica", "Shirley", "Cynthia",
        "Angela", "Melissa", "Brenda", "Amy", "Anna", "Rebecca", "Virginia", "Kathleen", "Pamela",
        "Martha", "Debra", "Amanda", "Stephanie", "Carolyn", "Christine", "Marie", "Janet",
        "Catherine", "Frances", "Ann", "Joyce", "Diane", "Alice", "Julie", "Heather", "Teresa",
        "Doris", "Gloria", "Evelyn", "Jean", "Cheryl", "Mildred", "Katherine", "Joan", "Ashley",
        "Judith", "Rose", "Janice", "Kelly", "Nicole", "Judy", "Christina", "Kathy", "Theresa",
        "Beverly", "Denise", "Tammy", "Irene", "Jane", "Lori", "Rachel", "Marilyn", "Andrea"
    ];

    female_names.choose(&mut rand::thread_rng()).unwrap().to_string()
}

//pub fn generate_firstname() -> (String, &str) {
//    // TODO: randomly choose gender
//    return (generate_female_firstname(), "female")
//}

pub fn generate_lastname() -> String {
    let lastnames = vec![
        "Smith", "Johnson", "Williams", "Jones", "Brown", "Davis", "Miller", "Wilson", "Moore",
        "Taylor", "Anderson", "Thomas", "Jackson", "White", "Harris", "Martin", "Thompson",
        "Garcia", "Martinez", "Robinson", "Clark", "Rodriguez", "Lewis", "Lee", "Walker", "Hall",
        "Allen", "Young", "Hernandez", "King", "Wright", "Lopez", "Hill", "Scott", "Green", "Adams",
        "Baker", "Gonzalez", "Nelson", "Carter", "Mitchell", "Perez", "Roberts", "Turner", "Phillips",
        "Campbell", "Parker", "Evans", "Edwards", "Collins", "Stewart", "Sanchez", "Morris", "Rogers",
        "Reed", "Cook", "Morgan", "Bell", "Murphy", "Bailey", "Rivera","Cooper", "Richardson", "Cox",
        "Howard", "Ward", "Torres", "Peterson", "Gray", "Ramirez", "James", "Watson", "Brooks",
        "Kelly", "Sanders", "Price", "Bennett", "Wood", "Barnes", "Ross", "Henderson"
    ];

    lastnames[0].to_string()
}

pub fn generate_email() -> (String) {
    format!("{}@{}.{}", generate_string(10), generate_string(3), generate_string(2))
}

pub fn generate_hash(s: &str) -> String {
    let mut hasher = Md5::new();

    hasher.input_str(&s);
    hasher.result_str()
}

pub fn hyphenate(hash: String) -> String {
    let uuid = format!("{}-{}-{}-{}-{}", &hash[0..8], &hash[8..12], &hash[12..16], &hash[16..20], &hash[20..32]);

    uuid
}
