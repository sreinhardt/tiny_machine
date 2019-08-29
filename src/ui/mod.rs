use orbtk::prelude::*;

pub mod disassembly;
pub mod memory;
pub mod registers;

use disassembly::DisassemblyView;
use memory::MemoryView;
use registers::RegisterView;

pub struct UiModelState {
    disassembly: DisassemblyView,
    memory:      MemoryView,
    registers:   RegisterView,
}
impl Default for UiModelState {
    fn default() -> Self {
        UiModelState {
            disassembly: DisassemblyView::default(),
            memory:      MemoryView::default(),
            registers:   RegisterView::default(),
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
                .rows(
                    Rows::create()
                        .repeat("*", 3)
                        .build()
                ).child(
                    Container::create()
                        .attach(GridRow(0))
                        .child(state.disassembly.generate(ctx))
                        .build(ctx),
                ).child(
                    Container::create()
                        .attach(GridRow(1))
                        .child(state.memory.generate(ctx))
                        .build(ctx),
                ).child(
                    Container::create()
                        .attach(GridRow(2))
                        .child(state.registers.generate(ctx))
                        .build(ctx),
                ).build(ctx)
        )
    }
}
