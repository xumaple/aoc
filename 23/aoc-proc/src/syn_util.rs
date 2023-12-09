use super::*;

pub enum StrLitOrExpr {
    LitStr(LitStr),
    Expr(Expr),
}

impl parse::Parse for StrLitOrExpr {
    fn parse(input: parse::ParseStream<'_>) -> Result<Self> {
        let first = input.parse();
        if let Ok(lit_str) = first {
            return Ok(Self::LitStr(lit_str));
        }
        Ok(Self::Expr(input.parse()?))
    }
}
