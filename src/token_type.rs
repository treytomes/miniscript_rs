#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Slash, Star,
    SemiColon, NewLine, // Both of these are used to separate statements.

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
    This, // TODO: Rename to Self after implementing.
    Var, // TODO: Remove the necessity of this one after implementing variable declarations.
    Print, // TODO: Replace this with some type of intrinsic function.
    True, False, // TODO: I really like the idea of these being runtime constants.
    And, Else, For, If, Not, Null, Or, Return, Super, While,

    EOF,
}
