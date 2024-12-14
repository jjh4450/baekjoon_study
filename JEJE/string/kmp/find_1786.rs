// SPDX-License-Identifier: AGPL-3.0-or-later
// NOTICE: This code is restricted from being used for any AI or machine learning training purposes.
use std::io::{stdin, Read, stdout, Write, BufWriter};

struct _SString {
    data: String,
    char_indices: Vec<usize>,
}

impl _SString {
    fn new(data: String) -> Self {
        let char_indices = data.char_indices()
                               .map(|(i, _)| i)
                               .collect();
        _SString { data, char_indices }
    }

    fn from_str(data: &str) -> Self {
        let char_indices = data.char_indices()
                               .map(|(i, _)| i)
                               .collect();
        _SString { data: data.to_string(), char_indices }
    }

    fn len(&self) -> usize {
        self.char_indices.len()
    }
}

use std::ops::Index;
impl Index<usize> for _SString {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.len() {
            let start = self.char_indices[index];
            let end = if index + 1 < self.len() {
                          self.char_indices[index + 1]
                      } else {
                          self.data.len()
                      };
            &self.data[start..end]
        } else {
            panic!("Index out of bounds");
        }
    }
}

impl std::fmt::Display for _SString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

fn get_pi(p: &_SString) -> Vec<usize> {
    let m = p.len();
    let mut pi = vec![0; m];
    let mut begin = 1;
    let mut matched = 0;
    while begin + matched < m {
        if p[begin + matched] == p[matched] {
            matched += 1;
            pi[begin + matched - 1] = matched;
        } else {
            if matched == 0 {
                begin += 1;
            } else {
                begin += matched - pi[matched - 1];
                matched = pi[matched - 1];
            }
        }
    }
    pi
}

fn kmp(s: &_SString, p: &_SString) -> Vec<usize> {
    let mut ret = Vec::new();
    let pi = get_pi(p);
    let mut j = 0;
    for i in 0..s.len() {
        while j > 0 && s[i] != p[j] {
            j = pi[j - 1];
        }
        if s[i] == p[j] {
            if j == p.len() - 1 {
                ret.push(i - p.len() + 1);
                j = pi[j];
            } else {
                j += 1;
            }
        }
    }
    ret
}


fn main(){
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);

    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut input = buf.split('\n');

    let t = _SString::from_str(input.next().unwrap());
    let p = _SString::from_str(input.next().unwrap());

    // println!("{}", &p[p.len() - 1]);

    let ret = kmp(&t, &p);
    write!(writer, "{}\n{}\n", ret.len(), ret.iter().map(|x| x + 1).map(|x| x.to_string()).collect::<Vec<String>>().join(" ")).unwrap();
}