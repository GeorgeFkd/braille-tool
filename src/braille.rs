pub mod braille {
    use core::fmt;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    impl fmt::Display for BrailleVal {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                BrailleVal::Filled => write!(f, "•"),
                BrailleVal::Unfilled => write!(f, "○"),
            }
        }
    }

    pub fn convert_to_braille(file_text: String, language: &str) -> String {
        let language_mappings =
            calculate_mappings_for_language(language).expect("Language is not supported yet");
        let mut total = "".to_owned();
        println!("Map: {:?}", language_mappings);
        for character in file_text.chars() {
            println!("Character: {character}");
            if character.eq_ignore_ascii_case(&' ') {
                continue;
            }
            let br_char = language_mappings
                .get(&character.to_uppercase().next().unwrap())
                .expect("This Character is not supported yet");
            // println!("Braille:\n {}", br_char);
            total.push_str(&br_char.to_string());
            total.push_str("\n");
            total.push_str("=====\n")
        }
        total
    }

    fn calculate_mappings_for_language(language: &str) -> Option<HashMap<char, Braille6P>> {
        if !language.eq_ignore_ascii_case("eng") {
            //read from file:
            let mut file_path = "braille-".to_owned();
            file_path.push_str(language);
            file_path.push_str(".txt");

            let file = File::open(file_path).expect("File for that language was not found");
            let mut reader = BufReader::new(file);
            let mut file_iter = reader.lines();
            let mut mappings_of_chars_of_language_to_braille: HashMap<char, Braille6P> =
                HashMap::new();
            loop {
                let letter = file_iter.next();
                if let Some(Err(err_val)) = letter {
                    break;
                } else if let Some(Ok(letter)) = letter {
                    let first_row = file_iter.next().unwrap().expect("first row not placed");
                    let second_row = file_iter.next().unwrap().expect("second row not placed");
                    let third_row = file_iter.next().unwrap().expect("3rd row not placed");

                    let p1 = first_row.split_ascii_whitespace().nth(0);
                    let p2 = first_row.split_ascii_whitespace().nth_back(0);
                    let p3 = second_row.split_ascii_whitespace().nth(0);
                    let p4 = second_row.split_ascii_whitespace().nth_back(0);
                    let p5 = third_row.split_ascii_whitespace().nth(0);
                    let p6 = third_row.split_ascii_whitespace().nth_back(0);
                    let vals = vec![p1, p2, p3, p4, p5, p6];
                    let vals: Vec<BrailleVal> = vals
                        .into_iter()
                        .map(|x| x.unwrap())
                        .map(|c| {
                            if c.eq("-") {
                                return BrailleVal::Unfilled;
                            }
                            BrailleVal::Filled
                        })
                        .collect();

                    let braille_of_letter = Braille6P {
                        p1: *vals.get(0).unwrap(),
                        p2: *vals.get(1).unwrap(),
                        p3: *vals.get(2).unwrap(),
                        p4: *vals.get(3).unwrap(),
                        p5: *vals.get(4).unwrap(),
                        p6: *vals.get(5).unwrap(),
                    };

                    let letter = letter.chars().next().unwrap();
                    println!("Letter: {}", letter);
                    mappings_of_chars_of_language_to_braille.insert(letter, braille_of_letter);
                } else {
                    break;
                }
            }
            return Some(mappings_of_chars_of_language_to_braille);
        }

        let mut mappings_of_chars_of_language_to_braille = HashMap::new();
        mappings_of_chars_of_language_to_braille.insert(
            ',',
            Braille6P {
                p5: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'A',
            Braille6P {
                p1: BrailleVal::Filled,
                ..Default::default()
            },
        );

        mappings_of_chars_of_language_to_braille.insert(
            'B',
            Braille6P {
                p1: BrailleVal::Filled,
                p3: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'C',
            Braille6P {
                p1: BrailleVal::Filled,
                p2: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'D',
            Braille6P {
                p1: BrailleVal::Filled,
                p2: BrailleVal::Filled,
                p4: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'E',
            Braille6P {
                p1: BrailleVal::Filled,
                p4: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'F',
            Braille6P {
                p1: BrailleVal::Filled,
                p2: BrailleVal::Filled,
                p3: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'G',
            Braille6P {
                p1: BrailleVal::Filled,
                p2: BrailleVal::Filled,
                p3: BrailleVal::Filled,
                p4: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'H',
            Braille6P {
                p1: BrailleVal::Filled,
                p3: BrailleVal::Filled,
                p4: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'I',
            Braille6P {
                p2: BrailleVal::Filled,
                p4: BrailleVal::Filled,

                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'J',
            Braille6P {
                p2: BrailleVal::Filled,
                p3: BrailleVal::Filled,
                p4: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'K',
            Braille6P {
                p1: BrailleVal::Filled,
                p5: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'L',
            Braille6P {
                p1: BrailleVal::Filled,
                p3: BrailleVal::Filled,
                p5: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'M',
            Braille6P {
                p1: BrailleVal::Filled,
                p2: BrailleVal::Filled,
                p5: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'N',
            Braille6P {
                p1: BrailleVal::Filled,
                p2: BrailleVal::Filled,
                p4: BrailleVal::Filled,
                p5: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'O',
            Braille6P {
                p1: BrailleVal::Filled,
                p4: BrailleVal::Filled,
                p5: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'P',
            Braille6P {
                p1: BrailleVal::Filled,
                p2: BrailleVal::Filled,
                p3: BrailleVal::Filled,
                p5: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'Q',
            Braille6P {
                p1: BrailleVal::Filled,
                p2: BrailleVal::Filled,
                p3: BrailleVal::Filled,
                p4: BrailleVal::Filled,
                p5: BrailleVal::Filled,
                ..Default::default()
            },
        );
        mappings_of_chars_of_language_to_braille.insert(
            'R',
            Braille6P {
                p1: BrailleVal::Filled,
                p3: BrailleVal::Filled,
                p4: BrailleVal::Filled,
                p5: BrailleVal::Filled,
                ..Default::default()
            },
        );

        mappings_of_chars_of_language_to_braille.insert(
            'S',
            Braille6P {
                p3: BrailleVal::Filled,
                p2: BrailleVal::Filled,
                p5: BrailleVal::Filled,
                ..Default::default()
            },
        );

        mappings_of_chars_of_language_to_braille.insert(
            'T',
            Braille6P {
                p2: BrailleVal::Filled,
                p3: BrailleVal::Filled,
                p4: BrailleVal::Filled,
                p5: BrailleVal::Filled,
                ..Default::default()
            },
        );

        mappings_of_chars_of_language_to_braille.insert(
            'U',
            Braille6P {
                p1: BrailleVal::Filled,
                p5: BrailleVal::Filled,
                p6: BrailleVal::Filled,
                ..Default::default()
            },
        );

        mappings_of_chars_of_language_to_braille.insert(
            'V',
            Braille6P {
                p1: BrailleVal::Filled,
                p3: BrailleVal::Filled,
                p5: BrailleVal::Filled,
                p6: BrailleVal::Filled,
                ..Default::default()
            },
        );

        mappings_of_chars_of_language_to_braille.insert(
            'W',
            Braille6P {
                p2: BrailleVal::Filled,
                p3: BrailleVal::Filled,
                p4: BrailleVal::Filled,
                p6: BrailleVal::Filled,
                ..Default::default()
            },
        );

        mappings_of_chars_of_language_to_braille.insert(
            'X',
            Braille6P {
                p1: BrailleVal::Filled,
                p2: BrailleVal::Filled,
                p5: BrailleVal::Filled,
                p6: BrailleVal::Filled,
                ..Default::default()
            },
        );

        mappings_of_chars_of_language_to_braille.insert(
            'Y',
            Braille6P {
                p1: BrailleVal::Filled,
                p2: BrailleVal::Filled,
                p4: BrailleVal::Filled,
                p5: BrailleVal::Filled,
                p6: BrailleVal::Filled,
                ..Default::default()
            },
        );

        mappings_of_chars_of_language_to_braille.insert(
            'Z',
            Braille6P {
                p1: BrailleVal::Filled,
                p3: BrailleVal::Filled,
                p5: BrailleVal::Filled,
                p6: BrailleVal::Filled,
                ..Default::default()
            },
        );
        Some(mappings_of_chars_of_language_to_braille)
    }
    #[derive(Default, Copy, Clone, Debug)]
    pub enum BrailleVal {
        Filled,
        #[default]
        Unfilled,
    }

    impl fmt::Display for Braille6P {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{} {}\n___\n {} {}\n___\n {} {}",
                self.p1, self.p2, self.p3, self.p4, self.p5, self.p6
            )
        }
    }

    #[derive(Default, Copy, Clone, Debug)]
    pub struct Braille6P {
        p1: BrailleVal,
        p2: BrailleVal,
        p3: BrailleVal,
        p4: BrailleVal,
        p5: BrailleVal,
        p6: BrailleVal,
    }

    // let mut mappingLettersToBraille = HashMap::new();
}
