// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::env::var;
// (header name, &[header dependencies], &[library dependencies])
const DATA: &'static [(&'static str, &'static [&'static str], &'static [&'static str])] = &[
    // km
    // mmos
    // shared
    ("basetsd", &[], &[]),
    ("bcrypt", &["minwindef", "winnt"], &["bcrypt"]),
    ("bugcodes", &["ntdef"], &[]),
    ("cderr", &["minwindef"], &[]),
    ("cfg", &["minwindef"], &[]),
    ("d3d9", &["basetsd", "d3d9caps", "d3d9types", "guiddef", "minwindef", "unknwnbase", "windef", "wingdi", "winnt"], &[]),
    ("d3d9caps", &["d3d9types", "guiddef", "minwindef", "winnt"], &[]),
    ("d3d9types", &["basetsd", "guiddef", "minwindef", "windef", "winnt"], &[]),
    ("dcomptypes", &["dxgitype", "minwindef", "winnt"], &[]),
    ("devguid", &[], &[]),
    ("devpkey", &["devpropdef"], &[]),
    ("devpropdef", &["guiddef", "minwindef", "winnt"], &[]),
    ("dxgi", &["basetsd", "dxgiformat", "dxgitype", "guiddef", "minwindef", "unknwnbase", "windef", "winnt"], &[]),
    ("dxgi1_2", &["basetsd", "dxgi", "dxgiformat", "dxgitype", "guiddef", "minwinbase", "minwindef", "unknwnbase", "windef", "winnt"], &[]),
    ("dxgi1_3", &["dxgi", "dxgi1_2", "dxgiformat", "minwindef", "unknwnbase", "windef", "winnt"], &[]),
    ("dxgi1_4", &["basetsd", "dxgi1_2", "dxgi1_3", "dxgiformat", "dxgitype", "guiddef", "minwindef", "unknwnbase", "winnt"], &[]),
    ("dxgi1_5", &["basetsd", "dxgi", "dxgi1_2", "dxgi1_3", "dxgi1_4", "dxgiformat", "minwindef", "unknwnbase", "winnt"], &[]),
    ("dxgiformat", &[], &[]),
    ("dxgitype", &["d3d9types", "dxgiformat", "minwindef"], &[]),
    ("guiddef", &[], &[]),
    ("hidclass", &["guiddef", "minwindef", "winioctl", "winnt"], &[]),
    ("hidpi", &["hidusage", "minwindef", "ntdef", "ntstatus", "winnt"], &[]),
    ("hidsdi", &["minwindef", "winnt"], &[]),
    ("hidusage", &["minwindef"], &[]),
    ("in6addr", &["minwindef"], &[]),
    ("inaddr", &["minwindef"], &[]),
    ("intsafe", &[], &[]),
    ("ksmedia", &[], &[]),
    ("lmcons", &["minwindef", "winnt"], &[]),
    ("minwindef", &["basetsd", "ntdef"], &[]),
    ("mmreg", &["guiddef", "minwindef"], &[]),
    ("mstcpip", &["basetsd", "guiddef", "in6addr", "inaddr", "minwindef", "winnt", "ws2def"], &["ntdll"]),
    ("ntddser", &["devpropdef"], &[]),
    ("ntdef", &["basetsd", "guiddef"], &[]),
    ("ntstatus", &["ntdef"], &[]),
    ("qos", &["minwindef"], &[]),
    ("rpc", &[], &[]),
    ("rpcdce", &["guiddef", "minwindef", "rpc"], &[]),
    ("rpcndr", &[], &[]),
    ("sspi", &["basetsd", "guiddef", "minwindef", "subauth", "wincred", "winnt"], &["credui", "secur32"]),
    ("usb", &["minwindef", "winnt"], &[]),
    ("usbiodef", &["guiddef", "minwindef", "winioctl", "winnt"], &[]),
    ("windef", &["minwindef", "winnt"], &[]),
    ("windowsx", &["minwindef"], &[]),
    ("winerror", &["minwindef"], &[]),
    ("wnnc", &["minwindef"], &[]),
    ("ws2def", &["basetsd", "guiddef", "inaddr", "minwindef", "vcruntime", "winnt"], &[]),
    ("ws2ipdef", &["in6addr", "inaddr", "minwindef"], &[]),
    ("wtypes", &["guiddef", "minwindef", "ntdef", "wtypesbase"], &[]),
    ("wtypesbase", &["minwindef", "rpcndr", "winnt"], &[]),
    // ucrt
    // um
    ("audioclient", &["audiosessiontypes", "basetsd", "guiddef", "minwindef", "mmreg", "strmif", "unknwnbase", "winerror", "winnt", "wtypesbase"], &[]),
    ("audiosessiontypes", &["minwindef"], &[]),
    ("avrt", &["guiddef", "minwindef", "winnt"], &["avrt"]),
    ("cfgmgr32", &["basetsd", "guiddef", "minwindef", "winnt"], &[]),
    ("cguid", &[], &[]),
    ("combaseapi", &["basetsd", "guiddef", "minwindef", "objidl", "objidlbase", "unknwnbase", "winnt", "wtypesbase"], &["ole32"]),
    ("coml2api", &["minwindef"], &[]),
    ("commapi", &["minwinbase", "minwindef", "winbase", "winnt"], &["kernel32"]),
    ("commctrl", &["basetsd", "commoncontrols", "guiddef", "minwinbase", "minwindef", "vcruntime", "windef", "winnt", "winuser"], &["comctl32"]),
    ("commdlg", &["basetsd", "minwindef", "prsht", "unknwnbase", "windef", "wingdi", "winnt", "winuser"], &[]),
    ("commoncontrols", &["commctrl", "guiddef", "minwindef", "unknwnbase", "windef", "winnt"], &["comctl32"]),
    ("consoleapi", &["minwindef", "wincon", "winnt"], &["kernel32"]),
    ("corsym", &["basetsd", "objidlbase", "unknwnbase", "winnt"], &[]),
    ("d2d1", &["basetsd", "d2dbasetypes", "d3dcommon", "dcommon", "dwrite", "dxgi", "guiddef", "minwindef", "unknwnbase", "wincodec", "windef", "winnt"], &[]),
    ("d2d1_1", &["basetsd", "d2d1", "d2d1effectauthor", "d2dbasetypes", "dcommon", "documenttarget", "dwrite", "dxgi", "dxgiformat", "guiddef", "minwindef", "objidlbase", "unknwnbase", "wincodec", "winnt"], &["d2d1"]),
    ("d2d1_2", &["d2d1", "d2d1_1", "dxgi", "minwindef", "winnt"], &["d2d1"]),
    ("d2d1effectauthor", &["basetsd", "minwindef", "ntdef", "unknwnbase"], &[]),
    ("d2d1effects", &[], &[]),
    ("d2d1effects_1", &[], &[]),
    ("d2dbasetypes", &["basetsd", "d3d9types", "minwindef", "windef"], &[]),
    ("d3d10", &["d3dcommon"], &[]),
    ("d3d10shader", &["d3d10", "d3dcommon", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3d11", &["basetsd", "d3dcommon", "dxgiformat", "dxgitype", "guiddef", "minwindef", "unknwnbase", "windef", "winnt"], &[]),
    ("d3d11on12", &["d3d11", "d3d12", "d3dcommon", "guiddef", "minwindef", "unknwnbase", "winnt"], &["d3d11"]),
    ("d3d11shader", &["basetsd", "d3dcommon", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3d12", &["basetsd", "d3dcommon", "dxgiformat", "dxgitype", "guiddef", "minwinbase", "minwindef", "unknwnbase", "windef", "winnt"], &[]),
    ("d3d12sdklayers", &["basetsd", "d3d12", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3d12shader", &["basetsd", "d3dcommon", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3dcommon", &["basetsd", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3dcompiler", &["basetsd", "d3dcommon", "minwindef"], &[]),
    ("dbghelp", &["basetsd", "guiddef", "minwindef", "vcruntime", "winnt"], &[]),
    ("dcommon", &["dxgiformat"], &[]),
    ("dcomp", &["d2d1", "d2d1_1", "d2d1effects", "d2dbasetypes", "d3d9types", "d3dcommon", "dcompanimation", "dcomptypes", "dxgi", "dxgi1_2", "dxgiformat", "guiddef", "minwinbase", "minwindef", "ntdef", "unknwnbase", "windef"], &["dcomp"]),
    ("dcompanimation", &["ntdef", "unknwnbase"], &[]),
    ("docobj", &["guiddef", "minwindef", "oaidl", "unknwnbase", "winnt"], &[]),
    ("documenttarget", &["basetsd", "guiddef", "ntdef", "unknwnbase"], &[]),
    ("dpa_dsa", &["basetsd", "minwindef", "winnt"], &["comctl32"]),
    ("dpapi", &["minwindef", "wincrypt", "windef", "winnt"], &["crypt32"]),
    ("dsgetdc", &["guiddef", "minwindef", "winnt"], &[]),
    ("dsound", &["guiddef", "minwindef", "mmsystem", "unknwnbase", "windef", "winerror", "winnt"], &[]),
    ("dsrole", &["guiddef", "minwindef", "winnt"], &[]),
    ("dwmapi", &["minwindef"], &[]),
    ("dwrite", &["basetsd", "d2d1", "dcommon", "guiddef", "minwindef", "unknwnbase", "windef", "winerror", "wingdi", "winnt"], &["dwrite"]),
    ("dwrite_1", &["basetsd", "dcommon", "dwrite", "minwindef", "winnt"], &[]),
    ("dwrite_2", &["basetsd", "d3d9types", "dcommon", "dwrite", "dwrite_1", "minwindef", "unknwnbase", "winnt"], &[]),
    ("dwrite_3", &["basetsd", "dcommon", "dwrite", "dwrite_1", "dwrite_2", "minwindef", "unknwnbase", "wingdi", "winnt"], &[]),
    ("errhandlingapi", &["basetsd", "minwindef", "winnt"], &[]),
    ("fileapi", &["minwinbase", "minwindef", "winnt"], &["kernel32"]),
    ("gl-gl", &[], &[]),
    ("handleapi", &["minwindef", "winnt"], &["kernel32"]),
    ("heapapi", &["basetsd", "minwindef"], &[]),
    ("http", &["guiddef", "minwindef", "sspi", "winnt", "ws2def"], &[]),
    ("imm", &[], &[]),
    ("libloaderapi", &["basetsd", "minwindef", "winnt"], &["kernel32", "user32"]),
    ("lmaccess", &["lmcons", "minwindef", "winnt"], &[]),
    ("lmdfs", &["guiddef", "minwindef", "winnt"], &[]),
    ("lmerrlog", &["minwindef", "winnt"], &[]),
    ("lmjoin", &["minwindef", "winnt"], &[]),
    ("lsalookup", &["guiddef", "minwindef", "winnt"], &[]),
    ("memoryapi", &["basetsd", "minwindef", "winnt"], &[]),
    ("minschannel", &["guiddef", "minwindef", "wincrypt", "winnt"], &[]),
    ("minwinbase", &["basetsd", "minwindef", "ntstatus", "winnt"], &[]),
    ("mmdeviceapi", &["guiddef", "minwindef", "propidl", "propsys", "unknwnbase", "winnt", "wtypes"], &[]),
    ("mmsystem", &["basetsd", "minwindef", "mmreg", "winnt"], &[]),
    ("mscat", &["guiddef", "minwindef", "mssip", "wincrypt", "winnt"], &[]),
    ("mssip", &["guiddef", "minwindef", "mscat", "wincrypt", "winnt"], &["crypt32"]),
    ("nb30", &["minwindef", "winnt"], &[]),
    ("ncrypt", &["basetsd", "sspi"], &["ncrypt"]),
    ("ntsecapi", &["basetsd", "guiddef", "lsalookup", "minwindef", "ntdef", "sspi", "subauth", "winnt"], &["advapi32"]),
    ("oaidl", &["basetsd", "guiddef", "minwindef", "rpcndr", "unknwnbase", "winnt", "wtypes", "wtypesbase"], &[]),
    ("objbase", &["combaseapi", "minwindef", "winnt"], &["ole32"]),
    ("objidl", &["basetsd", "guiddef", "minwindef", "objidlbase", "unknwnbase", "winnt", "wtypesbase"], &[]),
    ("objidlbase", &["basetsd", "guiddef", "minwindef", "unknwnbase", "winnt", "wtypesbase"], &[]),
    ("ocidl", &["guiddef", "minwindef", "ntdef", "oaidl", "unknwnbase", "wtypes", "wtypesbase"], &[]),
    ("oleauto", &["minwindef", "oaidl", "winnt", "wtypes", "wtypesbase"], &["oleaut32"]),
    ("olectl", &["winerror", "winnt"], &[]),
    ("pdh", &["basetsd", "guiddef", "minwindef", "windef", "winnt", "wtypesbase"], &["pdh"]),
    ("playsoundapi", &["minwindef"], &[]),
    ("processsnapshot", &["minwindef"], &[]),
    ("processthreadsapi", &["basetsd", "guiddef", "minwinbase", "minwindef", "winnt"], &["advapi32", "kernel32"]),
    ("propidl", &["minwindef", "wtypes"], &[]),
    ("propkeydef", &["guiddef", "wtypes"], &[]),
    ("propsys", &["minwindef", "propidl", "propkeydef", "unknwnbase", "winnt", "wtypes"], &[]),
    ("prsht", &["basetsd", "minwindef", "windef", "winnt", "winuser"], &["comctl32"]),
    ("psapi", &["basetsd", "minwindef", "winnt"], &["psapi"]),
    ("reason", &["minwindef"], &[]),
    ("restrictederrorinfo", &["unknwnbase", "winnt", "wtypes"], &[]),
    ("sapi", &["guiddef", "minwindef", "sapi53", "unknwnbase", "winnt"], &[]),
    ("sapi51", &["guiddef", "minwindef", "mmreg", "oaidl", "objidlbase", "servprov", "unknwnbase", "windef", "winnt", "wtypes", "wtypesbase"], &[]),
    ("sapi53", &["guiddef", "minwindef", "oaidl", "sapi51", "unknwnbase", "urlmon", "winnt", "wtypes"], &[]),
    ("sapiddk", &["guiddef", "minwindef", "sapi", "sapiddk51", "unknwnbase", "winnt"], &[]),
    ("sapiddk51", &["guiddef", "minwindef", "mmreg", "oaidl", "objidlbase", "sapi", "unknwnbase", "windef", "winnt"], &[]),
    ("schannel", &["guiddef", "minwindef", "wincrypt", "windef", "winnt"], &[]),
    ("securitybaseapi", &["minwindef", "winnt"], &["advapi32", "kernel32"]),
    ("servprov", &["guiddef", "unknwnbase", "winnt"], &[]),
    ("setupapi", &["basetsd", "commctrl", "devpropdef", "guiddef", "minwindef", "prsht", "spapidef", "windef", "winnt", "winreg"], &["setupapi"]),
    ("shellapi", &["basetsd", "guiddef", "minwinbase", "minwindef", "processthreadsapi", "windef", "winnt", "winuser"], &["shell32", "shlwapi"]),
    ("shellscalingapi", &[], &[]),
    ("shlobj", &["minwindef"], &[]),
    ("shobjidl", &["guiddef", "minwindef", "objidl", "propkeydef", "propsys", "shobjidl_core", "shtypes", "unknwnbase", "windef", "winnt"], &[]),
    ("shobjidl_core", &["guiddef", "minwindef", "objidl", "unknwnbase", "windef", "winnt"], &[]),
    ("shtypes", &["guiddef", "minwindef", "winnt"], &[]),
    ("spapidef", &["minwindef", "winnt"], &[]),
    ("sporder", &["guiddef", "minwindef"], &["sporder"]),
    ("sql", &["sqltypes"], &[]),
    ("sqlext", &["sql", "sqltypes"], &[]),
    ("sqltypes", &["basetsd", "guiddef", "windef"], &[]),
    ("sqlucode", &["sqltypes"], &[]),
    ("strmif", &["winnt"], &[]),
    ("subauth", &["minwindef", "winnt"], &[]),
    ("synchapi", &["basetsd", "minwinbase", "minwindef", "winnt"], &["kernel32", "synchronization"]),
    ("sysinfoapi", &["basetsd", "minwindef", "winnt"], &[]),
    ("threadpoolapiset", &["basetsd", "minwindef", "winnt"], &[]),
    ("timezoneapi", &["minwinbase", "minwindef", "winnt"], &[]),
    ("tlhelp32", &["basetsd", "minwindef", "winnt"], &[]),
    ("unknwnbase", &["guiddef", "minwindef", "winnt"], &[]),
    ("urlhist", &["docobj", "guiddef", "minwindef", "unknwnbase", "winnt", "wtypesbase"], &[]),
    ("urlmon", &["minwindef", "unknwnbase", "winnt"], &[]),
    ("usbspec", &["basetsd", "minwindef"], &[]),
    ("usp10", &["minwindef", "ntdef", "windef", "winerror", "wingdi", "winnt"], &["usp10"]),
    ("vsbackup", &["minwindef", "unknwnbase", "vss", "vswriter", "winnt", "wtypes"], &[]),
    ("vss", &["guiddef", "minwindef", "unknwnbase", "winnt"], &[]),
    ("vsserror", &["winnt"], &[]),
    ("vswriter", &["minwindef", "unknwnbase", "vss", "winnt", "wtypes"], &[]),
    ("werapi", &["minwindef", "winnt"], &["kernel32"]),
    ("winbase", &["basetsd", "cfgmgr32", "fileapi", "guiddef", "libloaderapi", "minwinbase", "minwindef", "processthreadsapi", "vadefs", "windef", "winnt"], &["kernel32"]),
    ("wincodec", &["basetsd", "d2d1", "d2d1_1", "dcommon", "dxgiformat", "dxgitype", "guiddef", "minwindef", "ntdef", "objidlbase", "ocidl", "propidl", "unknwnbase", "windef", "winerror", "winnt"], &[]),
    ("wincon", &["minwinbase", "minwindef", "windef", "wingdi", "winnt"], &["kernel32"]),
    ("wincred", &["minwindef", "sspi", "windef", "winnt"], &["advapi32", "credui"]),
    ("wincrypt", &["basetsd", "bcrypt", "guiddef", "minwinbase", "minwindef", "ncrypt", "vcruntime", "winnt"], &["advapi32", "crypt32", "cryptnet"]),
    ("winevt", &["basetsd", "guiddef", "minwinbase", "minwindef", "vcruntime", "winnt"], &["wevtapi"]),
    ("wingdi", &["basetsd", "minwindef", "windef", "winnt"], &["gdi32"]),
    ("winhttp", &["basetsd", "minwindef", "winnt"], &[]),
    ("wininet", &["basetsd", "minwinbase", "minwindef", "ntdef", "windef", "winineti", "winnt"], &["wininet"]),
    ("winineti", &["minwindef"], &[]),
    ("winioctl", &["devpropdef", "minwindef", "winnt"], &[]),
    ("winnetwk", &["basetsd", "minwindef", "windef", "winerror", "winnt"], &[]),
    ("winnls", &["basetsd", "guiddef", "minwindef", "winnt"], &[]),
    ("winnt", &["basetsd", "excpt", "guiddef", "minwindef"], &[]),
    ("winreg", &["basetsd", "minwinbase", "minwindef", "winnt"], &["advapi32"]),
    ("winscard", &["basetsd", "guiddef", "minwindef", "windef", "winnt"], &[]),
    ("winsmcrd", &["minwindef", "winioctl"], &[]),
    ("winsock2", &["basetsd", "guiddef", "inaddr", "minwinbase", "minwindef", "qos", "winbase", "windef", "winerror", "winnt", "ws2def", "wtypesbase"], &["ws2_32"]),
    ("winspool", &["minwindef", "wingdi", "winnt"], &[]),
    ("winsvc", &["minwindef", "winnt"], &["advapi32"]),
    ("winusb", &["minwindef", "winnt"], &[]),
    ("winusbio", &["minwindef", "usb"], &[]),
    ("winuser", &["basetsd", "guiddef", "limits", "minwinbase", "minwindef", "vadefs", "windef", "wingdi", "winnt"], &["user32"]),
    ("ws2spi", &["basetsd", "guiddef", "minwindef", "vcruntime", "windef", "winnt", "winsock2", "ws2def", "wtypesbase"], &["ws2_32"]),
    ("ws2tcpip", &["guiddef", "minwinbase", "minwindef", "mstcpip", "vcruntime", "winerror", "winnt", "winsock2", "ws2def", "wtypesbase"], &["fwpuclnt", "ws2_32"]),
    ("xinput", &["minwindef", "winnt"], &[]),
    // vc
    ("excpt", &[], &[]),
    ("limits", &[], &[]),
    ("vadefs", &[], &[]),
    ("vcruntime", &[], &[]),
    // winrt
    ("activation", &["inspectable", "winnt"], &[]),
    ("hstring", &["winnt"], &[]),
    ("inspectable", &["guiddef", "hstring", "minwindef", "unknwnbase", "winnt"], &[]),
    ("roapi", &["activation", "hstring", "winnt"], &[]),
    ("roerrorapi", &["basetsd", "minwindef", "winnt"], &[]),
    ("winstring", &["basetsd", "minwindef", "winnt"], &[]),
];
struct Header {
    required: bool,
    included: Cell<bool>,
    dependencies: &'static [&'static str],
    libraries: &'static [&'static str],
}
struct Graph(HashMap<&'static str, Header>);
impl Graph {
    fn generate() -> Graph {
        Graph(DATA.iter().map(|&(name, dependencies, libraries)| {
            let header = Header {
                required: false,
                included: Cell::new(false),
                dependencies: dependencies,
                libraries: libraries,
            };
            (name, header)
        }).collect())
    }
    fn identify_required(&mut self) {
        for (name, header) in &mut self.0 {
            if let Ok(_) = var(&format!("CARGO_FEATURE_{}", name.to_uppercase())) {
                header.required = true;
                header.included.set(true);
            }
        }
    }
    fn check_everything(&self) {
        if let Ok(_) = var("CARGO_FEATURE_EVERYTHING") {
            for (_, header) in &self.0 {
                header.included.set(true);
            }
        }
    }
    fn resolve_dependencies(&self) {
        let mut done = false;
        while !done {
            done = true;
            for (_, header) in &self.0 {
                if header.included.get() {
                    for dep in header.dependencies {
                        let dep = &self.0.get(dep).expect(dep);
                        if !dep.included.get() {
                            done = false;
                            dep.included.set(true);
                        }
                    }
                }
            }
        }
    }
    fn emit_features(&self) {
        for (name, header) in &self.0 {
            if header.included.get() && !header.required {
                println!("cargo:rustc-cfg=feature=\"{}\"", name);
            }
        }
    }
    fn emit_libraries(&self) {
        let libs = self.0.iter().filter(|&(_, header)| {
            header.included.get()
        }).flat_map(|(_, header)| {
            header.libraries.iter()
        }).collect::<HashSet<_>>();
        for lib in libs {
            println!("cargo:rustc-link-lib=dylib={}", lib);
        }
    }
}
fn try_everything() {
    let mut graph = Graph::generate();
    graph.identify_required();
    graph.check_everything();
    graph.resolve_dependencies();
    graph.emit_features();
    graph.emit_libraries();
}
fn main() {
    let target = var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target.get(2) == Some(&"windows") {
        try_everything();
    }
}
