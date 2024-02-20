#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, SemiColon, Slash, Star,

    // One or two character tokens.
    BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    Class, // TODO: Remove this once you understand how the class infrastructure works.
    Fun, // TODO: Rename to Fn after implementing.
    True, False, // TODO: Replace this with intrinsic constants.
    This, // TODO: Rename to Self after implementing.
    Var, // TODO: Remove the necessity of this one after implementing variable declarations.
    Print, // TODO: Replace this with some type of intrinsic function.
    And, Else, For, If, Not, Null, Or, Return, Super, While,

    EOF,
}
