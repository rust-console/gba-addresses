//! IO Control Registers.
//!
//! This region of memory allows you to control the various parts of the GBA.
//!
//! Each register has its own unique purpose.
//! * They are often 16-bits, but not always.
//! * They are often write-only, though some are read/write, and some are
//!   read-only.
//! * A the timer addresses do different things between reading and writing.
//!
//! Most names here are the same as in [GBATEK][1] with `_ADDR` on the end. Note
//! that some documents do sometimes use alternate names for some of the IO
//! registers (particularly the sound registers).
//!
//! [1]: https://problemkaputt.de/gbatek.htm#gbaiomap
//!
//! * **Wait states:** 0
//! * **Bus Size:** 32-bit
//! * **Read/Write:** 8/16/32

/// Display Control
///
/// * **Access:** read/write
/// * **Size:** 2
pub const DISPCNT_ADDR: usize = 0x0400_0000;

/// Display Status
///
/// * **Access:** read/write
/// * **Size:** 2
pub const DISPSTAT_ADDR: usize = 0x0400_0004;

/// Vertical Counter
///
/// * **Access:** read-only
/// * **Size:** 1
pub const VCOUNT_ADDR: usize = 0x0400_0006;

// // // // //
// BG Control
// // // // //

/// BG0 Control (video modes 0 or 1)
///
/// * **Access:** read/write
/// * **Size:** 2
pub const BG0CNT_ADDR: usize = 0x0400_0008;

/// BG1 Control (video modes 0 or 1)
///
/// * **Access:** read/write
/// * **Size:** 2
pub const BG1CNT_ADDR: usize = 0x0400_000A;

/// BG2 Control (video modes 0, 1, or 2)
///
/// * **Access:** read/write
/// * **Size:** 2
pub const BG2CNT_ADDR: usize = 0x0400_000C;

/// BG3 Control (video modes 0 or 2)
///
/// * **Access:** read/write
/// * **Size:** 2
pub const BG3CNT_ADDR: usize = 0x0400_000E;

// // // // //
// Text Offsets
// // // // //

/// BG0 Horizontal Offset (video modes 0 or 1)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG0HOFS_ADDR: usize = 0x0400_0010;

/// BG0 Vertical Offset (video modes 0 or 1)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG0VOFS_ADDR: usize = 0x0400_0012;

/// BG1 Horizontal Offset (video modes 0 or 1)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG1HOFS_ADDR: usize = 0x0400_0014;

/// BG1 Vertical Offset (video modes 0 or 1)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG1VOFS_ADDR: usize = 0x0400_0016;

/// BG2 Horizontal Offset (video mode 0)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG2HOFS_ADDR: usize = 0x0400_0018;

/// BG1 Vertical Offset (video mode 0)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG2VOFS_ADDR: usize = 0x0400_001A;

/// BG3 Horizontal Offset (video mode 0)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG3HOFS_ADDR: usize = 0x0400_001C;

/// BG3 Vertical Offset (video mode 0)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG3VOFS_ADDR: usize = 0x0400_001E;

// // // // //
// Affine Parameters
// // // // //

/// BG2 Affine Param A (video mode 1 and 2)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG2PA_ADDR: usize = 0x0400_0020;

/// BG2 Affine Param B (video mode 1 and 2)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG2PB_ADDR: usize = 0x0400_0022;

/// BG2 Affine Param C (video mode 1 and 2)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG2PC_ADDR: usize = 0x0400_0024;

/// BG2 Affine Param D (video mode 1 and 2)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG2PD_ADDR: usize = 0x0400_0026;

/// BG2 X reference point (video mode 1, 2, 3, 4, and 5)
///
/// * **Access:** write-only
/// * **Size:** 4
pub const BG2X_ADDR: usize = 0x0400_0028;

/// BG2 Y reference point (video mode 1, 2, 3, 4, and 5)
///
/// * **Access:** write-only
/// * **Size:** 4
pub const BG2Y_ADDR: usize = 0x0400_002C;

/// BG3 Affine Param A (video mode 2)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG3PA_ADDR: usize = 0x0400_0030;

/// BG3 Affine Param B (video mode 2)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG3PB_ADDR: usize = 0x0400_0032;

/// BG3 Affine Param C (video mode 2)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG3PC_ADDR: usize = 0x0400_0034;

/// BG3 Affine Param D (video mode 2)
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BG3PD_ADDR: usize = 0x0400_0036;

