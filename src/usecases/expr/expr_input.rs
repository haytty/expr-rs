use derive_more::Constructor;
use getset::Getters;

#[derive(Debug, Constructor, Getters)]
pub struct ExprInput {
    #[getset(get = "pub")]
    arg: String,
}
