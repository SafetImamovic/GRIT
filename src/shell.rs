use std::env;

pub fn get_shells() -> String
{
        if cfg!(windows)
        {
                env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string())
        }
        else
        {
                env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
        }
}
