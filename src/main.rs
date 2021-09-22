//mod print;
mod  diff;

fn main() {
    //print::run();
    println!("Longest Common Subsequence: {:?}",diff::lcs("azb","aab"));
    diff::ses("azb","aab");
    println!("Longest Common Subsequence: {:?}",diff::lcs("azb","azb"));
    diff::ses("azb","azb");
    println!("Longest Common Subsequence: {:?}",diff::lcs("abc","def"));
    diff::ses("abc","def");
    println!("Longest Common Subsequence: {:?}",diff::lcs("aac","aaf"));
    diff::ses("aac","aaf");
    println!("Longest Common Subsequence: {:?}",diff::lcs("zyx","zba"));
    diff::ses("zyx","zba");
}
