use orbtk::prelude::*;

const IDX_NAME: &str = "disas_idx";
const HEX_NAME: &str = "disas_hex";
const OP_NAME:  &str = "disas_op";

const IDX_WIDTH: f64 = 10.0;
const HEX_WIDTH: f64 = 30.0;
const OP_WIDTH:  f64 = 100.0;

pub struct DisassemblyView {
    instructions:   Vec<String>,
    show_hex:       bool,
    idx_width:      f64,
    hex_width:      f64,
    opcode_width:   f64,
}
impl Default for DisassemblyView {
    fn default() -> Self {
        DisassemblyView {
            //instructions: Vec::with_capacity(20),
            instructions: vec![
                "Hello".to_string(),
                "rust".to_string(),
                "Lets".to_string(),
                "make".to_string(),
                "a".to_string(),
                "ui".to_string()
            ],
            show_hex:       false,
            idx_width:      IDX_WIDTH,
            hex_width:      HEX_WIDTH,
            opcode_width:   OP_WIDTH,
        }
    }
}
impl DisassemblyView {
    pub fn generate(&self, ctx: &mut BuildContext) -> Entity {
        let mut columns = Columns::create()
            .column(Column::create()
                .width(ColumnWidth::Auto)
                .min_width(self.idx_width)
                .build());
        if self.show_hex {
            columns = columns.column(Column::create()
                .width(ColumnWidth::Auto)
                .min_width(self.hex_width)
                .build());
        }
        let columns = columns.column(Column::create()
                        .width(ColumnWidth::Auto)
                        .min_width(self.opcode_width)
                        .build())
                    .build();

        let mut grid = Grid::create()
            .name("disassembly")
            //.min_size(min_width, min_height)
            .columns(columns)
            .rows(Rows::create()
                .repeat("*", self.instructions.len())
                .build()
            );
        for idx in 0..self.instructions.len() {
            let mut col = 0;
            grid = grid.child( // row count
                TextBlock::create()
                    .name(IDX_NAME)
                    .text(format!{"{}", idx})
                    .attach(GridColumn(col))
                    .attach(GridRow(idx))
                    .build(ctx),
            );
            col += 1;
            if self.show_hex {
                grid = grid.child(
                    TextBlock::create()
                        .name(HEX_NAME)
                        .text("0x00")
                        .attach(GridColumn(col))
                        .attach(GridRow(idx))
                        .build(ctx),
                );
                col += 1;
            }
            grid = grid.child( // actual opcodes
                TextBlock::create()
                    .name(OP_NAME)
                    .text(self.instructions[idx].as_ref())
                    .attach(GridColumn(col))
                    .attach(GridRow(idx))
                    .build(ctx),
            );
        }
        grid.build(ctx)
    }
}
