use memory_addr::PhysAddr;
use tock_registers::{
    registers::{ReadOnly, ReadWrite, WriteOnly},
    register_bitfields, register_structs,
};
use tock_registers::interfaces::Writeable;
use tock_registers::interfaces::Readable;
use crate::platform::aarch64_common::psci::system_off as terminate;

use crate::mem::phys_to_virt;

const GPIO_BASE: PhysAddr = pa!(axconfig::devices::GPIO_PADDR);
pub const PL061GPIOREGS: *mut Pl061GpioRegs = phys_to_virt(GPIO_BASE).as_usize() as *mut Pl061GpioRegs;

register_bitfields! [
    u32,

    DIR [
        Pin0 OFFSET(0) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
        Pin7 OFFSET(7) NUMBITS(1) [
            Input = 0,
            Output = 1,
        ],
    ],

    IS [
        Pin0 OFFSET(0) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
        Pin7 OFFSET(7) NUMBITS(1) [
            Edge = 0,
            Level = 1,
        ],
    ],

    IBE [
        Pin0 OFFSET(0) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
        Pin7 OFFSET(7) NUMBITS(1) [
            SingleEdge = 0,
            BothEdges = 1,
        ],
    ],

    IEV [
        Pin0 OFFSET(0) NUMBITS(1) [
            FallingLow = 0,   // 下降沿或低电平
            RisingHigh = 1,   // 上升沿或高电平
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            FallingLow = 0,
            RisingHigh = 1,
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            FallingLow = 0,
            RisingHigh = 1,
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            FallingLow = 0,
            RisingHigh = 1,
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            FallingLow = 0,
            RisingHigh = 1,
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            FallingLow = 0,
            RisingHigh = 1,
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            FallingLow = 0,
            RisingHigh = 1,
        ],
        Pin7 OFFSET(7) NUMBITS(1) [
            FallingLow = 0,
            RisingHigh = 1,
        ],
    ],

    IE [
        Pin0 OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        Pin7 OFFSET(7) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
    ],

    RIS [
        Pin0 OFFSET(0) NUMBITS(1) [],
        Pin1 OFFSET(1) NUMBITS(1) [],
        Pin2 OFFSET(2) NUMBITS(1) [],
        Pin3 OFFSET(3) NUMBITS(1) [],
        Pin4 OFFSET(4) NUMBITS(1) [],
        Pin5 OFFSET(5) NUMBITS(1) [],
        Pin6 OFFSET(6) NUMBITS(1) [],
        Pin7 OFFSET(7) NUMBITS(1) [],
    ],

    MIS [
        Pin0 OFFSET(0) NUMBITS(1) [],
        Pin1 OFFSET(1) NUMBITS(1) [],
        Pin2 OFFSET(2) NUMBITS(1) [],
        Pin3 OFFSET(3) NUMBITS(1) [],
        Pin4 OFFSET(4) NUMBITS(1) [],
        Pin5 OFFSET(5) NUMBITS(1) [],
        Pin6 OFFSET(6) NUMBITS(1) [],
        Pin7 OFFSET(7) NUMBITS(1) [],
    ],

    IC [
        Pin0 OFFSET(0) NUMBITS(1) [],
        Pin1 OFFSET(1) NUMBITS(1) [],
        Pin2 OFFSET(2) NUMBITS(1) [],
        Pin3 OFFSET(3) NUMBITS(1) [],
        Pin4 OFFSET(4) NUMBITS(1) [],
        Pin5 OFFSET(5) NUMBITS(1) [],
        Pin6 OFFSET(6) NUMBITS(1) [],
        Pin7 OFFSET(7) NUMBITS(1) [],
    ],

    AFSEL [
        Pin0 OFFSET(0) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
        Pin7 OFFSET(7) NUMBITS(1) [
            GPIO = 0,
            Alternate = 1,
        ],
    ],
];


register_structs! {
    pub Pl061GpioRegs {
        (0x000 => data: [ReadWrite<u32>; 256]),
        (0x400 => dir: ReadWrite<u32, DIR::Register>),
        (0x404 => is: ReadWrite<u32, IS::Register>),
        (0x408 => ibe: ReadWrite<u32, IBE::Register>),
        (0x40c => iev: ReadWrite<u32, IEV::Register>),
        (0x410 => ie: ReadWrite<u32, IE::Register>),
        (0x414 => ris: ReadOnly<u32, RIS::Register>),
        (0x418 => mis: ReadOnly<u32, MIS::Register>),
        (0x41c => ic: WriteOnly<u32, IC::Register>),
        (0x420 => afsel: ReadWrite<u32, AFSEL::Register>),
        (0x424 => @END),
    }
}

pub fn init() {
    info!("PL061 GPIO base address: {:#x}", phys_to_virt(GPIO_BASE));
    #[cfg(feature = "irq")]
    crate::irq::set_enable(39, true);
    crate::irq::register_handler(39, handle);

    info!("Initializing PL061 GPIO...");
    let gpio_regs = unsafe { &mut *(PL061GPIOREGS) };
    gpio_regs.ie.write(IE::Pin3::SET);

    let status = gpio_regs.ie.get();
    info!("PL061 GPIO interrupt status: {:#x}", status);
}

fn handle() {
    let gpio_regs = unsafe { &mut *(PL061GPIOREGS) };

    // Read the interrupt status
    let status = gpio_regs.ie.get();

    info!("PL061 GPIO interrupt status: {:#x}", status);
    
    // Clear the interrupt
    gpio_regs.ic.set(status);

    if status == 0 {
        return; // No pending interrupts
    }

    terminate();
}
