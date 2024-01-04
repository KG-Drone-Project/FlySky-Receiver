#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
    r" Always include the device crate which contains the vector table"] use
    stm32f4xx_hal :: pac as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ;
    #[doc =
    r" Holds the maximum priority level for use by async HAL drivers."]
    #[no_mangle] static RTIC_ASYNC_MAX_LOGICAL_PRIO : u8 = 0u8 ; use cortex_m
    :: { asm, peripheral } ; use cortex_m_rt :: entry ; use core :: cell ::
    { Cell, RefCell } ; use stm32f4xx_hal ::
    { pac :: { self, TIM4 }, prelude :: *, timer :: { Timer, PwmInput } } ;
    use rtt_target :: { rprintln, rtt_init_print } ;
    #[doc = r" User code end"] #[doc = r"Shared resources"] struct Shared {}
    #[doc = r"Local resources"] struct Local { monitor : PwmInput < TIM4 >, }
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_init_Context <
    'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () >, #[doc = r" Core peripherals"] pub core : rtic :: export ::
        Peripherals, #[doc = r" Device peripherals (PAC)"] pub device :
        stm32f4xx_hal :: pac :: Peripherals,
        #[doc = r" Critical section token for init"] pub cs : rtic :: export
        :: CriticalSection < 'a >,
    } impl < 'a > __rtic_internal_init_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn
        new(core : rtic :: export :: Peripherals,) -> Self
        {
            __rtic_internal_init_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, device :
                stm32f4xx_hal :: pac :: Peripherals :: steal(), cs : rtic ::
                export :: CriticalSection :: new(), core,
            }
        }
    } #[allow(non_snake_case)] #[doc = "Initialization function"] pub mod init
    {
        #[doc(inline)] pub use super :: __rtic_internal_init_Context as
        Context ;
    } #[inline(always)] #[allow(non_snake_case)] fn
    init(ctx : init :: Context) -> (Shared, Local)
    {
        rtt_init_print! () ; rprintln! ("Init") ; let dp = ctx.device ; let
        rcc = dp.RCC.constrain() ; let clocks = rcc.cfgr.freeze() ; let gpioa
        = dp.GPIOA.split() ; let gpiod = dp.GPIOD.split() ; let pwm_reader_ch1
        = gpiod.pd12 ; let pwm_reader_ch2 = gpiod.pd13 ; let monitor = Timer
        :: new(dp.TIM4, & clocks).pwm_input(50.Hz(), pwm_reader_ch1) ;
        (Shared {}, Local { monitor })
    } #[allow(non_snake_case)] #[no_mangle] unsafe fn TIM4()
    {
        const PRIORITY : u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, ||
        { flysky_receiver(flysky_receiver :: Context :: new()) }) ;
    } impl < 'a > __rtic_internal_flysky_receiverLocalResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_flysky_receiverLocalResources
            {
                monitor : & mut *
                (& mut *
                __rtic_internal_local_resource_monitor.get_mut()).as_mut_ptr(),
                __rtic_internal_marker : :: core :: marker :: PhantomData,
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `flysky_receiver` has access to"] pub struct
    __rtic_internal_flysky_receiverLocalResources < 'a >
    {
        #[allow(missing_docs)] pub monitor : & 'a mut PwmInput < TIM4 >,
        #[doc(hidden)] pub __rtic_internal_marker : :: core :: marker ::
        PhantomData < & 'a () >,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_flysky_receiver_Context < 'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () >, #[doc = r" Local Resources this task has access to"] pub
        local : flysky_receiver :: LocalResources < 'a >,
    } impl < 'a > __rtic_internal_flysky_receiver_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_flysky_receiver_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, local :
                flysky_receiver :: LocalResources :: new(),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod
    flysky_receiver
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_flysky_receiverLocalResources as LocalResources ;
        #[doc(inline)] pub use super ::
        __rtic_internal_flysky_receiver_Context as Context ;
    } #[allow(non_snake_case)] fn
    flysky_receiver(ctx : flysky_receiver :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex :: prelude :: * ; let
        duty_cycle = ctx.local.monitor.get_duty_cycle() ; rprintln!
        ("FlySky duty cycle: {:?}", duty_cycle) ;
        ctx.local.monitor.clear_all_flags() ;
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic0"] static
    __rtic_internal_local_resource_monitor : rtic :: RacyCell < core :: mem ::
    MaybeUninit < PwmInput < TIM4 > >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ; #[doc(hidden)] #[no_mangle]
    unsafe extern "C" fn main() ->!
    {
        rtic :: export :: interrupt :: disable() ; let mut core : rtic ::
        export :: Peripherals = rtic :: export :: Peripherals ::
        steal().into() ; const _ : () =
        if(1 << stm32f4xx_hal :: pac :: NVIC_PRIO_BITS) < 1u8 as usize
        {
            :: core :: panic!
            ("Maximum priority used by interrupt vector 'TIM4' is more than supported by hardware")
            ;
        } ;
        core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: TIM4, rtic :: export ::
        cortex_logical2hw(1u8, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS),) ;
        rtic :: export :: NVIC ::
        unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: TIM4) ; #[inline(never)] fn __rtic_init_resources < F
        > (f : F) where F : FnOnce() { f() ; }
        __rtic_init_resources(||
        {
            let(shared_resources, local_resources) =
            init(init :: Context :: new(core.into())) ;
            __rtic_internal_local_resource_monitor.get_mut().write(core :: mem
            :: MaybeUninit :: new(local_resources.monitor)) ; rtic :: export
            :: interrupt :: enable() ;
        }) ; loop { rtic :: export :: nop() }
    }
}