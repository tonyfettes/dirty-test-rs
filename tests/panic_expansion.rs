use micro_test::{micro_call, report::{Reporter, set_reporter}};
use micro_test::micro_panic;
use micro_test::panic::micro_panic_receiver;
use micro_test::{
    backtrace::CallStack,
    panic::{micro_panic_relay, set_panic_handler, PanicInfo},
};

#[micro_panic_relay]
fn target_no_input_no_output() {
    println!("target with no input and no output");
    micro_panic!("PANIC!");
}

#[micro_panic_relay]
fn recursive_panic_relay(level: usize) {
    if level == 0 {
        micro_panic!("NO LEVELS LEFT!");
    } else {
        micro_call!(relay recursive_panic_relay(level - 1));
        panic!("SHOULDN'T PANIC HERE!");
    }
}

#[micro_panic_relay]
fn target_input_output(mut v: Vec<usize>) -> Vec<usize> {
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    micro_panic!("PANIC!");
    panic!("SHOULDN'T PANIC HERE!");
    Vec::<usize>::new()
}

#[test]
#[micro_panic_receiver]
fn panic_main() {

    fn handle(info: &PanicInfo) {
        println!("{}", info.message.unwrap());
    }

    set_panic_handler(handle);
    set_reporter(&Reporter {
        metadata: None,
        result: None,
        call_stack: Some(|call_stack: CallStack| {
            for (i, call) in call_stack.calls.iter().enumerate() {
                println!("#{}: {}", i, call.name);
            }
        }),
    });
    let v: Vec<usize> = vec![5, 6, 7, 8];
    micro_call!(receive target_input_output(v));
    micro_call!(receive recursive_panic_relay(10));
}
