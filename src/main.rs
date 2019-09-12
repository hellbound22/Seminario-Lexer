use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use base_x::{decode, encode};

#[derive(Debug, Clone)]
struct Palavra {
    pub definicao: String,
    pub contagem: Vec<usize>,
}

impl PartialEq for Palavra {
    fn eq(&self, other: &Self) -> bool {
        self.definicao == other.definicao
    }
}

#[derive(Debug, Clone)]
struct Dict {
    lista: HashMap<String, Palavra>,
}

impl Dict {
    pub fn new() -> Dict {
        Dict {
            lista: HashMap::new(),
        }
    }

    pub fn check_repitido(&mut self, s: String, l: usize) {
        let np = Palavra {
            definicao: s.clone(),
            contagem: vec![l],
        };

        match self.lista.get_mut(&s) {
            Some(x) => x.contagem.push(l),
            None => {
                self.lista.insert(s, np);
            }
        }
    }

    pub fn list(&self) -> Vec<Palavra> {
        let mut v: Vec<Palavra> = Vec::new();

        for p in self.lista.iter() {
            v.push(p.1.clone())
        }

        v.sort_by(|a, b| a.definicao.partial_cmp(&b.definicao).unwrap());

        v
    }
}

fn main() {
    // Filtro
    let filtro: Vec<char> = vec!['\t', '\r', '\n', ','];

    // Abre o arquivo
    let mut file = File::open("texto.txt").unwrap();
    let mut s = String::new();

    file.read_to_string(&mut s).unwrap();

    //Caracteres individuais
    let indv = s.chars();
    dbg!(indv);

    // retira as virgulas
    let s = s.replace(",", " ");

    // Divide as linhas
    let tmp: Vec<&str> = s.lines().collect();
    let mut split: Vec<String> = Vec::new();

    for l in tmp {
        if l == "" {
            continue;
        }
        let l = l.trim();
        split.push(l.to_string());
    }

    // Inicia o dicionário separando as palavras e contando as ocorrências delas
    let mut dict = Dict::new();

    for (i, ss) in split.iter_mut().enumerate() {
        for p in ss.split_whitespace() {
            dict.check_repitido(p.to_string(), i + 1)
        }
    }

    dbg!(dict.list());

    let num = 1488.to_string();

    let x = converter_para_base(BASE2, &num);

    println!("Numero em base 2: {}", converter_para_base(BASE2, &num));
    println!("Numero em base 16: {}", converter_para_base(BASE16, &num));
    println!("Numero em base 64: {}", converter_para_base(BASE64, &num));
    println!("Numero decimal: {}", converter_de_base(BASE2, &x))
}

const BASE2: &str = "01";
const BASE16: &str = "0123456789abcdef";
const BASE64: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

fn converter_para_base(base: &str, src: &str) -> String {
    encode(base, src.as_bytes())
}

fn converter_de_base(base: &str, src: &str) -> String {
    String::from_utf8(decode(base, src).unwrap()).unwrap()
}
