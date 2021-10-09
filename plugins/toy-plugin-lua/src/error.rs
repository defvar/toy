use std::backtrace::Backtrace;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum LuaFunctionError {
    #[error("error: {:?}", source)]
    LuaExecutionError {
        #[from]
        source: rlua::Error,
        backtrace: Backtrace,
    },
}
