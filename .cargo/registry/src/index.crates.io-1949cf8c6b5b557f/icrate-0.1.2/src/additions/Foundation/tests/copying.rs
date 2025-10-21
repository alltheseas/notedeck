#![cfg(feature = "Foundation_NSString")]
use icrate::Foundation::{NSCopying, NSMutableCopying, NSString};
use objc2::{rc::Id, runtime::ProtocolObject};

#[test]
fn copy() {
    let obj = NSString::new();
    let protocol_object: &ProtocolObject<dyn NSCopying> = ProtocolObject::from_ref(&*obj);
    let _: Id<ProtocolObject<dyn NSCopying>> = protocol_object.copy();
}

#[test]
fn copy_mutable() {
    let obj = NSString::new();
    let protocol_object: &ProtocolObject<dyn NSMutableCopying> = ProtocolObject::from_ref(&*obj);
    let _: Id<ProtocolObject<dyn NSMutableCopying>> = protocol_object.mutableCopy();
}
