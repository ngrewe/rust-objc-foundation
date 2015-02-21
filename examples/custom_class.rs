#[macro_use]
extern crate objc;
extern crate objc_foundation;

use std::sync::{Once, ONCE_INIT};

use objc::{EncodePtr, Id, Message};
use objc::declare::{ClassDecl, MethodDecl};
use objc::runtime::{Class, Object, Sel};
use objc_foundation::{INSObject, NSObject};

pub enum MYObject { }

impl MYObject {
    fn number(&self) -> u32 {
        let obj = unsafe {
            &*(self as *const _ as *const Object)
        };
        unsafe {
            *obj.get_ivar::<u32>("_number")
        }
    }

    fn set_number(&mut self, number: u32) {
        let obj = unsafe {
            &mut *(self as *mut _ as *mut Object)
        };
        unsafe {
            obj.set_ivar("_number", number);
        }
    }
}

unsafe impl Message for MYObject { }

impl EncodePtr for MYObject {
    fn ptr_code() -> &'static str { "@" }
}

static MYOBJECT_REGISTER_CLASS: Once = ONCE_INIT;

impl INSObject for MYObject {
    fn class() -> &'static Class {
        MYOBJECT_REGISTER_CLASS.call_once(|| {
            let superclass = <NSObject as INSObject>::class();
            let mut decl = ClassDecl::new(superclass, "MYObject").unwrap();
            assert!(decl.add_ivar::<u32>("_number"));

            // Add ObjC methods for getting and setting the number
            extern fn my_object_set_number(this: &mut MYObject, _cmd: Sel, number: u32) {
                this.set_number(number);
            }
            let method = MethodDecl::new(sel!(setNumber:),
                my_object_set_number as extern fn(&mut MYObject, Sel, u32));
            assert!(decl.add_method(method.unwrap()));

            extern fn my_object_get_number(this: &MYObject, _cmd: Sel) -> u32 {
                this.number()
            }
            let method = MethodDecl::new(sel!(number),
                my_object_get_number as extern fn(&MYObject, Sel) -> u32);
            assert!(decl.add_method(method.unwrap()));

            decl.register();
        });

        Class::get("MYObject").unwrap()
    }
}

fn main() {
    let mut obj: Id<MYObject> = INSObject::new();

    obj.set_number(7);
    println!("Number: {}", unsafe {
        let number: u32 = msg_send![obj, number];
        number
    });

    unsafe {
        let _: () = msg_send![obj, setNumber:12u32];
    }
    println!("Number: {}", obj.number());
}