/// BG3 X reference point (video mode 2)
///
/// * **Access:** write-only
/// * **Size:** 4
pub const BG3X_ADDR: usize = 0x0400_0038;

/// BG3 Y reference point (video mode 2)
///
/// * **Access:** write-only
/// * **Size:** 4
pub const BG3Y_ADDR: usize = 0x0400_003C;

// // // // //
// Window
// // // // //

/// Window 0 horizontal.
///
/// * **Access:** write-only
/// * **Size:** 2
pub const WIN0H_ADDR: usize = 0x0400_0040;

/// Window 1 horizontal.
///
/// * **Access:** write-only
/// * **Size:** 2
pub const WIN1H_ADDR: usize = 0x0400_0042;

/// Window 0 vertical.
///
/// * **Access:** write-only
/// * **Size:** 2
pub const WIN0V_ADDR: usize = 0x0400_0044;

/// Window 0 vertical.
///
/// * **Access:** write-only
/// * **Size:** 2
pub const WIN1V_ADDR: usize = 0x0400_0046;

/// Window 0 and 1 insides.
///
/// * **Access:** read/write
/// * **Size:** 2
pub const WININ_ADDR: usize = 0x0400_0048;

/// Object window insides and outside of all windows.
///
/// * **Access:** read/write
/// * **Size:** 2
pub const WINOUT_ADDR: usize = 0x0400_004A;

// // // // //
// Special Effects
// // // // //

/// Mosaic effect size.
///
/// * **Access:** write-only
/// * **Size:** 2
pub const MOSAIC_ADDR: usize = 0x0400_004C;

/// Color blend special effect.
///
/// * **Access:** read/write
/// * **Size:** 2
pub const BLDCNT_ADDR: usize = 0x0400_0050;

/// Alpha blend coefficients.
///
/// * **Access:** read/write
/// * **Size:** 2
pub const BLDALPHA_ADDR: usize = 0x0400_0052;

/// Brightness blend coefficients.
///
/// * **Access:** write-only
/// * **Size:** 2
pub const BLDY_ADDR: usize = 0x0400_0054;

// // // // //
// Sound
// // // // //

/// Channel 1 Sweep register.
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SOUND1CNT_L_ADDR: usize = 0x0400_0060;

/// Channel 1 duty / len / envelope
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SOUND1CNT_H_ADDR: usize = 0x0400_0062;

/// Channel 1 frequency / control
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SOUND1CNT_X_ADDR: usize = 0x0400_0064;

/// Channel 2 duty / len / envelope
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SOUND2CNT_L_ADDR: usize = 0x0400_0068;

/// Channel 2 frequency / control
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SOUND2CNT_H_ADDR: usize = 0x0400_006C;

/// Channel 3 stop / wave RAM select
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SOUND3CNT_L_ADDR: usize = 0x0400_0070;

/// Channel 3 len / volume
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SOUND3CNT_H_ADDR: usize = 0x0400_0072;

/// Channel 3 frequency / control
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SOUND3CNT_X_ADDR: usize = 0x0400_0074;

/// Channel 4 len / envelope
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SOUND4CNT_L_ADDR: usize = 0x0400_0078;

/// Channel 4 frequency / control
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SOUND4CNT_H_ADDR: usize = 0x0400_007C;

/// Sound Control left/right volume/enable
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SOUNDCNT_L_ADDR: usize = 0x0400_0080;

/// DMA Sound Control / Mixing
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SOUNDCNT_H_ADDR: usize = 0x0400_0082;

/// Sound on/off
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SOUNDCNT_X_ADDR: usize = 0x0400_0084;

/// Wave RAM data.
///
/// This is actually two banks. While you can access one bank here, the other
/// bank is the sound playing.
///
/// When the sound is played, the entire 32-bit region is rotated, 4 bits at a
/// time, and then the lowest 4 bits are played as that sample.
///
/// * **Access:** read/write
/// * **Size:** 32 (used as 4-bit samples)
pub const WAVE_RAM_BASE_ADDR: usize = 0x0400_0090;

/// FIFO sound target for sound using DMA 1.
///
/// * **Access:** write-only
/// * **Size:** 4
pub const FIFO_A_ADDR: usize = 0x0400_00A0;

/// FIFO sound target for sound using DMA 2.
///
/// * **Access:** write-only
/// * **Size:** 4
pub const FIFO_B_ADDR: usize = 0x0400_00A4;

