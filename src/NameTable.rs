use crate::{u8Size, Utils, thisFileWillBeDeleted};
use std::fs;
use memmap::MmapMut;

pub fn gen_name_table_from_path(path: &str) -> MmapMut { Utils::get_memmap(path, 10_000_000_000) }

pub fn place_name(mmap: &mut MmapMut, offset: u64, name: &[u8]) -> u64 {
    let offset = offset as usize;
    let len = name.len();
    Utils::place_item_raw(mmap, offset, &(len as u8));
    let offset = offset + u8Size;
    mmap[offset..offset+len].copy_from_slice(name);
    offset as u64 + len as u64
}

pub fn get_name(mmap: &MmapMut, offset: u64) -> Option<String> {
    let offset = offset as usize;
    let name_size: u8 = unsafe { *Utils::bytes_to_type_mut(&mmap[offset..offset+u8Size]) };
    if name_size == 0 { return None }
    let offset = offset + u8Size;
    let nameAsBytes = &mmap[offset..offset+name_size as usize];
    match std::str::from_utf8(nameAsBytes) {
        Ok(T) => Some(T.to_string()),
        Err(E) => None
    }
}

#[test]
fn place_and_get_names() {
    let scr = thisFileWillBeDeleted;

    let mut mmap = Utils::get_memmap(scr, 1_000);
    let in_name = "Hans Hansen";
    let offset1 = place_name(&mut mmap, 0, in_name.as_bytes());
    let out_name = get_name(&mmap,0);
    assert!(out_name.is_some());
    assert_eq!(in_name,out_name.unwrap());

    let in_name = "Per";
    let offset2 = place_name(&mut mmap, offset1, in_name.as_bytes());
    let out_name = get_name(&mmap,offset1);
    assert!(out_name.is_some());
    assert_eq!(in_name,out_name.unwrap());

    let in_name = "B";
    let offset3 = place_name(&mut mmap, offset2, in_name.as_bytes());
    let out_name = get_name(&mmap,offset2);
    assert!(out_name.is_some());
    assert_eq!(in_name,out_name.unwrap());

    fs::remove_file(scr);
}