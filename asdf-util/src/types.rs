pub enum ASDFCommand<'a> {
    PLUGIN(&'a str),
    CURRENT(&'a str),
    GLOBAL(&'a str),
    HELP(&'a str),
    INSTALL(&'a str),
    LATEST(&'a str),
    LIST(&'a str),
    LOCAL(&'a str),
    SHELL(&'a str),
    UNINSTALL(&'a str),
    WHERE(&'a str),
    WHICH(&'a str),
    EXEC(&'a str),
    ENV(&'a str),
    INFO,
    VERSION,
    RESHIM(&'a str),
    SHIMVERSION(&'a str),
    UPDATE,
    UPDATEHEAD,
}

pub enum CommandReturnType {
    CmdString(String),
    Empty,
}
