use crate::domain::models::Operator;

pub trait OperatorRepoTrait: Send + Sync {
    fn get_by_id(&self, id: i32) -> anyhow::Result<Option<Operator>>;
    fn create(&self, operator: &Operator) -> anyhow::Result<()>;
    fn update_by_id(&self, operator: &Operator) -> anyhow::Result<()>;
    fn list(&self) -> anyhow::Result<Vec<Operator>>;
}
