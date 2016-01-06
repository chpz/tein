use std::io::BufRead;
use std::str::FromStr;
use std::iter::Iterator;
use std::marker::PhantomData;

pub trait Inputer<Output, R: BufRead> {
    fn next(&mut self) -> Option<Output>;
    fn iter(&mut self) -> InputIter<Output, R>;
}

pub struct Tein<BR: BufRead> {
    line_reader: BR,
    buffer_line: String,
    idx: usize,
}

pub struct InputIter<'a, T, BR: BufRead + 'a> {
    reader: &'a mut Tein<BR>,
    _input_type: PhantomData<T>,
}

impl<'a, T, R> Iterator for InputIter<'a, T, R> where T: FromStr, R: BufRead {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.reader.next()
    }
}

impl<BR: BufRead> Tein<BR> {
    pub fn new(br: BR) -> Tein<BR> {
        Tein {
            line_reader: br,
            buffer_line: String::new(),
            idx: 0,
        }
    }
}

impl<T, BR> Inputer<T, BR> for Tein<BR> where T: FromStr, BR: BufRead {
    fn next(&mut self) -> Option<T> {
        loop {
            if let Some(beg_idx) = self.buffer_line[self.idx..].find(|c| !char::is_whitespace(c)) { 
                let beg_idx = self.idx + beg_idx;
                let end_idx = beg_idx + match self.buffer_line[beg_idx..].find(char::is_whitespace) {
                    Some(n) => n,
                    None    => self.buffer_line[beg_idx..].len(),
                };
                self.idx = end_idx;
                return self.buffer_line[beg_idx..end_idx].parse().ok();
            } else {
                self.buffer_line.clear();
                if let Ok(n) = self.line_reader.read_line(&mut self.buffer_line) {
                    if n > 0 {
                        self.idx = 0;
                        continue;
                    }
                }
            }
            return None;
        }
    }

    fn iter(&mut self) -> InputIter<T, BR> {
        InputIter {
            reader: self,
            _input_type: PhantomData,
        }
    }
}
