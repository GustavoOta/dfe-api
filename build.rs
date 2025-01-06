extern crate winapi;
extern crate winres;

use winres::WindowsResource;
fn main() {
    WindowsResource::new()
        .set_icon("im-ugly.ico")
        .set_language(winapi::um::winnt::MAKELANGID(
            winapi::um::winnt::LANG_PORTUGUESE,
            winapi::um::winnt::SUBLANG_PORTUGUESE_BRAZILIAN,
        ))
        .set("ProductName", "DFe API REST")
        .set(
            "FileDescription",
            "API REST para DFe (Documentos Fiscais Eletronicos)",
        )
        .set("LegalCopyright", "Gustavo Ota")
        .set("Legaltrademarks", "Gustavo Ota")
        .compile()
        .unwrap();
}