// // // // //
// DMA
// // // // //

/// DMA 0 Source Address
///
/// * **Access:** write-only
/// * **Size:** 4
pub const DMA0SAD_ADDR: usize = 0x0400_00B0;

/// DMA 0 Destination Address
///
/// * **Access:** write-only
/// * **Size:** 4
pub const DMA0DAD_ADDR: usize = 0x0400_00B4;

/// DMA 0 transfer count
///
/// * **Access:** write-only
/// * **Size:** 2
pub const DMA0CNT_L_ADDR: usize = 0x0400_00B8;

/// DMA 0 control bits
///
/// * **Access:** read/write
/// * **Size:** 2
pub const DMA0CNT_H_ADDR: usize = 0x0400_00BA;

/// DMA 1 Source Address
///
/// * **Access:** write-only
/// * **Size:** 4
pub const DMA1SAD_ADDR: usize = 0x0400_00BC;

/// DMA 1 Destination Address
///
/// * **Access:** write-only
/// * **Size:** 4
pub const DMA1DAD_ADDR: usize = 0x0400_00C0;

/// DMA 1 transfer count
///
/// * **Access:** write-only
/// * **Size:** 2
pub const DMA1CNT_L_ADDR: usize = 0x0400_00C4;

/// DMA 1 control bits
///
/// * **Access:** read/write
/// * **Size:** 2
pub const DMA1CNT_H_ADDR: usize = 0x0400_00C6;

/// DMA 2 Source Address
///
/// * **Access:** write-only
/// * **Size:** 4
pub const DMA2SAD_ADDR: usize = 0x0400_00C8;

/// DMA 2 Destination Address
///
/// * **Access:** write-only
/// * **Size:** 4
pub const DMA2DAD_ADDR: usize = 0x0400_00CC;

/// DMA 2 transfer count
///
/// * **Access:** write-only
/// * **Size:** 2
pub const DMA2CNT_L_ADDR: usize = 0x0400_00D0;

/// DMA 2 control bits
///
/// * **Access:** read/write
/// * **Size:** 2
pub const DMA2CNT_H_ADDR: usize = 0x0400_00D2;

/// DMA 3 Source Address
///
/// * **Access:** write-only
/// * **Size:** 4
pub const DMA3SAD_ADDR: usize = 0x0400_00D4;

/// DMA 3 Destination Address
///
/// * **Access:** write-only
/// * **Size:** 4
pub const DMA3DAD_ADDR: usize = 0x0400_00D8;

/// DMA 3 transfer count
///
/// * **Access:** write-only
/// * **Size:** 2
pub const DMA3CNT_L_ADDR: usize = 0x0400_00DC;

/// DMA 3 control bits
///
/// * **Access:** read/write
/// * **Size:** 2
pub const DMA3CNT_H_ADDR: usize = 0x0400_00DE;

// // // // //
// Timers
// // // // //

/// Timer 0 counter / reload
///
/// * **Access:** read/write
/// * **Size:** 2
pub const TM0CNT_L_ADDR: usize = 0x0400_0100;

/// Timer 0 control bits
///
/// * **Access:** read/write
/// * **Size:** 2
pub const TM0CNT_H_ADDR: usize = 0x0400_0102;

/// Timer 1 counter / reload
///
/// * **Access:** read/write
/// * **Size:** 2
pub const TM1CNT_L_ADDR: usize = 0x0400_0104;

/// Timer 1 control bits
///
/// * **Access:** read/write
/// * **Size:** 2
pub const TM1CNT_H_ADDR: usize = 0x0400_0106;

/// Timer 2 counter / reload
///
/// * **Access:** read/write
/// * **Size:** 2
pub const TM2CNT_L_ADDR: usize = 0x0400_0108;

/// Timer 2 control bits
///
/// * **Access:** read/write
/// * **Size:** 2
pub const TM2CNT_H_ADDR: usize = 0x0400_010A;

/// Timer 3 counter / reload
///
/// * **Access:** read/write
/// * **Size:** 2
pub const TM3CNT_L_ADDR: usize = 0x0400_010C;

/// Timer 3 control bits
///
/// * **Access:** read/write
/// * **Size:** 2
pub const TM3CNT_H_ADDR: usize = 0x0400_010E;

// // // // //
// Serial 1
// // // // //

