use orbtk::prelude::*;

const WND_NAME:   &str = "registers";
const LABEL_NAME: &str = "reg_lbl";
const HEX_NAME:   &str = "reg_hex";
const DEC_NAME:   &str = "reg_dec";
const FLAG_ON:    &str = "reg_flag_on";
const FLAG_OFF:   &str = "reg_flag_off";

const LABEL_WIDTH: f64 = 30.0;
const VALUE_WIDTH: f64 = 30.0;
const FLAG_WIDTH:  f64 = 15.0;

pub struct RegisterView {
    ip: u8,
    li: u8,
    ac: u8,
    fr: u8,
    show_hex:    bool,
    label_width: f64,
    value_width: f64,
    flag_width:  f64,
}
impl Default for RegisterView {
    fn default() -> Self {
        RegisterView {
            ip: 0,
            li: 0,
            ac: 0,
            fr: 0,
            show_hex:    true,
            label_width: LABEL_WIDTH,
            value_width: VALUE_WIDTH,
            flag_width:  FLAG_WIDTH,
        }
    }
}
impl RegisterView {
    pub fn generate(&self, ctx: &mut BuildContext) -> Entity {
        let disp_lbl = if self.show_hex { HEX_NAME } else { DEC_NAME };
        let mut grid = Grid::create()
            .name(WND_NAME)
            .columns(self.columns())
            .rows(self.rows());
        let mut row = 0;
        for (label, value) in vec![ ("IP", self.ip), ("LI", self.li), ("AC", self.ac), ("FR", self.fr) ] {
            let value = if self.show_hex { format!{"{:2X}", value} }
                        else { format!{"{}", value} };
            grid = grid.child(
                    TextBlock::create()
                        .name(LABEL_NAME)
                        .text(label)
                        .attach(GridColumn(0))
                        .attach(ColumnSpan(1))
                        .attach(GridRow(row))
                        .build(ctx)
                ).child(
                    TextBlock::create()
                        .name(disp_lbl)
                        .text(value)
                        .attach(GridColumn(2))
                        .attach(ColumnSpan(1))
                        .attach(GridRow(row))
                        .build(ctx)
                );
            row += 1;
        }
        // TODO flags bit boxes
        grid.build(ctx)
    }
    fn columns(&self) -> Columns {
        Columns::create()
            .repeat(
                Column::create()
                    .width(ColumnWidth::Auto)
                    .min_width(self.flag_width)
                    .build(),
                4
            ).build()
    }
    fn rows(&self) -> Rows {
        Rows::create()
            .repeat("*", 5)
            .build()
    }
}
