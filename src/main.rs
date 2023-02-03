use async_trait::async_trait;
use seedwing_policy_engine::lang::lir::EvalContext;
use seedwing_policy_engine::runtime::{EvaluationResult, World};

#[async_trait]
pub trait SomeOtherTrait {
    async fn foo(&self) -> anyhow::Result<EvaluationResult>;
}

pub struct MyImpl {
    world: World,
}

#[async_trait]
impl SomeOtherTrait for MyImpl {
    async fn foo(&self) -> anyhow::Result<EvaluationResult> {
        Ok(self
            .world
            .evaluate("path", "input", EvalContext::default())
            .await?)
    }
}

impl MyImpl {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }
}

#[tokio::main]
async fn main() {
    let s = MyImpl::new();

    s.foo().await.unwrap();
}
