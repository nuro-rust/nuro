use std::fmt::Debug;

/// 图状态抽象：
///
/// - `S` 本身承载整个有向图在某一时刻的状态；
/// - 每个节点执行后返回一个 `Update`，由状态自身通过 `apply_update` 合并。
///
/// 在简单场景中，可以让 `Update = Self`，在 `apply_update` 中直接覆盖字段；
/// 在复杂场景中，可以让 `Update` 只包含部分字段，并在此方法中实现精细的合并策略。
pub trait GraphStateTrait: Send + Sync + Clone + Debug + 'static {
    type Update;

    fn apply_update(&mut self, update: Self::Update);
}
