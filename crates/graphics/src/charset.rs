pub struct Charset<'a> {
    entries: &'a [Entry],
}

#[derive(Debug)]
enum Entry {
    Range { start: char, end: char },
    Single { value: char },
}

pub struct CharsetIter<'a> {
    entries: std::slice::Iter<'a, Entry>,
    current_range: Option<(char, char)>,
}

impl Iterator for CharsetIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((ref mut current, end)) = self.current_range
        {
            if *current <= end {
                let ret = *current;

                if let Some(next_char) =
                    std::char::from_u32((*current as u32) + 1)
                {
                    *current = next_char;
                    if *current > end {
                        self.current_range = None;
                    }
                } else {
                    self.current_range = None;
                }
                return Some(ret);
            } else {
                self.current_range = None;
            }
        }

        if let Some(entry) = self.entries.next() {
            match entry {
                Entry::Range { start, end } => {
                    self.current_range = Some((*start, *end));

                    return self.next();
                }
                Entry::Single { value } => {
                    return Some(*value);
                }
            }
        }
        None
    }
}

impl Charset<'_> {
    pub const ENGLISH: Self = Self {
        entries: &[
            Entry::Range {
                start: 'A',
                end: 'Z',
            },
            Entry::Range {
                start: 'a',
                end: 'z',
            },
            Entry::Range {
                start: '0',
                end: '9',
            },
            Entry::Single { value: ' ' },
            Entry::Single { value: '!' },
            Entry::Single { value: '"' },
            Entry::Single { value: '#' },
            Entry::Single { value: '$' },
            Entry::Single { value: '%' },
            Entry::Single { value: '&' },
            Entry::Single { value: '\'' },
            Entry::Single { value: '(' },
            Entry::Single { value: ')' },
            Entry::Single { value: '*' },
            Entry::Single { value: '+' },
            Entry::Single { value: ',' },
            Entry::Single { value: '-' },
            Entry::Single { value: '.' },
            Entry::Single { value: '/' },
            Entry::Single { value: ':' },
            Entry::Single { value: ';' },
            Entry::Single { value: '<' },
            Entry::Single { value: '=' },
            Entry::Single { value: '>' },
            Entry::Single { value: '?' },
            Entry::Single { value: '@' },
            Entry::Single { value: '[' },
            Entry::Single { value: '\\' },
            Entry::Single { value: ']' },
            Entry::Single { value: '^' },
            Entry::Single { value: '_' },
            Entry::Single { value: '`' },
            Entry::Single { value: '{' },
            Entry::Single { value: '|' },
            Entry::Single { value: '}' },
            Entry::Single { value: '~' },
        ],
    };

    pub fn iter(&self) -> CharsetIter {
        CharsetIter {
            entries: self.entries.iter(),
            current_range: None,
        }
    }
}
