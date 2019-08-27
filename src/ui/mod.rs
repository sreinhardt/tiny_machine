use orbtk::prelude::*;

pub mod disassembly;
pub use disassembly::DisassemblyView;

pub struct UiModelState {
    disassembly: DisassemblyView,
}
impl Default for UiModelState {
    fn default() -> Self {
        UiModelState {
            disassembly: DisassemblyView::default(),
        }
    }
}
impl State for UiModelState {
    fn update(&self, ctx: &mut Context<'_>) {

    }
}

widget!(UiModel<UiModelState>);
impl Template for UiModel {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let state = self.clone_state();

        self.name("UiModel").child(
            Grid::create()
                .rows(Rows::create().row("*").row("*").build())
                .child(
                    Container::create()
                        .attach(GridRow(0))
                        .child(state.disassembly.generate(ctx))
                        .build(ctx),
                ).build(ctx),
        )
    }
}
