/// Function which prints the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
///
/// # Call the function
///
/// ```
/// use p42::song::print_twelve_days_of_christmas;
/// print_twelve_days_of_christmas();
/// ```

struct SongIter {
    current_day: usize,
    days: Vec<&'static str>,
    gifts: Vec<&'static str>,
    state: State,
}

enum State {
    DayIntro,
    Gifts(usize),
}

impl SongIter {
    fn new() -> Self {
        SongIter {
            current_day: 0,
            days: vec![
                "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth",
                "ninth", "tenth", "eleventh", "twelfth",
            ],
            gifts: vec![
                "a Partridge in a Pear Tree",
                "two Turtle Doves",
                "three French Hens",
                "four Calling Birds",
                "five Gold Rings",
                "six Geese a Laying",
                "seven Swans a Swimming",
                "eight Maids a Milking",
                "nine Ladies Dancing",
                "ten Lords a Leaping",
                "eleven Pipers Piping",
                "twelve Drummers Drumming",
            ],
            state: State::DayIntro,
        }
    }
}

impl Iterator for SongIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::DayIntro => {
                if self.current_day >= self.days.len() {
                    return None;
                }
                self.state = State::Gifts(self.current_day);
                Some(format!(
                    "On the {} day of Christmas my true love gave to me:\n",
                    self.days[self.current_day]
                ))
            }
            State::Gifts(gift) => {
                let gift_index = gift as usize;
                let line = if gift_index == 0 && self.current_day > 0 {
                    format!("and {}\n", self.gifts[gift_index])
                } else {
                    format!("{}\n", self.gifts[gift_index])
                };
                if gift == 0 {
                    self.current_day += 1;
                    self.state = State::DayIntro;
                } else {
                    self.state = State::Gifts(gift - 1);
                }
                Some(line)
            }
        }
    }
}

struct NumberedSongIter {
    song_iter: SongIter,
    line_number: usize,
}

impl NumberedSongIter {
    fn new() -> Self {
        NumberedSongIter {
            song_iter: SongIter::new(),
            line_number: 1,
        }
    }
}

impl Iterator for NumberedSongIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.song_iter.next() {
            let numbered_line = format!("{:02}: {}", self.line_number, line);
            self.line_number += 1;
            Some(numbered_line)
        } else {
            None
        }
    }
}

pub fn print_twelve_days_of_christmas() {
    let numbered_song_iter = NumberedSongIter::new();

    for line in numbered_song_iter {
        print!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use crate::song::print_twelve_days_of_christmas;
    #[test]
    fn test_print_twelve_days_of_christmas() {
        print_twelve_days_of_christmas();
    }
}
