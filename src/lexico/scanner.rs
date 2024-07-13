use std::fs::File;
use std::io::{BufReader, Read};

const BUFFER_SIZE: usize = 500;

/// escaneia arquivo e converte todos os lexemas em tokens
pub struct Scanner {
    stream: BufReader<File>,
    buffer: [u8; 2 * BUFFER_SIZE],
    pointer: usize,
    current_buffer: usize,
    lexeme_start: usize,
    lexeme: String,
}

impl Scanner {

    /// incrementa o ponteiro do buffer, recarregando a proxima metade, se necessario
    fn increment(&mut self) {
        self.pointer += 1;
        if self.pointer == BUFFER_SIZE {
            self.load_buffer(1);
        } else if self.pointer == BUFFER_SIZE * 2 {
            self.load_buffer(0);
            self.pointer = 0;
        }
    }

    /// recarrega uma das metades do buffer
    fn load_buffer(&mut self, buffer_half: usize) {
        if self.current_buffer != buffer_half {
            self.current_buffer = buffer_half;
            let n = self.stream.read(&mut self.buffer[buffer_half * BUFFER_SIZE..buffer_half * BUFFER_SIZE + BUFFER_SIZE]).unwrap();
            if n < BUFFER_SIZE {
                self.buffer[self.current_buffer * BUFFER_SIZE + n] = b'\0';
            }
        }
    }

    /// retorna proximo caracter do buffer
    fn next_buffer_char(&mut self) -> char {
        let c = self.buffer[self.pointer];
        self.increment();
        c as char
    }
    
    /// retorna instancia de scanner
    pub fn new(file: &str) -> Self {
        let file = File::open(file).unwrap();

        let mut reader = Self {
            stream: BufReader::new(file),
            buffer: [0; 2 * BUFFER_SIZE],
            current_buffer: 1,
            lexeme_start: 0,
            lexeme: "".to_string(),
            pointer: 0
        };

        reader.load_buffer(0);

        reader
    }

    /// retorna proximo caracter do buffer apos adiciona-lo ao lexema atual
    pub fn next_char(&mut self) -> char {
        let c = self.next_buffer_char();
        self.lexeme += &c.to_string();
        c
    }

    /// decrementa o ponteiro do buffer e remove o ultimo caracter do lexema atual
    pub fn decrement(&mut self) {
        if self.pointer > 0 {
            self.pointer -= 1;
        } else {
            self.pointer = BUFFER_SIZE * 2 - 1
        }
        self.lexeme.pop();
    }

    /// reinicia lexema
    pub fn reset(&mut self) {
        self.pointer = self.lexeme_start;
        self.lexeme = "".to_string();
    }

    /// confirma lexema
    pub fn confirm(&mut self) {
        self.lexeme_start = self.pointer;
        self.lexeme = "".to_string();
    }

    /// retorna lexema
    pub fn get_lexeme(&mut self) -> String {
        self.lexeme.to_string()
    }

    /// debug: exibe conteúdo do buffer
    pub fn print_buffer(&mut self) {
        let mut out = "Buffer:[".to_string();
        for i in 0..BUFFER_SIZE * 2 {
            let chr = self.buffer[i] as char;
            match chr {
                '\n' => out += "\\",
                '\t' => out += "/",
                _ => out += &chr.to_string(),
            }
        }
        out += "]\n        ";
        for i in 0..BUFFER_SIZE * 2 {
            out += if i == self.pointer {"^"} else {" "}
        }
        println!("{}", out);
        println!("{}", self.pointer);
    }
}