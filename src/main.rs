struct I8080 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    halt: bool,
    flags: State,
}

struct State {
    flagbits: u8, // c, ac, z, p, int, s
}

/// genereate **set** and **clear** methods for State
macro_rules! gen_set_clear {
    ($scope:vis, $(($bit_symb:ident, $bit_index:path)),+) => {
        $(paste::item! {
            $scope fn [< set_ $bit_symb >](&mut self) {
                self.flagbits |= (1 << $bit_index);
            }
        }

        paste::item! {
            $scope fn [< clear_ $bit_symb >](&mut self) {
                self.flagbits &= !(1 << $bit_index);
            }
        })+
    }
}


impl State {
    const Z_FLAG_INDEX: u8 = 0;
    const C_FLAG_INDEX: u8 = 1;
    const AC_FLAG_INDEX: u8 = 2;
    const S_FLAG_INDEX: u8 = 3;
    const I_FLAG_INDEX: u8 = 4;

    gen_set_clear! {
        pub(crate),
        (z, State::Z_FLAG_INDEX),
        (c, State::C_FLAG_INDEX),
        (ac, State::AC_FLAG_INDEX),
        (s, State::S_FLAG_INDEX),
        (i, State::I_FLAG_INDEX)
    }
}

fn main() {
    println!("Hello, world!");
}
