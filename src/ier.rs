use crate::{Register, Uart16550IO, IER};

impl<R: Register> IER<R> {
    /// 写入中断使能设置。
    #[inline]
    pub fn write(&self, io_region: &dyn Uart16550IO<R>, val: InterruptTypes) {
        io_region.write_at(self.offset, R::from(val.0));
    }

    /// 读取中断使能设置。
    #[inline]
    pub fn read(&self, io_region: &dyn Uart16550IO<R>) -> InterruptTypes {
        let val = io_region.read_at(self.offset).val();
        InterruptTypes(val)
    }

    /// 写入波特率分频器。
    pub fn write_divisor(&self, io_region: &dyn Uart16550IO<R>, val: R) {
        io_region.write_at(self.offset, val);
    }
}

/// 中断使能设置。
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct InterruptTypes(u8);

impl InterruptTypes {
    /// 关闭所有中断。
    pub const ZERO: Self = Self(0);

    const RDA: u8 = 1 << 0;
    const THRE: u8 = 1 << 1;
    const RLS: u8 = 1 << 2;
    const MS: u8 = 1 << 3;

    /// 使能接收数据中断。
    #[inline]
    pub const fn enable_rda(self) -> Self {
        Self(self.0 | Self::RDA)
    }

    /// 禁用接收数据中断。
    #[inline]
    pub const fn disable_rda(self) -> Self {
        Self(self.0 & !Self::RDA)
    }

    /// 接收数据中断。
    #[inline]
    pub const fn rda_enabled(self) -> bool {
        self.0 & Self::RDA == Self::RDA
    }

    /// 使能发送数据中断。
    #[inline]
    pub const fn enable_thre(self) -> Self {
        Self(self.0 | Self::THRE)
    }

    /// 禁用发送数据中断。
    #[inline]
    pub const fn disable_thre(self) -> Self {
        Self(self.0 & !Self::THRE)
    }

    /// 发送寄存器空中断。
    #[inline]
    pub const fn thre_enabled(self) -> bool {
        self.0 & Self::THRE == Self::THRE
    }

    /// 使能接收线状态中断。
    #[inline]
    pub const fn enable_rls(self) -> Self {
        Self(self.0 | Self::RLS)
    }

    /// 禁用接收线状态中断。
    #[inline]
    pub const fn disable_rls(self) -> Self {
        Self(self.0 & !Self::RLS)
    }

    /// 接收线状态中断。
    #[inline]
    pub const fn rls_enabled(self) -> bool {
        self.0 & Self::RLS == Self::RLS
    }

    /// 使能调制解调器中断。
    #[inline]
    pub const fn enable_ms(self) -> Self {
        Self(self.0 | Self::MS)
    }

    /// 禁用调制解调器中断。
    #[inline]
    pub const fn disable_ms(self) -> Self {
        Self(self.0 & !Self::MS)
    }

    /// 调制解调器中断。
    #[inline]
    pub const fn ms_enabled(self) -> bool {
        self.0 & Self::MS == Self::MS
    }
}
