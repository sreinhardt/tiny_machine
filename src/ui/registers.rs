use bit_field::*;
use orbtk::prelude::*;

use crate::registers::{CF_BIT, ZF_BIT, OF_BIT, HF_BIT};


const WND_NAME:         &str = "registers";
const LABEL_CLASS:      &str = "lbl";
const HEX_CLASS:        &str = "hex";
const DEC_CLASS:        &str = "dec";
const FLAG_ON_CLASS:    &str = "flag_on";
const FLAG_OFF_CLASS:   &str = "flag_off";

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
        let mut row = 0;
        let mut column = 0;
        let disp_lbl = if self.show_hex { HEX_CLASS } else { DEC_CLASS };
        let selector = Selector::from(WND_NAME);

        let mut grid = Grid::create()
            .selector(selector.clone())
            .columns(self.columns())
            .rows(self.rows());
        // build individual register rows
        for (label, value) in vec![ ("IP", self.ip), ("LI", self.li), ("AC", self.ac), ("FR", self.fr) ] {
            let value = if self.show_hex { format!{"{:2X}", value} }
                        else { format!{"{}", value} };
            grid = grid.child(
                    TextBlock::create()
                        .selector(selector.clone().class(LABEL_CLASS))
                        .text(label)
                        .attach(GridColumn(column))
                        .attach(ColumnSpan(1))
                        .attach(GridRow(row))
                        .build(ctx)
                ).child(
                    TextBlock::create()
                        .selector(selector.clone().class(disp_lbl))
                        .text(value)
                        .attach(GridColumn(column+2)) // account for span
                        .attach(ColumnSpan(1))
                        .attach(GridRow(row))
                        .build(ctx)
                );
            row += 1;
        }
        // build FR bit flags
        let flags = vec![
            ("CF", self.fr.get_bit(CF_BIT)),
            ("ZF", self.fr.get_bit(ZF_BIT)),
            ("OF", self.fr.get_bit(OF_BIT)),
            ("HF", self.fr.get_bit(HF_BIT))
        ];
        for (label, value) in flags.iter() {
            let flag = if *value { FLAG_OFF_CLASS } else { FLAG_ON_CLASS };
            grid = grid.child(
                TextBlock::create()
                    .selector(selector.clone().class(flag))
                    .text(*label)
                    .attach(GridColumn(column))
                    .attach(GridRow(row))
                    .build(ctx)
            );
            column += 1;
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
                4 // repeat 4
            ).build()
    }
    fn rows(&self) -> Rows {
        Rows::create()
            .repeat("*", 5)
            .build()
    }
}
