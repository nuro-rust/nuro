//! nuro-macros — 占位用过程宏实现。
//!
//! 当前仅提供三个 no-op 宏：
//! - `#[derive(GraphState)]`
//! - `#[derive(Tool)]`
//! - `#[agent]`
//!
//! 这些宏不会生成任何额外代码，只是原样返回输入 TokenStream，
//! 主要用于保证对外 API 形状稳定，后续可在不破坏用户代码的前提下
//! 增量实现真正的代码生成逻辑。

use proc_macro::TokenStream;

#[proc_macro_derive(GraphState, attributes(reducer))]
pub fn derive_graph_state(input: TokenStream) -> TokenStream {
    // 目前为 no-op，实现仅返回原始输入。
    input
}

#[proc_macro_derive(Tool, attributes(tool))]
pub fn derive_tool(input: TokenStream) -> TokenStream {
    // 目前为 no-op，实现仅返回原始输入。
    input
}

#[proc_macro_attribute]
pub fn agent(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 目前为 no-op，实现仅返回原始输入。
    item
}