/// ?
///
/// * **Access:** read/write
/// * **Size:** 4
pub const SIODATA32_ADDR: usize = 0x0400_0120;

/// ?
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SIOMULTI0_ADDR: usize = 0x0400_0120;

/// ?
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SIOMULTI1_ADDR: usize = 0x0400_0122;

/// ?
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SIOMULTI2_ADDR: usize = 0x0400_0124;

/// ?
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SIOMULTI3_ADDR: usize = 0x0400_0126;

/// ?
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SIOCNT_ADDR: usize = 0x0400_0128;

/// ?
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SIOMLT_SEND_ADDR: usize = 0x0400_012A;

/// ?
///
/// * **Access:** read/write
/// * **Size:** 2
pub const SIODATA8_ADDR: usize = 0x0400_012A;

// // // // //
// Keypad
// // // // //

/// Keys pressed
///
/// * 0: A
/// * 1: B
/// * 2: Select
/// * 3: Start
/// * 4: Right
/// * 5: Left
/// * 6: Up
/// * 7: Down
/// * 8: R
/// * 9: L
///
/// Bits use a "low active" convention: 0 when pressed, 1 when released.
///
/// * **Access:** read-only
/// * **Size:** 2
pub const KEYINPUT_ADDR: usize = 0x0400_0130;

/// Keypad interrupt control
///
/// Uses the same bit ordering as the [`KEYINPUT_ADDR`](KEYINPUT_ADDR).
///
/// * **Access:** read/write
/// * **Size:** 2
pub const KEYCNT_ADDR: usize = 0x0400_0132;

// // // // //
// Serial 2
// // // // //

/// ?
///
/// * **Access:** read/write
/// * **Size:** 2
pub const RCNT_ADDR: usize = 0x0400_0134;

/// ?
///
/// * **Access:** read/write
/// * **Size:** 2
pub const JOYCNT_ADDR: usize = 0x0400_0140;

/// ?
///
/// * **Access:** read/write
/// * **Size:** 4
pub const JOY_RECV_ADDR: usize = 0x0400_0150;

/// ?
///
/// * **Access:** read/write
/// * **Size:** 4
pub const JOY_TRANS_ADDR: usize = 0x0400_0154;

/// ?
///
/// * **Access:** read/write
/// * **Size:** 2
pub const JOYSTAT_ADDR: usize = 0x0400_0158;

// // // // //
// Interrupt / Waitstate
// // // // //

/// Interrupt Enable
///
/// * 0: Vertical blank
/// * 1: Horizontal blank
/// * 2: `VCOUNT` matches `DISPCNT` value.
/// * 3: Timer 0 Overflow
/// * 4: Timer 1 Overflow
/// * 5: Timer 2 Overflow
/// * 6: Timer 3 Overflow
/// * 7: Serial Communication
/// * 8: DMA 0
/// * 9: DMA 1
/// * 10: DMA 2
/// * 11: DMA 3
/// * 12: Keypad
///
/// Controls when the system will *accept* an interrupt from each source. If an
/// interrupt type isn't set here, then interrupts from that source will be
/// ignored.
///
/// Note that most sources actually *send* an interrupt or not based on their
/// own control register values elsewhere.
///
/// * **Access:** read/write
/// * **Size:** 2
pub const IE_ADDR: usize = 0x0400_0200;

/// Interrupt Flags
///
/// These bits are set during an interrupt. To acknowledge an interrupt during
/// the interrupt handler, write a "1" to one or more bits within this register.
///
/// Note that if you're acknowledging an interrupt within
/// `IntrWait`/`VBlankIntrWait` then you must also BIT_OR your changes to the
/// `IF` address to the
/// [`IRQ_INTR_WAIT_CHECK_FLAG_ADDR`](IRQ_INTR_WAIT_CHECK_FLAG_ADDR).
///
/// * **Access:** read/write
/// * **Size:** 2
pub const IF_ADDR: usize = 0x0400_0202;

/// Waitstate Control
///
/// * **Access:** read/write
/// * **Size:** 2
pub const WAITCNT_ADDR: usize = 0x0400_0204;

/// Interrupt Master Enable
///
/// Bit 0 controls if interrupts can happen, all other bits are ignored.
///
/// Note that calling `IntrWait` or `VBlankIntrWait` will force bit 0 on.
///
/// * **Access:** read/write
/// * **Size:** 2
pub const IME_ADDR: usize = 0x0400_0208;
