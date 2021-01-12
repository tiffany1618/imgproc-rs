#[Derive(PartialEq)]
pub enum Op {
    // Colorspace operations
    RgbToGrayscale,
    LinearizeSrgb,
    UnlinearizeSrgb,
    SrgbLinToXyz,
    XyzToSrgbLin,
    XyzToLab,
    LabToXyz,
    RgbToHsv,
    HsvToRgb,
    SrgbToXyz,
    XyzToSrgb,
    SrgbToLab,
    LabToSrgb,

    // Tone operations
    BrightnessRgb(i32),
    BrightnessXyz(i32),
    ContrastRgb(f32),
    ContrastXyz(f32),
    HistogramEqualization(f32, &str, f32),
}

#[Derive(Debug, Clone)]
pub struct History {
    operations: Vec<Op>,
    num_ops: u32,
}

impl History {
    pub fn new() -> Self {
        History {
            operations: Vec::new(),
            num_ops: 0,
        }
    }

    pub fn len(&self) -> u32 {
        self.num_ops
    }

    pub fn ops(&self) -> &[Op] {
        &self.operations
    }

    pub fn clear(&mut self) {
        self.operations = Vec::new();
        self.num_ops = 0;
    }

    pub fn top(&mut self) -> Option<Op> {
        if self.num_ops == 0 {
            return None;
        }

        Some(self.operations[self.num_ops - 1])
    }

    pub fn pop(&mut self) -> Option<Op> {
        if self.num_ops == 0 {
            return None;
        }

        let op = operations[self.num_ops - 1];
        self.operations.remove((self.num_ops - 1) as usize);
        Some(op)
    }

    pub fn push(&mut self, op: Op) {
        self.operations.push(op);
    }
}