#![feature(const_type_name)]

use quill::*;

#[quill::component]
pub struct Account {
    money: i32,
}

#[quill::component]
pub struct Foo(i32);

#[quill::component]
pub struct Bar;

#[quill::init]
pub fn my_plugin() -> Plugin {
    PluginBuilder::default()
        .build()
}

fn player_jump(account: Account) {
    // jump.set_velocity(0, 0, 0);
}