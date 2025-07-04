use std::alloc::alloc;
use std::alloc::dealloc;
use std::alloc::GlobalAlloc;
use std::alloc::Layout;
use std::ptr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

#[cfg(not(target_arch = "wasm32"))]
use mimalloc::MiMalloc;
#[cfg(target_arch = "wasm32")]
use wee_alloc::WeeAlloc;

use super::mulya::Mulya;
use super::smrti_prabandhan::GcRef;
use super::vastuni::Vakya;

#[cfg(not(target_arch = "wasm32"))]
static ALLOCATOR: MiMalloc = MiMalloc;

#[cfg(target_arch = "wasm32")]
static ALLOCATOR: WeeAlloc = WeeAlloc::INIT;

pub struct VaishvikAllocator {
    pub bytes_allocated: AtomicUsize,
}

impl VaishvikAllocator {
    pub fn bytes_allocated(&self) -> usize {
        self.bytes_allocated.load(Ordering::Relaxed)
    }
}

unsafe impl GlobalAlloc for VaishvikAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.bytes_allocated
            .fetch_add(layout.size(), Ordering::Relaxed);
        ALLOCATOR.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        ALLOCATOR.dealloc(ptr, layout);
        self.bytes_allocated
            .fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

#[global_allocator]
pub static VAISHVIK_ALLOCATOR: VaishvikAllocator = VaishvikAllocator {
    bytes_allocated: AtomicUsize::new(0),
};

struct Pravisti {
    kunji: Option<GcRef<Vakya>>,
    mulya: Mulya,
}

pub struct Smrti {
    sagkhya: usize,
    dharanasakti: usize,
    pravisti: *mut Pravisti,
}
impl Default for Smrti {
    fn default() -> Self {
        Self::new()
    }
}

impl Smrti {
    const ADHIKATAMA_DHARANSAKTI: f32 = 0.75;

    pub fn new() -> Self {
        Smrti {
            sagkhya: 0,
            dharanasakti: 0,
            pravisti: ptr::null_mut(),
        }
    }

    pub fn set(&mut self, kunji: GcRef<Vakya>, mulya: Mulya) -> bool {
        unsafe {
            if self.sagkhya + 1
                > (self.dharanasakti as f32 * Smrti::ADHIKATAMA_DHARANSAKTI) as usize
            {
                let capacity = if self.dharanasakti < 8 {
                    8
                } else {
                    self.dharanasakti * 2
                };
                self.adjust_capacity(capacity);
            }
            let pravisti = Smrti::find_pravisti(self.pravisti, self.dharanasakti, kunji);
            let is_new_kunji = (*pravisti).kunji.is_none();
            if is_new_kunji {
                if let Mulya::Na = (*pravisti).mulya {
                    self.sagkhya += 1;
                }
            }
            (*pravisti).kunji = Some(kunji);
            (*pravisti).mulya = mulya;
            is_new_kunji
        }
    }

    pub fn get(&self, kunji: GcRef<Vakya>) -> Option<Mulya> {
        unsafe {
            if self.sagkhya == 0 {
                return None;
            }
            let pravisti = Smrti::find_pravisti(self.pravisti, self.dharanasakti, kunji);
            if (*pravisti).kunji.is_none() {
                None
            } else {
                Some((*pravisti).mulya)
            }
        }
    }

    pub fn delete(&mut self, kunji: GcRef<Vakya>) -> bool {
        unsafe {
            if self.sagkhya == 0 {
                return false;
            }
            let pravisti = Smrti::find_pravisti(self.pravisti, self.dharanasakti, kunji);
            if (*pravisti).kunji.is_none() {
                return false;
            }
            (*pravisti).kunji = None;
            (*pravisti).mulya = Mulya::Tarka(true);
            true
        }
    }

    pub fn iter(&self) -> IterSmrti {
        IterSmrti {
            ptr: self.pravisti,
            end: unsafe { self.pravisti.add(self.dharanasakti) },
        }
    }

