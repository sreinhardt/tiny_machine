use orbtk::prelude::*;

const WND_NAME:     &str = "memory";
const IDX_CLASS:    &str = "idx";
const HEX_CLASS:    &str = "hex";
const ASCII_CLASS:  &str = "ascii";

const ROW_SIZE:    usize = 8;
const IDX_WIDTH:   f64   = 10.0;
const HEX_WIDTH:   f64   = 100.0;
const ASCII_WIDTH: f64   = 100.0;

pub struct MemoryView {
    memory:      Vec<u8>,
    show_hex:    bool,
    show_ascii:  bool,
    row_size:    usize,
    idx_width:   f64,
    hex_width:   f64,
    ascii_width: f64,
}
impl Default for MemoryView {
    fn default() -> Self {
        MemoryView {
//            memory:      Vec::with_capacity(16),
            memory:      vec![
                            0, 1, 2, 3, 4, 5, 6, 7,
                            8, 9, 10, 11, 12, 13, 14, 15
                         ],
            show_hex:    true,
            show_ascii:  true,
            row_size:    ROW_SIZE,
            idx_width:   IDX_WIDTH,
            hex_width:   HEX_WIDTH,
            ascii_width: ASCII_WIDTH,
        }
    }
}
impl MemoryView {
    pub fn generate(&self, ctx: &mut BuildContext) -> Entity {
        let num_rows = self.memory.len() / self.row_size;
        let selector = Selector::from(WND_NAME);

        let mut grid = Grid::create()
            .selector(selector.clone())
            .columns(self.columns())
            .rows(self.rows(num_rows));
        for row in 0..num_rows {
            let lbound = row * self.row_size;
            let ubound = lbound + self.row_size;
            let slice = &self.memory[lbound..ubound];
            let mut col = 0;
            // index column
            grid = grid.child(
                TextBlock::create()
                    .selector(selector.clone().class(IDX_CLASS))
                    .text(format!{"{}", lbound})
                    .attach(GridColumn(col))
                    .attach(GridRow(row))
                    .build(ctx)
            );
            col += 1;
            // hex column
            if self.show_hex {
                // from u8 -> string
                let mut hex_str = slice.iter()
                    .map(|v| format!{"{:02x} ", v})
                    .collect::<String>();
                let _ = hex_str.pop(); // remove trailing space
                grid = grid.child(
                    TextBlock::create()
                        .selector(selector.clone().class(HEX_CLASS))
                        .text(hex_str)
                        .attach(GridColumn(col))
                        .attach(GridRow(row))
                        .build(ctx)
                );
                col += 1;
            }
            if self.show_ascii {
                // from u8 -> string
                let mut ascii_str = slice.iter()
                    .map(|mut v|{
                        if v < &32 || &126 < v {
                            v = &46;
                        }
                        format!{"{} ", *v as char}
                    })
                    .collect::<String>();
                let _ = ascii_str.pop(); // remove trailing space
                grid = grid.child(
                    TextBlock::create()
                        .selector(selector.clone().class(HEX_CLASS))
                        .text(ascii_str)
                        .attach(GridColumn(col))
                        .attach(GridRow(row))
                        .build(ctx)
                );
            }
        }
        grid.build(ctx)
    }
    fn columns(&self) -> Columns {
        let mut columns = Columns::create()
            .column(Column::create()
                .width(ColumnWidth::Auto)
//                .min_width(self.idx_width)
                .build());
        if self.show_hex {
            columns = columns.column(Column::create()
                .width(ColumnWidth::Auto)
//                .min_width(self.hex_width)
                .build());
        }
        if self.show_ascii {
            columns = columns.column(Column::create()
                .width(ColumnWidth::Auto)
//                .min_width(self.ascii_width)
                .build());
        }
        columns.build()
    }
    fn rows(&self, num: usize) -> Rows {
        Rows::create()
			.repeat("*", num)
            .build()
    }
}
