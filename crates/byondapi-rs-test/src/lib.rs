#![allow(clippy::missing_safety_doc)]

use byondapi::{
    list::ByondValueList,
    parse_args,
    value::{pointer::ByondValuePointer, ByondValue},
};

#[no_mangle]
pub unsafe extern "C" fn test_connection(
    _argc: byondapi_sys::u4c,
    _argv: *mut ByondValue,
) -> ByondValue {
    ByondValue::new_num(69.0)
}

#[no_mangle]
pub unsafe extern "C" fn test_args(argc: byondapi_sys::u4c, argv: *mut ByondValue) -> ByondValue {
    let args = parse_args(argc, argv);
    assert_eq!(args.len(), 1);
    args[0].clone()
}

#[no_mangle]
pub unsafe extern "C" fn send_test(_argc: byondapi_sys::u4c, _argv: *mut ByondValue) -> ByondValue {
    // let args = parse_args(argc, argv);
    let new_value = ByondValue::new_str("Meow").unwrap();

    std::thread::spawn(move || {
        std::mem::drop(new_value);
    });

    ByondValue::null()
}

#[no_mangle]
pub unsafe extern "C" fn test_ptr(argc: byondapi_sys::u4c, argv: *mut ByondValue) -> ByondValue {
    let args = parse_args(argc, argv);
    let pointer = match ByondValuePointer::new(args[0].clone()) {
        Ok(ptr) => ptr,
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    let strobj = match pointer.read() {
        Ok(ptr) => ptr,
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    let new_name: ByondValue = format!("awa{}", strobj.get_string().unwrap())
        .try_into()
        .unwrap();

    match pointer.write(&new_name) {
        Ok(_) => {}
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    ByondValue::null()
}

#[no_mangle]
pub unsafe extern "C" fn test_proc_call(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    let args = parse_args(argc, argv);

    // FIXME: Byond will change this in the future
    let result = args[0].call("get name", &[]);

    match result {
        Ok(res) => res,
        Err(e) => format!("{:#?}", e).try_into().unwrap(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_readwrite_var(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    let args = parse_args(argc, argv);
    let object = &args[0];

    match object.read_var("name") {
        Ok(res) => res,
        Err(e) => format!("{:#?}", e).try_into().unwrap(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_list_push(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    let args = parse_args(argc, argv);

    let mut list: ByondValueList = match (&args[0]).try_into() {
        Ok(list) => list,
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    match list.push(&ByondValue::new_num(8.0)) {
        Ok(_) => {}
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    list.try_into().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn test_list_double(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    let args = parse_args(argc, argv);

    let mut _list: ByondValueList = match (&args[0]).try_into() {
        Ok(list) => list,
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    ByondValue::null()
}
