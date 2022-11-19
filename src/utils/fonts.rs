use druid::{FontDescriptor, FontFamily, FontStyle, FontWeight};

const SZ_XS: f64 = 12.0;
const SZ_S: f64 = 16.0;
const SZ_M: f64 = 20.0;
const SZ_L: f64 = 24.0;
const SZ_XL: f64 = 28.0;

pub enum FontSize {
    XS,
    SM,
    MD,
    LG,
    XL,
}

impl From<FontSize> for f64 {
    fn from(sz: FontSize) -> Self {
        match sz {
            FontSize::XS => SZ_XS,
            FontSize::SM => SZ_S,
            FontSize::MD => SZ_M,
            FontSize::LG => SZ_L,
            FontSize::XL => SZ_XL,
        }
    }
}

pub struct Font {
    family: FontFamily,
    weight: FontWeight,
    style: FontStyle,
    sz: f64,
}

impl Font {
    pub fn get(self) -> FontDescriptor {
        FontDescriptor::new(self.family.clone())
            .with_weight(self.weight)
            .with_size(self.sz)
    }

    pub fn xs(mut self) -> Self {
        self.sz = SZ_XS;
        self
    }

    pub fn sm(mut self) -> Self {
        self.sz = SZ_S;
        self
    }

    pub fn md(mut self) -> Self {
        self.sz = SZ_M;
        self
    }

    pub fn lg(mut self) -> Self {
        self.sz = SZ_L;
        self
    }

    pub fn xl(mut self) -> Self {
        self.sz = SZ_XL;
        self
    }

    pub fn bold(mut self) -> Self {
        self.weight = FontWeight::SEMI_BOLD;
        self
    }

    pub fn normal(mut self) -> Self {
        self.weight = FontWeight::NORMAL;
        self
    }

    pub fn dejavu(mut self) -> Self {
        self.family = FontFamily::new_unchecked("DejaVu Sans");
        self
    }

    pub fn sf_apple(mut self) -> Self {
        self.family = FontFamily::new_unchecked("SF Pro Text");
        self
    }

    pub fn arial(mut self) -> Self {
        self.family = FontFamily::new_unchecked("Arial");
        self
    }

    pub fn italic(mut self) -> Self {
        self.style = FontStyle::Italic;
        self
    }
}

impl Default for Font {
    fn default() -> Self {
        Font {
            family: FontFamily::new_unchecked("DejaVu Sans"),
            weight: FontWeight::NORMAL,
            sz: SZ_M,
            style: FontStyle::Regular,
        }
    }
}
