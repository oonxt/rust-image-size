pub const _EMUS_PER_INCH: f32 = 914400f32;
pub const _EMUS_PER_CM: f32 = 360000f32;
pub const _EMUS_PER_MM: f32 = 36000f32;
pub const _EMUS_PER_PT: f32 = 12700f32;
pub const _EMUS_PER_TWIP: f32 = 635f32;

#[derive(Debug, Clone)]
pub enum Length {
    Inches(f32),
    Cm(f32),
    Emu(f32),
    Mm(f32),
    Pt(f32),
    Twips(f32),
}

impl Length {
    pub fn value(&self) -> u32 {
        match self {
            Length::Inches(inches) => (*inches * _EMUS_PER_INCH) as u32,
            Length::Cm(cm) => (*cm * _EMUS_PER_CM) as u32,
            Length::Emu(emu) => *emu as u32,
            Length::Mm(mm) => (*mm * _EMUS_PER_MM) as u32,
            Length::Pt(pt) => (*pt * _EMUS_PER_PT) as u32,
            Length::Twips(twips) => (*twips * _EMUS_PER_TWIP) as u32,
        }
    }
}
