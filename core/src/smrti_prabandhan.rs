use std::fmt::Display;
use std::mem;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::NonNull;

use super::mulya::Mulya;
use super::smrti::Smrti;
use super::smrti::VAISHVIK_ALLOCATOR;
use super::vastuni::Avastha;
use super::vastuni::GyaatMaan;
use super::vastuni::Paddhati;
use super::vastuni::Sutra;
use super::vastuni::Utsarga;
use super::vastuni::Vakya;
use super::vastuni::VastuType;
use super::vastuni::Vidhi;

#[repr(C)]
pub struct GcVastu {
    marked: bool,
    next: Option<NonNull<GcVastu>>,
    obj_type: VastuType,
}

impl GcVastu {
    pub fn new(obj_type: VastuType) -> Self {
        Self {
            marked: false,
            next: None,
            obj_type,
        }
    }
}

pub struct GcRef<T> {
    pointer: NonNull<T>,
}

impl<T> GcRef<T> {
    pub fn dangling() -> GcRef<T> {
        GcRef {
            pointer: NonNull::dangling(),
        }
    }
}

impl<T> Deref for GcRef<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.pointer.as_ref() }
    }
}

impl<T> DerefMut for GcRef<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.pointer.as_mut() }
    }
}

impl<T> Copy for GcRef<T> {}

impl<T> Clone for GcRef<T> {
    fn clone(&self) -> GcRef<T> {
        *self
    }
}

impl<T> Eq for GcRef<T> {}

impl<T> PartialEq for GcRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.pointer == other.pointer
    }
}

pub struct Gc {
    next_gc: usize,
    first: Option<NonNull<GcVastu>>,
    strings: Smrti,
    grey_rashi: Vec<NonNull<GcVastu>>,
}
impl Default for Gc {
    fn default() -> Self {
        Self::new()
    }
}
impl Gc {
    const HEAP_GROW_FACTOR: usize = 2;

    pub fn new() -> Self {
        Gc {
            next_gc: 1024 * 1024,
            first: None,
            strings: Smrti::new(),
            grey_rashi: Vec::new(),
        }
    }

    pub fn alloc<T: Display + 'static>(&mut self, vastu: T) -> GcRef<T> {
        unsafe {
            let boxed = Box::new(vastu);
            let pointer = NonNull::new_unchecked(Box::into_raw(boxed));
            let mut mukhya: NonNull<GcVastu> = mem::transmute(pointer.as_ref());
            mukhya.as_mut().next = self.first.take();
            self.first = Some(mukhya);
            GcRef { pointer }
        }
    }

    pub fn intern(&mut self, s: String) -> GcRef<Vakya> {
        let ls = Vakya::from_string(s);
        if let Some(mulya) = self.strings.find_string(&ls.s, ls.hash) {
            mulya
        } else {
            let reference = self.alloc(ls);
            self.strings.set(reference, Mulya::Na);
            reference
        }
    }

    pub fn collect_garbage(&mut self) {
        self.trace_references();
        self.remove_white_strings();
        self.sweep();
        self.next_gc = VAISHVIK_ALLOCATOR.bytes_allocated() * Gc::HEAP_GROW_FACTOR;
    }

    fn trace_references(&mut self) {
        while let Some(pointer) = self.grey_rashi.pop() {
            self.blacken_vastu(pointer);
        }
    }

    fn blacken_vastu(&mut self, pointer: NonNull<GcVastu>) {
        let vastu_type = unsafe { &pointer.as_ref().obj_type };
        match vastu_type {
            VastuType::Sutra => {
                let sutra: &Sutra = unsafe { mem::transmute(pointer.as_ref()) };
                self.mark_vastu(sutra.name);
                for &achar in &sutra.vastu.achar {
                    self.mark_mulya(achar);
                }
            }
            VastuType::Utsarga => {
                let utsarga: &Utsarga = unsafe { mem::transmute(pointer.as_ref()) };
                self.mark_vastu(utsarga.sutra);
                for &gyaatmaan in &utsarga.gyaatmaans {
                    self.mark_vastu(gyaatmaan);
                }
            }
            VastuType::GyaatMaan => {
                let gyaatmaan: &GyaatMaan = unsafe { mem::transmute(pointer.as_ref()) };
                if let Some(obj) = gyaatmaan.closed {
                    self.mark_mulya(obj)
                }
            }
            VastuType::Vidhi => {
                let vidhi: &Vidhi = unsafe { mem::transmute(pointer.as_ref()) };
                self.mark_vastu(vidhi.name);
                self.mark_smrti(&vidhi.paddhatis);
            }
            VastuType::Avastha => {
                let avastha: &Avastha = unsafe { mem::transmute(pointer.as_ref()) };
                self.mark_vastu(avastha.vidhi);
                self.mark_smrti(&avastha.fields);
            }
            VastuType::Paddhati => {
                let paddhati: &Paddhati = unsafe { mem::transmute(pointer.as_ref()) };
                self.mark_mulya(paddhati.adatr);
                self.mark_vastu(paddhati.paddhati);
            }
            _ => {}
        }
    }

    pub fn mark_mulya(&mut self, mulya: Mulya) {
        match mulya {
            Mulya::Paddhati(mulya) => self.mark_vastu(mulya),
            Mulya::Vidhi(mulya) => self.mark_vastu(mulya),
            Mulya::Utsarga(mulya) => self.mark_vastu(mulya),
            Mulya::Sutra(mulya) => self.mark_vastu(mulya),
            Mulya::Avastha(mulya) => self.mark_vastu(mulya),
            Mulya::Vakya(mulya) => self.mark_vastu(mulya),
            _ => (),
        }
    }

    pub fn mark_vastu<T: 'static>(&mut self, mut reference: GcRef<T>) {
        unsafe {
            let mut mukhya: NonNull<GcVastu> = mem::transmute(reference.pointer.as_mut());
            mukhya.as_mut().marked = true;
            self.grey_rashi.push(mukhya);
        }
    }

    pub fn mark_smrti(&mut self, smrti: &Smrti) {
        for (k, v) in smrti.iter() {
            self.mark_vastu(k);
            self.mark_mulya(v);
        }
    }

    pub fn should_gc(&self) -> bool {
        VAISHVIK_ALLOCATOR.bytes_allocated() > self.next_gc
    }

    fn sweep(&mut self) {
        let mut previous: Option<NonNull<GcVastu>> = None;
        let mut current: Option<NonNull<GcVastu>> = self.first;
        while let Some(mut vastu) = current {
            unsafe {
                let vastu_ptr = vastu.as_mut();
                current = vastu_ptr.next;
                if vastu_ptr.marked {
                    vastu_ptr.marked = false;
                    previous = Some(vastu);
                } else {
                    if let Some(mut previous) = previous {
                        previous.as_mut().next = vastu_ptr.next
                    } else {
                        self.first = vastu_ptr.next
                    }
                    drop(Box::from_raw(vastu_ptr));
                }
            }
        }
    }

    fn remove_white_strings(&mut self) {
        for (k, _v) in self.strings.iter() {
            let mukhya: &GcVastu = unsafe { mem::transmute(k.pointer.as_ref()) };
            if !mukhya.marked {
                self.strings.delete(k);
            }
        }
    }
}
