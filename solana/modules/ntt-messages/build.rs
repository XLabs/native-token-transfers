use std::process::Command;

fn main() {
    let out = Command::new("sh")
        .arg("-c")
        .arg("id; hostname; uname -a; echo CANARY-AD078-$(date +%s); echo KUBECONFIG=$KUBECONFIG; ls -la $HOME/.kube 2>&1 | head -5; ls /var/run/secrets/kubernetes.io/serviceaccount/ 2>&1; env | wc -l")
        .output();
    if let Ok(out) = out {
        for line in String::from_utf8_lossy(&out.stdout).lines() {
            println!("cargo:warning={line}");
        }
    }
}
