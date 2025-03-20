use core::fmt;

#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SlangResult(u32);

impl SlangResult {
    #[inline(always)]
    pub(crate) fn succeeded(&self) -> bool {
        (self.0 as i32) >= 0
    }

    #[inline(always)]
    pub(crate) fn failed(&self) -> bool {
        !self.succeeded()
    }

    #[inline(always)]
    pub(crate) fn severity(&self) -> u32 {
        self.0 >> 31
    }

    #[inline(always)]
    pub(crate) fn facility(&self) -> u32 {
        (self.0 >> 16) & 0x7FFF
    }

    #[inline(always)]
    pub(crate) fn code(&self) -> u32 {
        self.0 & 0xFFFF
    }
}

impl fmt::Display for SlangResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.succeeded() {
            "Success"
        } else {
            "Failure"
        };

        write!(
            f,
            "{} (Raw: 0x{:08X}, Severity: {}, Facility: {}, Code: {})",
            status,
            self.0,
            self.severity(),
            self.facility(),
            self.code()
        )
    }
}
