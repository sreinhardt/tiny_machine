use azul::prelude::*;
use azul::css::css_parser;
use azul::widgets::{label::Label, button::Button, table_view::*};

pub struct UiModel {
    instructions: Vec<String>,
    memory: Vec<u8>,
    registers: Vec<u8>,
    in_port: Vec<u8>,
    out_port: Vec<u8>,
}
impl Default for UiModel {
    fn default() -> Self {
        UiModel {
            //instructions: Vec::with_capacity(20),
            instructions: vec!["Hello".to_string(), "rust".to_string()],
            //memory: Vec::with_capacity(20),
            memory: vec![0x00, 0x01, 0x02, 0x03],
            registers: Vec::with_capacity(4),
            in_port: Vec::with_capacity(10),
            out_port: Vec::with_capacity(10),
        }
    }
}
impl Layout for UiModel {
    fn layout(&self, info: LayoutInfo<Self>) -> Dom<Self> {
        let reset_button = Button::with_label("Reset State").dom();
        let next_button = Button::with_label("Next Instruction").dom();

        Dom::div()
            .with_id("main")
            .with_child(
                Dom::div()
                    .with_id("disassembly")
                    .with_child(self.disassembly())
                    .with_child(
                        Dom::div()
                            .with_id("memory")
                            .with_child(self.memory())
                    )
            )
            .with_child(
                Dom::div()
                    .with_class("row")
                    .with_child(reset_button)
                    .with_child(next_button)
            )
    }
}
impl UiModel {
    pub fn css(&self) -> Css {
        css_parser::new_from_str("
            * {
                box-sizing: border-box;
                flex-grow: 1;
                flex-direction: column;
            }
            #disassembly {
                background-color: gray;
                width: 300px;
                height: 500px;
                border: 1px solid black;
            }
        ").unwrap()
    }
    fn update_instructions(&mut self, instructions: Vec<String>) {
        self.instructions = instructions;
    }
    fn disassembly(&self) -> Dom<Self> {
        self.instructions.iter().enumerate()
            .map(|(idx, op)| {
                NodeData::label(op.clone())
            }).collect::<Dom<Self>>()
    }
    fn memory(&self) -> Dom<Self> {
        self.memory.iter().enumerate()
            .map(|(idx, val)| {
                NodeData::label(*val)
            }).collect::<Dom<Self>>()
    }
    fn registers(&self) -> Dom<Self> {
        self.registers.iter().enumerate()
            .map(|(idx, reg)| {
                NodeData::label(*reg)
            }).collect::<Dom<Self>>()
    }
}
