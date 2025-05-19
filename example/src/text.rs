#[derive(Clone, Copy)]
struct Glyph([[Color; 3]; 5]);

impl Glyph {
    const GX: Glyph = Glyph([
        [Color::BLACK, Color::BLACK, Color::BLACK],
        [Color::BLACK, Color::BLACK, Color::BLACK],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::BLACK, Color::WHITE, Color::BLACK],
        [Color::WHITE, Color::BLACK, Color::WHITE],
    ]);
    const G0: Glyph = Glyph([
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::WHITE, Color::WHITE],
    ]);
    const G1: Glyph = Glyph([
        [Color::BLACK, Color::WHITE, Color::BLACK],
        [Color::WHITE, Color::WHITE, Color::BLACK],
        [Color::BLACK, Color::WHITE, Color::BLACK],
        [Color::BLACK, Color::WHITE, Color::BLACK],
        [Color::WHITE, Color::WHITE, Color::WHITE],
    ]);
    const G2: Glyph = Glyph([
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::BLACK, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::BLACK],
        [Color::WHITE, Color::WHITE, Color::WHITE],
    ]);
    const G3: Glyph = Glyph([
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::BLACK, Color::BLACK, Color::WHITE],
        [Color::BLACK, Color::WHITE, Color::WHITE],
        [Color::BLACK, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::WHITE, Color::WHITE],
    ]);
    const G4: Glyph = Glyph([
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::BLACK, Color::BLACK, Color::WHITE],
        [Color::BLACK, Color::BLACK, Color::WHITE],
    ]);
    const G5: Glyph = Glyph([
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::BLACK],
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::BLACK, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::WHITE, Color::WHITE],
    ]);
    const G6: Glyph = Glyph([
        [Color::WHITE, Color::BLACK, Color::BLACK],
        [Color::WHITE, Color::BLACK, Color::BLACK],
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::WHITE, Color::WHITE],
    ]);
    const G7: Glyph = Glyph([
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::BLACK, Color::BLACK, Color::WHITE],
        [Color::BLACK, Color::BLACK, Color::WHITE],
        [Color::BLACK, Color::BLACK, Color::WHITE],
        [Color::BLACK, Color::BLACK, Color::WHITE],
    ]);
    const G8: Glyph = Glyph([
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::WHITE, Color::WHITE],
    ]);
    const G9: Glyph = Glyph([
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::BLACK, Color::BLACK, Color::WHITE],
        [Color::BLACK, Color::BLACK, Color::WHITE],
    ]);
    const GA: Glyph = Glyph([
        [Color::BLACK, Color::WHITE, Color::BLACK],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::WHITE],
    ]);
    const GB: Glyph = Glyph([
        [Color::WHITE, Color::WHITE, Color::BLACK],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::WHITE, Color::BLACK],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::WHITE, Color::BLACK],
    ]);
    const GC: Glyph = Glyph([
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::BLACK],
        [Color::WHITE, Color::BLACK, Color::BLACK],
        [Color::WHITE, Color::BLACK, Color::BLACK],
        [Color::WHITE, Color::WHITE, Color::WHITE],
    ]);
    const GD: Glyph = Glyph([
        [Color::WHITE, Color::WHITE, Color::BLACK],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::WHITE],
        [Color::WHITE, Color::WHITE, Color::BLACK],
    ]);
    const GE: Glyph = Glyph([
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::BLACK],
        [Color::WHITE, Color::WHITE, Color::BLACK],
        [Color::WHITE, Color::BLACK, Color::BLACK],
        [Color::WHITE, Color::WHITE, Color::WHITE],
    ]);
    const GF: Glyph = Glyph([
        [Color::WHITE, Color::WHITE, Color::WHITE],
        [Color::WHITE, Color::BLACK, Color::BLACK],
        [Color::WHITE, Color::WHITE, Color::BLACK],
        [Color::WHITE, Color::BLACK, Color::BLACK],
        [Color::WHITE, Color::BLACK, Color::BLACK],
    ]);

    const NUMBER: [Glyph; 16] = [
        Glyph::G0,
        Glyph::G1,
        Glyph::G2,
        Glyph::G3,
        Glyph::G4,
        Glyph::G5,
        Glyph::G6,
        Glyph::G7,
        Glyph::G8,
        Glyph::G9,
        Glyph::GA,
        Glyph::GB,
        Glyph::GC,
        Glyph::GD,
        Glyph::GE,
        Glyph::GF,
    ];
}

use core::ops::Index;

impl Index<(u8, u8)> for Glyph {
    type Output = Color;

    fn index(&self, index: (u8, u8)) -> &Self::Output {
        &self.0[index.1 as usize][index.0 as usize]
    }
}

#[derive(Clone, Copy)]
pub struct Color(u16);

impl Color {
    const WHITE: Color = Color::new(63, 63, 63);
    const BLACK: Color = Color::new(0, 0, 0);

    const fn normalize(v: u8) -> u16 {
        (v & 0b11111) as u16
    }

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        let (r, g, b) = (Self::normalize(r), Self::normalize(g), Self::normalize(b));
        Self(r << 10 | g << 5 | b)
    }
}

pub struct TextDisplay {
    row: u8,
    col: u8,
}

impl TextDisplay {
    pub fn init() -> Self {
        TextDisplay { row: 0, col: 0 }
    }

    pub fn new_line(&mut self) {
        self.row += 1;
        self.col = 0;
    }

    pub fn write(&mut self, value: u32) {
        self.write_glyph(Glyph::G0);
        self.write_glyph(Glyph::GX);

        for offset in (0..8).rev() {
            let n = ((value >> offset * 4) & 0xF) as usize;
            self.write_glyph(Glyph::NUMBER[n]);
        }

        self.new_line();
    }

    fn write_glyph(&mut self, glyph: Glyph) {
        let (x, y) = (self.col * 4, self.row * 6);
        for dy in 0..5 {
            for dx in 0..3 {
                pixel(1 + x + dx, 1 + y + dy, glyph[(dx, dy)]);
            }
        }
        self.col += 1;
    }
}

#[inline(always)]
fn pixel(x: u8, y: u8, color: Color) {
    const VRAM: *mut u16 = 0x0680_0000 as *mut u16;
    use core::ptr;
    unsafe {
        ptr::write_volatile(VRAM.wrapping_add(256 * y as usize + x as usize), color.0);
    }
}
