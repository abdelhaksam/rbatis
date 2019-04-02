use std::collections::HashMap;
use serde_json::Value;

/**
* 表达式引擎，T:lexer类型,R 返回类型
**/

pub trait ExpressionEngine<T, R> {
    fn Name(&self) -> String;
    fn Lexer(&self,lexerArg: String) -> Result<T, String>;
    fn Eval(&self, lexerResult: &T, arg: &Value) -> Result<R, String>;
}