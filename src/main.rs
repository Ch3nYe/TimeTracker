use std::borrow::{Borrow, Cow};
use std::str::{from_utf8, Utf8Error};
use windows::{
    core::*,
    Foundation::Uri,
    Web::Syndication::SyndicationClient,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::MessageBoxA,
    Win32::UI::WindowsAndMessaging::MB_OK,
    Win32::System::Diagnostics::ToolHelp::*,
};

fn parse_chararray(char_array: &[CHAR]) -> String {
    let mut chars:Vec<u8> = Vec::new();
    for c in char_array{
        if c.0 == 0x00 {
            break;
        }
        chars.push(c.0);
    }
    let str= String::from_utf8_lossy(chars.borrow());
    str.into_owned()

}

fn main() -> Result<()> {
    // 获取rss订阅内容-实验
    // get rss subscribe-experiment
    let uri = Uri::CreateUri("https://blogs.windows.com/feed")?;
    let client = SyndicationClient::new()?;
    let feed = client.RetrieveFeedAsync(uri)?.get()?;
    let mut str = String::new();
    for item in feed.Items()? {
        println!("{}", item.Title()?.Text()?);
        str.push_str(&(item.Title()?.Text()?.to_string_lossy()+"\n"));
    }

    unsafe {
        MessageBoxA(None, str, "RSS", MB_OK);
    }

    // 打印当前进程-实验
    // print process-experiment
    unsafe {
        let hProcessSnap: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if (hProcessSnap == INVALID_HANDLE_VALUE)
        {
            println!("CreateToolhelp32Snapshot error.");
            return Ok(())
        }

        let mut pe32 = PROCESSENTRY32{
            dwSize:304,
            cntUsage:0,
            th32ProcessID:0,
            th32DefaultHeapID:0,
            th32ModuleID:0,
            cntThreads:0,
            th32ParentProcessID:0,
            pcPriClassBase:0,
            dwFlags:0,
            szExeFile:[CHAR(0x00); 260],
        };
        let mpe32 = &mut pe32 as *mut PROCESSENTRY32;
        let mut bProcess = Process32First(hProcessSnap, mpe32);
        println!("[-]execute here! parse success? {}",bProcess.as_bool());
        let mut str = String::new();
        while (bProcess.as_bool())
        {
            //打印进程名和进程ID
            let exefile = parse_chararray(&pe32.szExeFile);
            println!("[+]{}---------{}", exefile, pe32.th32ProcessID);
            if pe32.th32ProcessID<1000 { // too long, dont show
                str.push_str(format!("[+]{}---------{}\n", exefile, pe32.th32ProcessID).borrow());
            }
            bProcess = Process32Next(hProcessSnap, mpe32);
        }

        MessageBoxA(None, str, "Process", MB_OK);
    }

    Ok(())
}