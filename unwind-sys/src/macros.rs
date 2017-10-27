#[allow(unused_macros)]
macro_rules! make_api {
    () => {
        #[repr(C)]
        pub struct unw_cursor_t {
            pub opaque: [unw_word_t; UNW_TDEP_CURSOR_LEN as usize],
        }

        pub type unw_context_t = unw_tdep_context_t;

        #[repr(C)]
        pub struct unw_proc_info_t {
            pub start_ip: unw_word_t,
            pub end_ip: unw_word_t,
            pub lsda: unw_word_t,
            pub handler: unw_word_t,
            pub gp: unw_word_t,
            pub flags: unw_word_t,
            pub format: c_int,
            pub unwind_info_size: c_int,
            pub unwind_info: *mut c_void,
            pub extra: unw_tdep_proc_info_t,
        }

        #[repr(C)]
        pub struct unw_accessors_t {
            pub find_proc_info: Option<
                unsafe extern "C" fn(
                    asp: unw_addr_space_t,
                    ip: unw_word_t,
                    pip: *mut unw_proc_info_t,
                    need_unwind_info: c_int,
                    arg: *mut c_void,
                ) -> c_int,
            >,
            pub put_unwind_info: Option<
                unsafe extern "C" fn(
                    asp: unw_addr_space_t,
                    pip: *mut unw_proc_info_t,
                    arg: *mut c_void,
                ) -> c_void
            >,
            pub get_dyn_info_list_addr: Option<
                unsafe extern "C" fn(
                    asp: unw_addr_space_t,
                    dilap: *mut unw_word_t,
                    arg: *mut c_void,
                ) -> c_int,
            >,
            pub access_mem: Option<
                unsafe extern "C" fn(
                    asp: unw_addr_space_t,
                    addr: unw_word_t,
                    valp: *mut unw_word_t,
                    write: c_int,
                    arg: *mut c_void,
                ) -> c_int,
            >,
            pub access_reg: Option<
                unsafe extern "C" fn(
                    asp: unw_addr_space_t,
                    regnum: unw_regnum_t,
                    valp: *mut unw_word_t,
                    write: c_int,
                    arg: *mut c_void,
                ) -> c_int,
            >,
            // unw_fpreg_t is a long double :(
            access_fpreg: Option<unsafe extern "C" fn()>,
            pub resume: Option<
                unsafe extern "C" fn(
                    asp: unw_addr_space_t,
                    cp: *mut unw_cursor_t,
                    arg: *mut c_void,
                ) -> c_int,
            >,
            pub get_proc_name: Option<
                unsafe extern "C" fn(
                    asp: unw_addr_space_t,
                    addr: unw_word_t,
                    bufp: *mut c_char,
                    buf_len: size_t,
                    offp: *mut unw_word_t,
                    arg: *mut c_void,
                ) -> c_int,
            >,
        }

        #[repr(C)]
        pub union unw_save_loc_t_u {
            pub addr: unw_word_t,
            pub regnum: unw_regnum_t,
        }

        #[repr(C)]
        pub struct unw_save_loc_t {
            pub type_: unw_save_loc_type_t,
            pub u: unw_save_loc_t_u,
            pub extra: unw_tdep_save_loc_t,
        }

        pub const UNW_REG_IP: c_int = UNW_TDEP_IP;
        pub const UNW_REG_SP: c_int = UNW_TDEP_SP;
        pub const UNW_REG_EH: c_int = UNW_TDEP_EH;
        pub const UNW_REG_LAST: c_int = UNW_TDEP_LAST_REG;
    }
}