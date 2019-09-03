use orbtk::prelude::*;

use crate::machine::PORT_SIZE;

const WND_NAME:    &str = "portdata";
const IDX_CLASS:   &str = "idx";
const HEX_CLASS:   &str = "hex";
const ASCII_CLASS: &str = "ascii";

const IDX_WIDTH:    f64 = 10.0;
const HEX_WIDTH:    f64 = 50.0;
const ASCII_WIDTH:  f64 = 50.0;

pub struct PortView {
    data:        Vec<u8>,
    is_in_port:  bool,
    show_hex:    bool,
    show_ascii:  bool,
    idx_width:   f64,
    hex_width:   f64,
    ascii_width: f64
}
impl Default for PortView {
    fn default() -> Self {
        PortView {
            //data:        Vec::with_capacity(PORT_SIZE),
            data:        vec![ 0, 0x7f, 0x41, 0x53 ],
            is_in_port:  false,
            show_hex:    true,
            show_ascii:  true,
            idx_width:   IDX_WIDTH,
            hex_width:   HEX_WIDTH,
            ascii_width: ASCII_WIDTH
        }
    }
}
impl PortView {
    pub fn in_port() -> Self {
        Self::default()
    }
    pub fn out_port() -> Self {
        let mut v = Self::default();
        v.is_in_port = false;
        v
    }
    pub fn generate(&self, ctx: &mut BuildContext) -> Entity {
        let selector = Selector::from(WND_NAME);
        let mut grid = Grid::create()
            .selector(selector.clone())
            //.min_size(min_width, min_height)
            .columns(self.columns())
            .rows(self.rows());

        for (idx, val) in self.data.iter().enumerate() {
            let mut column = 0;
            grid = grid.child(
                TextBlock::create()
                    .selector(selector.clone().class(IDX_CLASS))
                    .text(format!{"{}", idx})
                    .attach(GridColumn(column))
                    .attach(GridRow(idx))
                    .build(ctx)
            );
            column += 1;
            if self.show_hex {
                grid = grid.child(
                    TextBlock::create()
                        .selector(selector.clone().class(HEX_CLASS))
                        .text(format!{"{:2X}", val})
                        .attach(GridColumn(column))
                        .attach(GridRow(idx))
                        .build(ctx)
                );
                column += 1;
            }
            if self.show_ascii {
                let val = if val < &32 || &126 < val { &46 } else { val };
                grid = grid.child(
                    TextBlock::create()
                        .selector(selector.clone().class(ASCII_CLASS))
                        .text(format!("{}", *val as char))
                        .attach(GridColumn(column))
                        .attach(GridRow(idx))
                        .build(ctx)
                );
                column += 1;
            }
        }

        grid.build(ctx)
    }
    fn columns(&self) -> Columns {
        let mut columns = Columns::create()
            .column(Column::create()
                .width(ColumnWidth::Auto)
                .min_width(self.idx_width)
                .build()
            );
        if self.show_hex {
            columns = columns.column(
                Column::create()
                    .width(ColumnWidth::Auto)
                    .min_width(self.hex_width)
                    .build()
                );
        }
        if self.show_ascii {
            columns = columns.column(
                Column::create()
                    .width(ColumnWidth::Auto)
                    .min_width(self.ascii_width)
                    .build()
                );
        }
        columns.build()
    }
    fn rows(&self) -> Rows {
        Rows::create()
            .repeat("*", self.data.len())
            .build()
    }
}
