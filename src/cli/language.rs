use clap::ValueEnum;

#[derive(Clone, ValueEnum, Debug)]
pub enum Language {
    Eng,
    Chs,
    Jpn,
    JpnRo
}
