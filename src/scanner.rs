#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Lambda,

    Eof,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        match &self.literal {
            Some(literal) => match literal {
                Literal::Identifier(s) => {
                    format!("{:?} {:?} {:?}", self.token_type, self.lexeme, s)
                }
                Literal::Str(s) => format!("{:?} {:?} {:?}", self.token_type, self.lexeme, s),
                Literal::Number(n) => format!("{:?} {:?} {:?}", self.token_type, self.lexeme, n),
            },
            None => format!("{:?} {:?}", self.token_type, self.lexeme),
        }
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

impl Scanner {
    fn new(source: String) -> Scanner{
        Scanner{
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, String::new(), None, self.line));
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // TODO: implement scan_token
    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
           '(' => self.add_token(TokenType::LeftParen),
           ')' => self.add_token(TokenType::RightParen),
           '{' => self.add_token(TokenType::LeftBrace),
           '}' => self.add_token(TokenType::RightBrace),
           ',' => self.add_token(TokenType::Comma),
           '.' => self.add_token(TokenType::Dot),
           '-' => self.add_token(TokenType::Minus),
           '+' => self.add_token(TokenType::Plus),
           ';' => self.add_token(TokenType::Semicolon),
           '*' => self.add_token(TokenType::Star),
           _ => {}
        }
    }

    fn advance(&mut self) -> char{
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type:TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, text.to_string(), None, self.line));
    }
}