    pub fn add_all(&mut self, other: &Smrti) {
        unsafe {
            for i in 0..(other.dharanasakti as isize) {
                let pravisti = other.pravisti.offset(i);
                if let Some(kunji) = (*pravisti).kunji {
                    self.set(kunji, (*pravisti).mulya);
                }
            }
        }
    }

    pub fn find_string(&self, s: &str, hash: usize) -> Option<GcRef<Vakya>> {
        unsafe {
            if self.sagkhya == 0 {
                return None;
            }
            let mut anukram = hash & (self.dharanasakti - 1);
            loop {
                let pravisti = self.pravisti.add(anukram);
                match (*pravisti).kunji {
                    Some(kunji) => {
                        if s == kunji.s {
                            return Some(kunji);
                        }
                    }
                    None => {
                        if let Mulya::Na = (*pravisti).mulya {
                            return None;
                        }
                    }
                }
                anukram = (anukram + 1) & (self.dharanasakti - 1);
            }
        }
    }

    unsafe fn find_pravisti(
        pravisti: *mut Pravisti,
        capacity: usize,
        kunji: GcRef<Vakya>,
    ) -> *mut Pravisti {
        let mut anukram = kunji.hash & (capacity - 1);
        let mut tombstone: *mut Pravisti = ptr::null_mut();
        loop {
            let pravisti = pravisti.add(anukram);
            match (*pravisti).kunji {
                Some(k) => {
                    if k == kunji {
                        return pravisti;
                    }
                }
                None => {
                    if let Mulya::Na = (*pravisti).mulya {
                        return if !tombstone.is_null() {
                            tombstone
                        } else {
                            pravisti
                        };
                    } else if tombstone.is_null() {
                        tombstone = pravisti;
                    }
                }
            }
            anukram = (anukram + 1) & (capacity - 1);
        }
    }

    unsafe fn adjust_capacity(&mut self, capacity: usize) {
        let mukya_pravisthi = alloc(Layout::array::<Pravisti>(capacity).unwrap()) as *mut Pravisti;
        for i in 0..(capacity as isize) {
            let pravisti = mukya_pravisthi.offset(i);
            (*pravisti).kunji = None;
            (*pravisti).mulya = Mulya::Na
        }
        self.sagkhya = 0;
        for i in 0..(self.dharanasakti as isize) {
            let pravisti = self.pravisti.offset(i);
            match (*pravisti).kunji {
                Some(k) => {
                    let dest = Smrti::find_pravisti(mukya_pravisthi, capacity, k);
                    (*dest).kunji = (*pravisti).kunji;
                    (*dest).mulya = (*pravisti).mulya;
                    self.sagkhya += 1;
                }
                None => continue,
            }
            if (*pravisti).kunji.is_none() {
                continue;
            }
        }
        dealloc(
            self.pravisti.cast(),
            Layout::array::<Pravisti>(self.dharanasakti).unwrap(),
        );
        self.pravisti = mukya_pravisthi;
        self.dharanasakti = capacity;
    }
}

impl Drop for Smrti {
    fn drop(&mut self) {
        unsafe {
            if !self.pravisti.is_null() {
                dealloc(
                    self.pravisti.cast(),
                    Layout::array::<Pravisti>(self.dharanasakti).unwrap(),
                );
            }
        }
    }
}

pub struct IterSmrti {
    ptr: *mut Pravisti,
    end: *const Pravisti,
}

impl Iterator for IterSmrti {
    type Item = (GcRef<Vakya>, Mulya);

    fn next(&mut self) -> Option<Self::Item> {
        while !std::ptr::eq(self.ptr, self.end) {
            unsafe {
                let pravisti = self.ptr;
                self.ptr = self.ptr.offset(1);
                if let Some(kunji) = (*pravisti).kunji {
                    return Some((kunji, (*pravisti).mulya));
                }
            }
        }
        None
    }
}
