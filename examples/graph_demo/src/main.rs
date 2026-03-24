use std::collections::HashMap;

use anyhow::Result;
use nuro::prelude::*;

/// 一个极简的 StateGraph 示例：
///
/// 图结构： start -> (left | right) -> end
/// - 入口节点 `start` 根据状态中的 `next` 字段进行条件路由；
/// - `left` 与 `right` 节点只是在文本上追加标记；
/// - `end` 节点用于终止执行。
///
/// 运行示例：
/// ```bash
/// # 走 left 分支
/// cargo run -p graph_demo -- left
///
/// # 走 right 分支
/// cargo run -p graph_demo -- right
/// ```
#[derive(Debug, Clone)]
struct DemoState {
    text: String,
    next: String,
}

impl GraphStateTrait for DemoState {
    type Update = DemoState;

    fn apply_update(&mut self, update: Self::Update) {
        if !update.text.is_empty() {
            if !self.text.is_empty() {
                self.text.push(' ');
            }
            self.text.push_str(&update.text);
        }
        if !update.next.is_empty() {
            self.next = update.next;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 从命令行参数决定走哪个分支，默认为 left。
    let initial_next = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "left".to_string());

    // 构建图：4 个节点 + 一条条件边。
    let graph = StateGraph::<DemoState>::new()
        .add_node(
            "start",
            FnNode::new(|state: &DemoState, _ctx: &mut NodeContext| DemoState {
                text: format!("{}[start]", state.text),
                next: state.next.clone(),
            }),
        )
        .add_node(
            "left",
            FnNode::new(|state: &DemoState, _ctx: &mut NodeContext| DemoState {
                text: format!("{} -> [left]", state.text),
                next: state.next.clone(),
            }),
        )
        .add_node(
            "right",
            FnNode::new(|state: &DemoState, _ctx: &mut NodeContext| DemoState {
                text: format!("{} -> [right]", state.text),
                next: state.next.clone(),
            }),
        )
        .add_node(
            "end",
            FnNode::new(|state: &DemoState, _ctx: &mut NodeContext| DemoState {
                text: format!("{} -> [end]", state.text),
                next: state.next.clone(),
            }),
        )
        .set_entry_point("start")
        .add_conditional_edge(
            "start",
            |state: &DemoState| state.next.clone(),
            HashMap::from([
                ("left".to_string(), "left".to_string()),
                ("right".to_string(), "right".to_string()),
            ]),
        )
        .add_edge("left", "end")
        .add_edge("right", "end")
        .set_finish_point("end")
        .compile()?;

    let initial_state = DemoState {
        text: "graph_demo".to_string(),
        next: initial_next,
    };

    let result = graph.invoke(initial_state).await?;
    println!("Final state: {:?}", result);

    Ok(())
}
