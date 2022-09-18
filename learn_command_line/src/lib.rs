use std::collections::HashMap;

pub fn run(question: &str) -> String {
    let hashmap = get_question_answer_hashmap();
    let result = search(question, &hashmap);
    result.to_string()
}

pub fn search <'a>(question: &'a str, hashmap: &'a HashMap<&str, &str>) -> &'a str {
    match hashmap.get(question) {
        Some(answer) => answer,
        None => "There is no such a command"
    }
}

pub fn get_question_answer_hashmap() -> HashMap <&'static str, &'static str> {
    HashMap::from([
        ("how to call a batch file from another one", "call"),
        ("how to change directory", "cd", ),
        ("how to clear screen", "cls", ),
        ("how to start command prompt", "cmd"),
        ("how to change console color", "color"),
        ("how to show/set date", "date"),
        ("how to list directory content", "dir"),
        ("how to text output", "echo"),
        ("how to exits the command prompt or a batch file", "exit"),
        ("how to find files", "find"),
        ("how to display host name", "hostname"),
        ("how to pauses the execution of a batch file and shows a message", "pause"),
        ("how to start a program as another user", "runas"),
        ("how to shutdown the computer", "shutdown"),
        ("how to sort the screen output", "sort"),
        ("how to start an own window to execute a program or command", "start"),
        ("how to terminate a process or a application", "taskkill"),
        ("how to display applications and related tasks", "tasklist"),
        ("how to display/edit the system time", "time"),
        ("how to wait any time", "timeout"),
        ("how to set title for prompt", "title"),
        ("how to display operating system version", "ver"),
        ("how to setting time synchronisation/time server/time zone", "w32tm"),
        ("how to transfer files to a FTP server", "ftp"),
        ("how to display file type and mapping", "ftype"),
        ("how to display MAC address", "getmac"),
        ("how to display IP network settings", "ipconfig"),
        ("how to configure/control/display network components", "netsh"),
        ("how to display TCP/IP connections and status", "netstat"),
        ("how to query the DNS", "nslookup"),
        ("how to test the connection to a specific IP address", "pathping"),
        ("how to pings the network", "ping"),
        ("how to display network routing table, add static routes", "route"),
        ("how to displays computer-specific properties and configurations", "systeminfo"),
        ("how to establish Telnet connection", "telnet"),
        ("how to transfer files to a TFTP server", "tftp"),
        ("how to trace routes similar to patchping", "tracert"),
        ("how to display file attributes", "attib"),
        ("how to compare file contents", "comp"),
        ("how to display/change file compression", "compact"),
        ("how to copy files", "copy / xcopy"),
        ("how to compare content of two floppy disks", "diskcomp"),
        ("how to copy floppy disc to another one", "diskcopy"),
        ("how to delete one or more files", "erase / del"),
        ("how to extract files", "expand"),
        ("how to copare files and display the differences", "fc"),
        ("how to create a new directory", "mkdir"),
        ("how to move/rename files", "move"),
        ("how to rename files", "rename"),
        ("how to replace files", "replace"),
        ("how to delete directory", "rmdir / rd"),
        ("how to display folder structure graphically", "tree"),
        ("how to display content of text files", "type"),
        ("how to check volumes", "chkdsk"),
        ("how to display/change volume check at startup", "ckhntfs"),
        ("how to defragment media", "defrag"),
        ("how to volume management", "dispart"),
        ("how to display installed devices and their properties", "driverquery"),
        ("how to format volumes", "format"),
        ("how to change volume name", "label"),
        ("how to configure interfaces/devices", "mode"),
        ("how to assign/delete drive mountpoints", "mountvol"),
        ("how to monitoring whether volumes are written correctly", "verify"),
        ("how to show volume description and serial numbers of the HDDs", "vol"),
        ("how to for loop", "for"),
        ("how to display group policies", "gpresult"),
        ("how to update group policies", "grupdate"),
        ("how to start performance monitor", "perfmon"),
        ("how to change command prompt", "prompt"),
        ("how to add/read/import/export registry entries", "reg")
    ])
} 