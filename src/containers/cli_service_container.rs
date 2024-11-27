use crate::usecases::expr::expr_interactor::ExprInteractor;
use crate::usecases::expr::expr_usecase::ExprUsecase;
use getset::Getters;

#[derive(Debug, Getters)]
pub struct CliServiceContainer {
    #[getset(get = "pub")]
    expr_usecase: Box<dyn ExprUsecase>,
}

impl CliServiceContainer {
    pub fn build() -> Self {
        let expr_interactor = Box::new(ExprInteractor::new());

        Self {
            expr_usecase: expr_interactor,
        }
    }
}
