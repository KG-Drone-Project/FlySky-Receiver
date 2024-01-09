#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
    r" Always include the device crate which contains the vector table"] use
    stm32f4xx_hal :: pac as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ;
    #[doc =
    r" Holds the maximum priority level for use by async HAL drivers."]
    #[no_mangle] static RTIC_ASYNC_MAX_LOGICAL_PRIO : u8 = 0u8 ; use hal ::
    {
        dma ::
        {
            config :: DmaConfig, DmaFlag, PeripheralToMemory, Stream2,
            StreamsTuple, Transfer
        }, pac :: { DMA2, USART1 }, prelude :: *, rcc :: RccExt, serial,
    } ; use rtt_target :: { rprintln, rtt_init_print } ; use stm32f4xx_hal as
    hal ; const BUFFER_SIZE : usize = 100 ; type RxTransfer = Transfer <
    Stream2 < DMA2 >, 4, serial :: Rx < USART1 >, PeripheralToMemory, &
    'static mut [u8 ; BUFFER_SIZE], > ; #[doc = r" User code end"] impl < 'a >
    __rtic_internal_initLocalResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_initLocalResources
            {
                rx_pool_memory : & mut *
                __rtic_internal_local_init_rx_pool_memory.get_mut(),
                __rtic_internal_marker : :: core :: marker :: PhantomData,
            }
        }
    } #[doc = r"Shared resources"] struct Shared
    { rx_transfer : RxTransfer, bytes : & [u8], } #[doc = r"Local resources"]
    struct Local { rx_buffer : Option < & 'static mut [u8 ; BUFFER_SIZE] >, }
    #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `init` has access to"] pub struct
    __rtic_internal_initLocalResources < 'a >
    {
        #[allow(missing_docs)] pub rx_pool_memory : & 'static mut [u8 ; 400],
        #[doc(hidden)] pub __rtic_internal_marker : :: core :: marker ::
        PhantomData < & 'a () >,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_init_Context <
    'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () >, #[doc = r" Core peripherals"] pub core : rtic :: export ::
        Peripherals, #[doc = r" Device peripherals (PAC)"] pub device :
        stm32f4xx_hal :: pac :: Peripherals,
        #[doc = r" Critical section token for init"] pub cs : rtic :: export
        :: CriticalSection < 'a >,
        #[doc = r" Local Resources this task has access to"] pub local : init
        :: LocalResources < 'a >,
    } impl < 'a > __rtic_internal_init_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn
        new(core : rtic :: export :: Peripherals,) -> Self
        {
            __rtic_internal_init_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, device :
                stm32f4xx_hal :: pac :: Peripherals :: steal(), cs : rtic ::
                export :: CriticalSection :: new(), core, local : init ::
                LocalResources :: new(),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Initialization function"] pub mod init
    {
        #[doc(inline)] pub use super :: __rtic_internal_initLocalResources as
        LocalResources ; #[doc(inline)] pub use super ::
        __rtic_internal_init_Context as Context ;
    } #[inline(always)] #[allow(non_snake_case)] fn init(cx : init :: Context)
    -> (Shared, Local)
    {
        rtt_init_print! () ; rprintln! ("Init") ; let dp : hal :: pac ::
        Peripherals = cx.device ; let rcc = dp.RCC.constrain() ; let clocks =
        rcc.cfgr.freeze() ; let gpioa = dp.GPIOA.split() ; let rx_pin =
        gpioa.pa10 ; let mut rx =
        dp.USART1.rx(rx_pin, serial :: Config ::
        default().baudrate(115200.bps()).dma(serial :: config :: DmaConfig ::
        Rx), & clocks,).unwrap() ; rx.listen_idle() ; let dma2 = StreamsTuple
        :: new(dp.DMA2) ; let rx_buffer1 = cortex_m :: singleton!
        (: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE]).unwrap() ; let rx_buffer2 =
        cortex_m :: singleton!
        (: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE]).unwrap() ; let mut
        rx_transfer = Transfer ::
        init_peripheral_to_memory(dma2.2, rx, rx_buffer1, None, DmaConfig ::
        default().memory_increment(true).fifo_enable(true).fifo_error_interrupt(true).transfer_complete_interrupt(true),)
        ; let bytes : & [u8] ; rx_transfer.start(| _rx | {}) ;
        (Shared { rx_transfer, bytes }, Local
        { rx_buffer : Some(rx_buffer2), },)
    } #[allow(non_snake_case)] #[no_mangle] unsafe fn USART1()
    {
        const PRIORITY : u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, || { usart1(usart1 :: Context :: new()) }) ;
    } impl < 'a > __rtic_internal_usart1LocalResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_usart1LocalResources
            {
                rx_buffer : & mut *
                (& mut *
                __rtic_internal_local_resource_rx_buffer.get_mut()).as_mut_ptr(),
                __rtic_internal_marker : :: core :: marker :: PhantomData,
            }
        }
    } impl < 'a > __rtic_internal_usart1SharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_usart1SharedResources
            {
                rx_transfer : & mut *
                (& mut *
                __rtic_internal_shared_resource_rx_transfer.get_mut()).as_mut_ptr(),
                bytes : shared_resources :: bytes_that_needs_to_be_locked ::
                new(), __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } #[allow(non_snake_case)] #[no_mangle] unsafe fn DMA2_STREAM2()
    {
        const PRIORITY : u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, || { dma2_stream2(dma2_stream2 :: Context :: new()) }) ;
    } impl < 'a > __rtic_internal_dma2_stream2SharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_dma2_stream2SharedResources
            {
                rx_transfer : & mut *
                (& mut *
                __rtic_internal_shared_resource_rx_transfer.get_mut()).as_mut_ptr(),
                __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `usart1` has access to"] pub struct
    __rtic_internal_usart1LocalResources < 'a >
    {
        #[allow(missing_docs)] pub rx_buffer : & 'a mut Option < & 'static mut
        [u8 ; BUFFER_SIZE] >, #[doc(hidden)] pub __rtic_internal_marker : ::
        core :: marker :: PhantomData < & 'a () >,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `usart1` has access to"] pub struct
    __rtic_internal_usart1SharedResources < 'a >
    {
        #[allow(missing_docs)] pub rx_transfer : & 'a mut RxTransfer,
        #[allow(missing_docs)] pub bytes : shared_resources ::
        bytes_that_needs_to_be_locked < 'a >, #[doc(hidden)] pub
        __rtic_internal_marker : core :: marker :: PhantomData < & 'a () >,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_usart1_Context <
    'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () >, #[doc = r" Local Resources this task has access to"] pub
        local : usart1 :: LocalResources < 'a >,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        usart1 :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_usart1_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_usart1_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, local :
                usart1 :: LocalResources :: new(), shared : usart1 ::
                SharedResources :: new(),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod usart1
    {
        #[doc(inline)] pub use super :: __rtic_internal_usart1LocalResources
        as LocalResources ; #[doc(inline)] pub use super ::
        __rtic_internal_usart1SharedResources as SharedResources ;
        #[doc(inline)] pub use super :: __rtic_internal_usart1_Context as
        Context ;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `dma2_stream2` has access to"] pub struct
    __rtic_internal_dma2_stream2SharedResources < 'a >
    {
        #[allow(missing_docs)] pub rx_transfer : & 'a mut RxTransfer,
        #[doc(hidden)] pub __rtic_internal_marker : core :: marker ::
        PhantomData < & 'a () >,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_dma2_stream2_Context < 'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () >, #[doc = r" Shared Resources this task has access to"] pub
        shared : dma2_stream2 :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_dma2_stream2_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_dma2_stream2_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, shared :
                dma2_stream2 :: SharedResources :: new(),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod dma2_stream2
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_dma2_stream2SharedResources as SharedResources ;
        #[doc(inline)] pub use super :: __rtic_internal_dma2_stream2_Context
        as Context ;
    } #[allow(non_snake_case)] fn usart1(mut cx : usart1 :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex :: prelude :: * ; let
        transfer = & mut cx.shared.rx_transfer ; if transfer.is_idle()
        {
            let bytes_count = BUFFER_SIZE - transfer.number_of_transfers() as
            usize ; let new_buffer = cx.local.rx_buffer.take().unwrap() ;
            let(buffer, _) = transfer.next_transfer(new_buffer).unwrap() ;
            cx.shared.bytes.lock(| bytes |
            { bytes = & buffer [.. bytes_count] ; }) ; * cx.local.rx_buffer =
            Some(buffer) ;
        }
    } #[allow(non_snake_case)] fn
    dma2_stream2(mut cx : dma2_stream2 :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex :: prelude :: * ; let
        transfer = & mut cx.shared.rx_transfer ; let flags = transfer.flags()
        ;
        transfer.clear_flags(DmaFlag :: FifoError | DmaFlag ::
        TransferComplete) ; if flags.is_transfer_complete() {}
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic0"] static
    __rtic_internal_shared_resource_rx_transfer : rtic :: RacyCell < core ::
    mem :: MaybeUninit < RxTransfer >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic1"] static
    __rtic_internal_shared_resource_bytes : rtic :: RacyCell < core :: mem ::
    MaybeUninit < & [u8] >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ; impl < 'a > rtic :: Mutex
    for shared_resources :: bytes_that_needs_to_be_locked < 'a >
    {
        type T = & [u8] ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut & [u8]) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_bytes.get_mut() as * mut
                _, CEILING, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } mod shared_resources
    {
        #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        bytes_that_needs_to_be_locked < 'a >
        { __rtic_internal_p : :: core :: marker :: PhantomData < & 'a () >, }
        impl < 'a > bytes_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new() -> Self
            {
                bytes_that_needs_to_be_locked
                { __rtic_internal_p : :: core :: marker :: PhantomData }
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic2"] static
    __rtic_internal_local_resource_rx_buffer : rtic :: RacyCell < core :: mem
    :: MaybeUninit < Option < & 'static mut [u8 ; BUFFER_SIZE] > >> = rtic ::
    RacyCell :: new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_local_init_rx_pool_memory : rtic ::
    RacyCell < [u8 ; 400] > = rtic :: RacyCell :: new([0 ; 400]) ;
    #[doc(hidden)] #[no_mangle] unsafe extern "C" fn main() ->!
    {
        rtic :: export :: assert_send :: < RxTransfer > () ; rtic :: export ::
        assert_send :: < & [u8] > () ; rtic :: export :: interrupt ::
        disable() ; let mut core : rtic :: export :: Peripherals = rtic ::
        export :: Peripherals :: steal().into() ; let _ =
        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
        interrupt :: TIM2 ; const _ : () =
        if(1 << stm32f4xx_hal :: pac :: NVIC_PRIO_BITS) < 1u8 as usize
        {
            :: core :: panic!
            ("Maximum priority used by interrupt vector 'USART1' is more than supported by hardware")
            ;
        } ;
        core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: USART1, rtic :: export ::
        cortex_logical2hw(1u8, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS),) ;
        rtic :: export :: NVIC ::
        unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: USART1) ; const _ : () =
        if(1 << stm32f4xx_hal :: pac :: NVIC_PRIO_BITS) < 1u8 as usize
        {
            :: core :: panic!
            ("Maximum priority used by interrupt vector 'DMA2_STREAM2' is more than supported by hardware")
            ;
        } ;
        core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: DMA2_STREAM2, rtic :: export ::
        cortex_logical2hw(1u8, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS),) ;
        rtic :: export :: NVIC ::
        unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: DMA2_STREAM2) ; #[inline(never)] fn
        __rtic_init_resources < F > (f : F) where F : FnOnce() { f() ; }
        __rtic_init_resources(||
        {
            let(shared_resources, local_resources) =
            init(init :: Context :: new(core.into())) ;
            __rtic_internal_shared_resource_rx_transfer.get_mut().write(core
            :: mem :: MaybeUninit :: new(shared_resources.rx_transfer)) ;
            __rtic_internal_shared_resource_bytes.get_mut().write(core :: mem
            :: MaybeUninit :: new(shared_resources.bytes)) ;
            __rtic_internal_local_resource_rx_buffer.get_mut().write(core ::
            mem :: MaybeUninit :: new(local_resources.rx_buffer)) ; rtic ::
            export :: interrupt :: enable() ;
        }) ; loop { rtic :: export :: nop() }
    }
}