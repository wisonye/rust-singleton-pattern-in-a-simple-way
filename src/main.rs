mod computer;

use computer::Computer;

///
fn build_computer(computer: &mut Computer) -> &mut Computer {
    computer
        .add_cpu("ARM", "CortexM4")
        .add_memory("SAMSUNG", 48)
}

///
fn main() {
    // my_computer: &'static mut Computer
    let my_computer = Computer::get_computer();
    println!("my_computer: {:#?}", my_computer);

    {
        build_computer(my_computer);
        println!("my_computer: {:#?}", my_computer);
    }

    println!("Should be the same computer instance: {:#?}", Computer::get_computer());
}

