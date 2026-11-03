use std::vec;


fn calc_green_and_yellow(guess: &[u8; 4], secret: &[u8; 4]) -> String
{
    let s = "⬜⬜⬜⬜";
    let mut s: Vec<char> = s.chars().collect();
    let mut matched: Vec<bool> = vec![false, false, false, false];
    for i in 0..guess.len() {
        if guess[i] == secret[i] {
            s[i] = '🟩';
            matched[i] = true;
        }
    }

    for i in 0..secret.len() {
        if guess[i] == secret[i] {
            continue;
        }
        if let Some(j) = secret.iter().position(|item | *item == guess[i])
        {
            if matched[j] == false {
                s[i] = '🟨';
                matched[j] = true;
            }
        }
    }
    

    let s: String = s.iter().collect();
    s
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn all_wrong() {
    assert_eq!(
        &calc_green_and_yellow(&[5, 6, 7, 8], &[1, 2, 3, 4]),
        "⬜⬜⬜⬜"
    );
}

#[test]
fn all_green() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 3, 4]),
        "🟩🟩🟩🟩"
    );
}

#[test]
fn one_wrong() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 5], &[1, 2, 3, 4]),
        "🟩🟩🟩⬜"
    );
}

#[test]
fn all_yellow() {
    assert_eq!(
        &calc_green_and_yellow(&[4, 3, 2, 1], &[1, 2, 3, 4]),
        "🟨🟨🟨🟨"
    );
}

#[test]
fn one_wrong_but_duplicate() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 1], &[1, 2, 3, 4]),
        "🟩🟩🟩⬜"
    );
}

#[test]
fn one_right_others_duplicate() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 1, 1, 1], &[1, 2, 3, 4]),
        "🟩⬜⬜⬜"
    );
}

#[test]
fn two_right_two_swapped() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 2, 2], &[2, 2, 2, 1]),
        "🟨🟩🟩🟨"
    );
}

#[test]
fn two_wrong_two_swapped() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 3, 3, 2], &[2, 2, 2, 1]),
        "🟨⬜⬜🟨"
    );
}

#[test]
fn a_bit_of_everything() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 9, 4, 3], &[1, 2, 3, 4]),
        "🟩⬜🟨🟨"
    );
}

#[test]
fn two_in_guess_one_in_secret() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 3], &[3, 9, 9, 9]),
        "⬜⬜🟨⬜"
    );
}

#[test]
fn two_in_secret_one_in_guess() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 4], &[3, 3, 9, 9]),
        "⬜⬜🟨⬜"
    );
}
